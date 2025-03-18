use std::{fmt::Display, ops::RangeInclusive, str::FromStr};

use anyhow::anyhow;
use sqlx::{mysql::MySqlRow, Row};

use crate::{db::economydb, StoragePool};

const ATTACK_RANDOM_FACTOR: RangeInclusive<f64> = 0.8..=1.2;
const DEFENSE_RANDOM_FACTOR: RangeInclusive<f64> = 0.5..=1.0;
const WRONG_ACTION_DOER: &str = "Сейчас не ваш ход";

pub struct Duel {
    pub host_id: i64,
    pub part_id: i64,
    pub host_hp: f64,
    pub part_hp: f64,
    pub host_attack: f64,
    pub host_defense: f64,
    pub part_attack: f64,
    pub part_defense: f64,
    pub bid: f64,
}

#[derive(PartialEq)]
pub enum Duelist {
    Host,
    Part,
}

impl Display for Duelist {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Host => write!(f, "host"),
            Self::Part => write!(f, "part"),
        }
    }
}
impl FromStr for Duelist {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "host" => Ok(Self::Host),
            "part" => Ok(Self::Part),
            _ => Err(anyhow!("Unknown type")),
        }
    }
}

pub enum DuelActionType {
    Attack,
    Defense,
}

impl Display for DuelActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Attack => write!(f, "attack"),
            Self::Defense => write!(f, "defense"),
        }
    }
}
impl FromStr for DuelActionType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "attack" => Ok(Self::Attack),
            "defense" => Ok(Self::Defense),
            _ => Err(anyhow!("Unknown type")),
        }
    }
}

impl Duel {
    pub fn from_mysql_row(row: &MySqlRow) -> anyhow::Result<Self> {
        let host_id: i64 = row.try_get("host_id")?;
        let part_id: i64 = row.try_get("part_id")?;
        let host_hp: f64 = row.try_get("host_hp")?;
        let part_hp: f64 = row.try_get("part_hp")?;
        let host_attack: f64 = row.try_get("host_attack")?;
        let host_defense: f64 = row.try_get("host_defense")?;
        let part_attack: f64 = row.try_get("part_attack")?;
        let part_defense: f64 = row.try_get("part_defense")?;

        let bid: f64 = row.try_get("bid")?;
        Ok(Self {
            host_id,
            part_id,
            host_hp,
            part_hp,
            bid,
            host_attack,
            host_defense,
            part_attack,
            part_defense,
        })
    }
}

pub async fn proccess_duel_results(
    pool: &StoragePool,
    winner_id: u64,
    loser_id: u64,
    bid: f64,
) -> anyhow::Result<()> {
    economydb::add_money(pool, winner_id, 2.0 * bid).await?;
    economydb::sub_money(pool, loser_id, bid).await?;
    Ok(())
}

pub mod inline {
    use teloxide::{
        payloads::AnswerInlineQuerySetters,
        prelude::Requester,
        types::{InlineQuery, InlineQueryResult},
        Bot,
    };

    use crate::{handlers::articles, StoragePool};

    pub async fn inline_duel(
        bot: Bot,
        q: &InlineQuery,
        pool: &StoragePool,
        bid: f64,
    ) -> anyhow::Result<()> {
        let duel =
            articles::inline_duel_article(&pool, q.from.id.0, q.from.mention().unwrap(), bid)
                .await?;
        let articles = vec![InlineQueryResult::Article(duel)];

        bot.answer_inline_query(&q.id, articles)
            .cache_time(0)
            .await?;
        Ok(())
    }
}

pub mod callback {
    use std::str::FromStr;

    use anyhow::anyhow;
    use rand::Rng;
    use teloxide::{
        payloads::{
            AnswerCallbackQuerySetters, EditMessageReplyMarkupInlineSetters,
            EditMessageTextInlineSetters,
        },
        prelude::{Request, Requester},
        types::CallbackQuery,
        Bot,
    };

    use crate::{
        config::consts,
        controllers::{inventory, pig::Pig},
        db::{dueldb, economydb, pigdb, userdb},
        handlers::keyboard,
        StoragePool,
    };

    use super::{
        proccess_duel_results, Duel, DuelActionType, Duelist, ATTACK_RANDOM_FACTOR,
        DEFENSE_RANDOM_FACTOR, WRONG_ACTION_DOER,
    };

