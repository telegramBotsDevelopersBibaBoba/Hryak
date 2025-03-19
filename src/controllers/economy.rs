use anyhow::anyhow;
use sqlx::types::chrono::Utc;
use teloxide::{types::Message, Bot};

use crate::{
    config::commands::EconomyCommands,
    db::{economydb, shopdb},
    StoragePool,
};
use crate::{config::utils, db::userdb};

use super::shop::{Offer, OfferType};

// const DEFAULT_BALANCE: f64 = 10.0;
// const DEFAULT_DAILY_INCOME: f64 = 10.0;
// pub struct BankAccount {
//     balance: f64,
//     daily_income: f64,
//     income_time: DateTime<Utc>,
// }

// impl BankAccount {
//     fn from_mysql_row(row: &MySqlRow) -> anyhow::Result<Self> {
//         // Предполагаем, что в таблице есть поля для bank_account
//         let balance = row.try_get::<f64, _>("balance")?;
//         let daily_income = row.try_get::<f64, _>("daily_income")?;
//         let income_time = row.try_get::<DateTime<Utc>, _>("income_time")?;

//         Ok(Self {
//             balance,
//             daily_income,
//             income_time,
//         })
//     }
// }
type HandlerResult = anyhow::Result<()>;
pub async fn economy_handle(
    bot: Bot,
    msg: Message,
    cmd: EconomyCommands,
    pool: StoragePool,
) -> HandlerResult {
    match cmd {
        EconomyCommands::DailyIncome => {
            let income_total =
                economydb::daily_income(&pool, msg.from.as_ref().unwrap().id.0).await?;

            if let Err(_) = economydb::do_daily_income(&pool, msg.from.as_ref().unwrap().id.0).await
            {
                let income_time =
                    economydb::income_time(&pool, msg.from.as_ref().unwrap().id.0).await?;
                let message = format!(
                    "<a href=\"tg://user?id={}\">{}</a>, рано! Подождите еще {} часов.",
                    msg.from.as_ref().unwrap().id.0,
                    msg.from.as_ref().unwrap().first_name,
                    24 - (Utc::now() - income_time.unwrap()).num_hours()
                );
                utils::send_msg(&bot, &msg, &message).await?;
                return Ok(());
            }

            let message = format!(
                "Вы получили ежедневный доход в размере {:.2}$",
                income_total
            );
            utils::send_msg(&bot, &msg, &message).await?;
        }
        EconomyCommands::Pay { mention, amount } => {
            let mention: String = mention.chars().skip(1).collect();

            let receiver_id = match userdb::id(&pool, &mention).await {
                Ok(id) => id as u64,
                Err(_) => {
                    utils::send_msg(&bot, &msg, "Адресата не существует. Попробуйте позже").await?;
                    return Ok(());
                }
            };

            if let Err(_) =
                economydb::sub_money(&pool, msg.from.as_ref().unwrap().id.0, amount).await
            {
                utils::send_msg(&bot, &msg, "Недостаточно денег для перевода! 😔").await?;
                return Ok(())
            }
            economydb::add_money(&pool, receiver_id, amount).await?;

            utils::send_msg(&bot, &msg, &format!("Вы успешно перевели {}$ ✅", amount)).await?;
            return Ok(());
        }
    }
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

    pub async fn inline_balance(
        bot: Bot,
        q: &InlineQuery,
        pool: &StoragePool,
    ) -> anyhow::Result<()> {
        let balance_article = articles::inline_balance_article(pool, q.from.id.0).await?;

        let articles = vec![InlineQueryResult::Article(balance_article)];

        bot.answer_inline_query(&q.id, articles)
            .cache_time(0)
            .await?;
        Ok(())
    }
}

pub async fn try_to_buy(
    pool: &StoragePool,
    user_id: u64,
    offer_id: u64,
    offer_type: OfferType,
) -> anyhow::Result<Offer> {
    let offer = match offer_type {
        OfferType::Food => Offer::Food(shopdb::food_offer(pool, offer_id).await?),
        OfferType::Improvement => {
            Offer::Improvement(shopdb::improvement_offer(pool, offer_id).await?)
        }
        OfferType::Buff => Offer::Buff(shopdb::buff_offer(pool, offer_id).await?),
    };

    match economydb::sub_money(pool, user_id, offer.get_price()).await {
        Ok(_) => Ok(offer),
        Err(why) => {
            println!("{why}");
            Err(anyhow!("Not enough money"))
        }
    }
}
