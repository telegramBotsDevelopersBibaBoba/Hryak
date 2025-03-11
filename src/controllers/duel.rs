use std::{fmt::Display, ops::RangeInclusive, str::FromStr};

use anyhow::anyhow;
use sqlx::{mysql::MySqlRow, MySqlPool, Row};

use crate::{db::economydb, StoragePool};

const ATTACK_RANDOM_FACTOR: RangeInclusive<f64> = 0.8..=1.2;
const DEFENSE_RANDOM_FACTOR: RangeInclusive<f64> = 0.5..=1.0;
const WRONG_ACTION_DOER: &str = "Сейчас не ваш ход";

pub struct Duel {
    pub host_id: i64,
    pub part_id: i64,
    pub host_hp: f64,
    pub part_hp: f64,
    pub bid: f64,
}

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
        let bid: f64 = row.try_get("bid")?;
        Ok(Self {
            host_id,
            part_id,
            host_hp,
            part_hp,
            bid,
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
    use sqlx::MySqlPool;
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
    use std::{str::FromStr, time::Duration};

    use anyhow::anyhow;
    use rand::Rng;
    use sqlx::MySqlPool;
    use teloxide::{
        payloads::{AnswerCallbackQuerySetters, EditMessageTextInlineSetters},
        prelude::{Request, Requester},
        types::CallbackQuery,
        Bot,
    };

    use crate::{
        config::consts,
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
                .text("Нельзя дуэлить себя идиот гадэмн бля")
                .send()
                .await?;
            return Ok(());
        }

        let bid = data[2]
            .trim()
            .parse::<f64>()
            .unwrap_or(consts::DUEL_DEFAULT_BID);
        let part_balance = economydb::balance(pool, part_id).await?;
        if part_balance < bid {
            bot.answer_callback_query(&q.id)
                .text("Недостаточно денег!")
                .send()
                .await?;
            return Ok(());
        }
        // Withdraw bids so it works good when users are in several duels at once
        economydb::sub_money(pool, host_id, bid).await?;
        economydb::sub_money(pool, part_id, bid).await?;
        // Creeate a duel in table
        let host_pig = pigdb::pig_by_userid(pool, host_id).await?;
        let part_pig = pigdb::pig_by_userid(pool, part_id).await?;
        dueldb::create_duel(
            pool,
            host_id,
            part_id,
            host_pig.weight,
            part_pig.weight,
            bid,
        )
        .await?;

        // Setup message, add reply markup

        let msg = format!(
            "Очередь: хоста\nЗдоровье хоста: {} хп\nЗдоровье участника: {} hp",
            host_pig.weight, part_pig.weight
        );
        bot.edit_message_text_inline(q.inline_message_id.as_ref().unwrap(), msg)
            .reply_markup(keyboard::make_duel_action(host_id, Duelist::Host))
            .await?;

        Ok(())
    }

    pub async fn callback_duel_action(
        bot: &Bot,
        q: &CallbackQuery,
        data: &[&str],
        pool: &StoragePool,
    ) -> anyhow::Result<()> {
        /*
        TODO 1: Better algo for defense (maybe)
        TODO 2: fix all kinds of errors (user playing several duels at the same time)


        */

        // Parse all callback data
        let host_id = data[0].trim().parse::<u64>()?;
        let action_type = DuelActionType::from_str(data[1])?;
        let duelist = Duelist::from_str(data[2])?;

        // Track health and etc
        let mut duel: Duel = dueldb::duel(pool, host_id).await?;

        // Get info about participants pigs
        let host_pig = pigdb::pig_by_userid(pool, host_id).await?;
        let part_pig = pigdb::pig_by_userid(pool, q.from.id.0).await?;

        match duelist {
            // Act based on who it is
            Duelist::Host => {
                if q.from.id.0 != host_id {
                    bot.answer_callback_query(&q.id)
                        .text(WRONG_ACTION_DOER)
                        .send()
                        .await?;
                    return Ok(());
                }

                match action_type {
                    DuelActionType::Attack => {
                        duel.part_hp -=
                            host_pig.attack * rand::rng().random_range(ATTACK_RANDOM_FACTOR);

                        if duel.part_hp <= 0.0 {
                            // Host won
                            let username = userdb::username(pool, host_id).await?;
                            let msg = format!("@{} выиграл {}$", username, 2.0 * duel.bid);
                            bot.edit_message_text_inline(
                                q.inline_message_id.as_ref().unwrap(),
                                msg,
                            )
                            .await?;

                            proccess_duel_results(pool, host_id, duel.part_id as u64, duel.bid)
                                .await?;

                            dueldb::remove_duel(pool, host_id).await?; // Remove duel from database so the user can start a new one
                                                                       // (P.S duels are auto deleted every 10 minutes (if created_at date is older than 10 mins))
                            return Ok(());
                        }
                    }
                    DuelActionType::Defense => {
                        // TODO i think, better algo for defense
                        duel.host_hp +=
                            host_pig.defense * rand::rng().random_range(DEFENSE_RANDOM_FACTOR);
                        if duel.host_hp > host_pig.weight {
                            duel.host_hp = host_pig.weight;
                        }
                    }
                }
                let msg = format!(
                    "Очередь участника\nЗдоровье хоста: {} хп\nЗдоровье участника: {} хп",
                    (duel.host_hp * 100.0).floor() / 100.0,
                    (duel.part_hp * 100.0).floor() / 100.0
                );
                dueldb::update_duel(pool, duel).await?;

                bot.edit_message_text_inline(q.inline_message_id.as_ref().unwrap(), msg)
                    .reply_markup(keyboard::make_duel_action(host_id, Duelist::Part))
                    .await?;
            }
            Duelist::Part => {
                if q.from.id.0 != duel.part_id as u64 {
                    bot.answer_callback_query(&q.id)
                        .text(WRONG_ACTION_DOER)
                        .send()
                        .await?;
                    return Ok(());
                }
                match action_type {
                    DuelActionType::Attack => {
                        duel.host_hp -=
                            part_pig.attack * rand::rng().random_range(ATTACK_RANDOM_FACTOR);

                        if duel.host_hp <= 0.0 {
                            // Part won
                            let username = userdb::username(pool, duel.part_id as u64).await?;
                            let msg = format!("@{} выиграл {}$", username, 2.0 * duel.bid);
                            bot.edit_message_text_inline(
                                q.inline_message_id.as_ref().unwrap(),
                                msg,
                            )
                            .await?;

                            proccess_duel_results(pool, duel.part_id as u64, host_id, duel.bid)
                                .await?;

                            dueldb::remove_duel(pool, host_id).await?;
                            return Ok(());
                        }
                    }
                    DuelActionType::Defense => {
                        duel.part_hp +=
                            part_pig.defense * rand::rng().random_range(DEFENSE_RANDOM_FACTOR);
                        if duel.part_hp > part_pig.weight {
                            duel.part_hp = part_pig.weight;
                        }
                    }
                }
                let msg = format!(
                    "Очередь хоста\nЗдоровье хоста: {} хп\nЗдоровье участника: {} хп",
                    (duel.host_hp * 100.0).floor() / 100.0,
                    (duel.part_hp * 100.0).floor() / 100.0
                );
                dueldb::update_duel(pool, duel).await?;

                bot.edit_message_text_inline(q.inline_message_id.as_ref().unwrap(), msg)
                    .reply_markup(keyboard::make_duel_action(host_id, Duelist::Host))
                    .await?;
            }
        }

        Ok(())
    }
}