    pub async fn callbak_start_duel(
        bot: &Bot,
        q: &CallbackQuery,
        data: &[&str],
        part_id: u64,
        pool: &StoragePool,
    ) -> anyhow::Result<()> {
        if data.is_empty() {
            bot.edit_message_text_inline(
                q.inline_message_id
                    .as_ref()
                    .ok_or(anyhow!("No data in start duel"))?,
                "Ошибка при дуэли. Отмена",
            )
            .send()
            .await?;
            return Ok(());
        }

        let host_id = data[0].trim().parse::<u64>()?;

        if host_id == part_id {
            bot.answer_callback_query(&q.id)
                .text("Нельзя дуэлить себя")
                .send()
                .await?;
            return Ok(());
        }

        let bid = data[2]
            .trim()
            .parse::<f64>()
            .unwrap_or(consts::DUEL_DEFAULT_BID);

        // Withdraw bids so it works good when users are in several duels at once
        economydb::sub_money(pool, host_id, bid).await?;
        if let Err(why) = economydb::sub_money(pool, part_id, bid).await {
            eprintln!("Error sub money from part: {}", why);
            bot.answer_callback_query(&q.id).await?;
            return Ok(());
        }

        // Creeate a duel in table
        let host_pig = pigdb::pig_by_userid(pool, host_id).await?;
        let part_pig = pigdb::pig_by_userid(pool, part_id).await?;

        dueldb::create_duel(
            pool,
            host_id,
            part_id,
            host_pig.weight,
            part_pig.weight,
            host_pig.attack,
            host_pig.defense,
            part_pig.attack,
            part_pig.defense,
            bid,
        )
        .await?;

        bot.edit_message_text_inline(
            q.inline_message_id.as_ref().unwrap(),
            format!(
                "🎭 Очередь: Хост\n❤️ ХП Хоста: {:.2}\n❤️ ХП Участника: {:.2}",
                host_pig.weight, part_pig.weight
            ),
        )
        .reply_markup(
            keyboard::make_duel_action(pool, host_id, part_id.clone(), Duelist::Host, 0).await,
        )
        .await?;

        Ok(())
    }

    pub async fn callback_duel_action(
        bot: &Bot,
        q: &CallbackQuery,
        data: &[&str],
        pool: &StoragePool,
    ) -> anyhow::Result<()> {
        // Парсинг данных из callback
        let host_id = data[0].trim().parse::<u64>()?;
        let action_type = DuelActionType::from_str(data[1])?;
        let duelist = Duelist::from_str(data[2])?;

        let mut duel = dueldb::duel(pool, host_id).await?;
        let host_pig = pigdb::pig_by_userid(pool, host_id).await?;
        let part_pig = pigdb::pig_by_userid(pool, q.from.id.0).await?;

        match duelist {
            Duelist::Host => {
                if q.from.id.0 != host_id {
                    return wrong_action(bot, q).await;
                }
                handle_host_action(bot, q, pool, host_id, &mut duel, &host_pig, action_type)
                    .await?;
            }
            Duelist::Part => {
                if q.from.id.0 != duel.part_id as u64 {
                    return wrong_action(bot, q).await;
                }
                handle_part_action(bot, q, pool, host_id, &mut duel, &part_pig, action_type)
                    .await?;
            }
        }

        Ok(())
    }

    async fn handle_host_action(
        bot: &Bot,
        q: &CallbackQuery,
        pool: &StoragePool,
        host_id: u64,
        duel: &mut Duel,
        host_pig: &Pig,
        action_type: DuelActionType,
    ) -> anyhow::Result<()> {
        match action_type {
            DuelActionType::Attack => {
                duel.part_hp -= duel.host_attack * rand::rng().random_range(ATTACK_RANDOM_FACTOR);
                if duel.part_hp <= 0.0 {
                    return process_victory(
                        bot,
                        q,
                        pool,
                        host_id,
                        duel.part_id as u64,
                        duel.bid,
                        host_id,
                    )
                    .await;
                }
            }
            DuelActionType::Defense => {
                duel.host_hp += duel.host_defense * rand::rng().random_range(DEFENSE_RANDOM_FACTOR);
                if duel.host_hp > host_pig.weight {
                    duel.host_hp = host_pig.weight;
                }
            }
        }

        let msg = format!(
            "🔄 Очередь участника\n❤️ ХП Хоста: {:.2}\n❤️ ХП Участника: {:.2}",
            duel.host_hp, duel.part_hp
        );
        dueldb::update_duel(pool, duel).await?;
        let keyboard =
            keyboard::make_duel_action(pool, host_id, duel.part_id as u64, Duelist::Part, 0).await;
        bot.edit_message_text_inline(q.inline_message_id.as_ref().unwrap(), msg)
            .reply_markup(keyboard)
            .await?;

        Ok(())
    }

    async fn handle_part_action(
        bot: &Bot,
        q: &CallbackQuery,
        pool: &StoragePool,
        host_id: u64,
        duel: &mut Duel,
        part_pig: &Pig,
        action_type: DuelActionType,
    ) -> anyhow::Result<()> {
        match action_type {
            DuelActionType::Attack => {
                duel.host_hp -= duel.part_attack * rand::rng().random_range(ATTACK_RANDOM_FACTOR);
                if duel.host_hp <= 0.0 {
                    return process_victory(
                        bot,
                        q,
                        pool,
                        duel.part_id as u64,
                        host_id,
                        duel.bid,
                        host_id,
                    )
                    .await;
                }
            }
            DuelActionType::Defense => {
                duel.part_hp += duel.part_defense * rand::rng().random_range(DEFENSE_RANDOM_FACTOR);
                if duel.part_hp > part_pig.weight {
                    duel.part_hp = part_pig.weight;
                }
            }
        }

        let msg = format!(
            "🔄 Очередь хоста\n❤️ ХП Хоста: {:.2}\n❤️ ХП Участника: {:.2}",
            duel.host_hp, duel.part_hp
        );
        dueldb::update_duel(pool, duel).await?;
        let keyboard =
            keyboard::make_duel_action(pool, host_id, duel.part_id as u64, Duelist::Host, 0).await;
        bot.edit_message_text_inline(q.inline_message_id.as_ref().unwrap(), msg)
            .reply_markup(keyboard)
            .await?;

        Ok(())
    }

    async fn process_victory(
        bot: &Bot,
        q: &CallbackQuery,
        pool: &StoragePool,
        winner_id: u64,
        loser_id: u64,
        bid: f64,
        host_id: u64,
    ) -> anyhow::Result<()> {
        let username = userdb::username(pool, winner_id).await?;
        let msg = format!(
            "🎉 Победитель: @{} 🏆\n💰 Выигрыш: {}$",
            username,
            2.0 * bid
        );

        bot.edit_message_text_inline(q.inline_message_id.as_ref().unwrap(), msg)
            .await?;
        proccess_duel_results(pool, winner_id, loser_id, bid).await?;
        dueldb::remove_duel(pool, host_id).await?;
        Ok(())
    }

    async fn wrong_action(bot: &Bot, q: &CallbackQuery) -> anyhow::Result<()> {
        bot.answer_callback_query(&q.id)
            .text(WRONG_ACTION_DOER)
            .send()
            .await?;
        Ok(())
    }

    pub async fn callback_use_buff(
        bot: &Bot,
        q: &CallbackQuery,
        data: &[&str], // &host_id.to_string(), &user_id.to_string(), &invslot_id.to_string())
        pool: &StoragePool,
    ) -> anyhow::Result<()> {
        let host_id = data[0].parse::<u64>()?;
        let user_id = data[1].parse::<u64>()?;
        let invslot_id = data[2].parse::<u64>()?;

        if q.from.id.0 != user_id {
            bot.answer_callback_query(&q.id)
                .text("Не ваша очередь")
                .send()
                .await?;
            return Ok(());
        }

        let mut duel: Duel = dueldb::duel(pool, host_id).await?;

        match inventory::use_item(pool, invslot_id).await {
            Ok((buff_type, power)) => {
                if buff_type == "attack" {
                    if q.from.id.0 == host_id {
                        duel.host_attack += power;
                    } else {
                        duel.part_attack += power;
                    }
                } else {
                    if q.from.id.0 == host_id {
                        duel.host_defense += power;
                    } else {
                        duel.part_defense += power;
                    }
                }
                dueldb::update_duel(pool, &duel).await?;

                bot.answer_callback_query(&q.id)
                    .text("Успешно использовано")
                    .send()
                    .await?;
            }
            Err(why) => {
                eprintln!("Err using item: {}", why);
                bot.answer_callback_query(&q.id)
                    .text("Предмет закончился. Ошибка.")
                    .send()
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn callback_switch_page(
        bot: &Bot,
        q: &CallbackQuery,
        data: &[&str], // &host_id.to_string(), &part_id.to_string(), &duelist.to_string(), offset
        pool: &StoragePool,
    ) -> anyhow::Result<()> {
        let host_id = data[0].parse::<u64>()?;
        let part_id = data[1].parse::<u64>()?;
        let duelist = Duelist::from_str(data[2])?;
        let offset = data[3].parse::<u32>()?;

        if (duelist == Duelist::Host && q.from.id.0 != host_id)
            || (duelist == Duelist::Part && q.from.id.0 != part_id)
        {
            bot.answer_callback_query(&q.id)
                .text("Не ваша очередь")
                .send()
                .await?;
            return Ok(());
        }

        bot.edit_message_reply_markup_inline(q.inline_message_id.as_ref().unwrap())
            .reply_markup(keyboard::make_duel_action(pool, host_id, part_id, duelist, offset).await)
            .send()
            .await?;
        Ok(())
    }
}
