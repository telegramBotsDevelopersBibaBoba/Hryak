use sqlx::{
    mysql::MySqlRow,
    types::chrono::{DateTime, Utc},
    MySqlPool, Row,
};
use teloxide::{prelude::ResponseResult, types::Message, Bot};

use crate::{config::utils, db::userdb};
use crate::{
    config::{commands::EconomyCommands, consts},
    db::economydb,
};

pub struct BankAccount {
    balance: f64,
    daily_income: f64,
    income_time: DateTime<Utc>,
}

impl BankAccount {
    fn from_mysql_row(row: &MySqlRow) -> anyhow::Result<Self> {
        // Предполагаем, что в таблице есть поля для bank_account
        let balance = row.try_get::<f64, _>("balance")?;
        let daily_income = row.try_get::<f64, _>("daily_income")?;
        let income_time = row.try_get::<DateTime<Utc>, _>("income_time")?;

        Ok(Self {
            balance,
            daily_income,
            income_time,
        })
    }
}
type HandlerResult = anyhow::Result<()>;
pub async fn economy_handle(
    bot: Bot,
    msg: Message,
    cmd: EconomyCommands,
    pool: MySqlPool,
) -> HandlerResult {
    match cmd {
        EconomyCommands::DailyIncome => {
            let (income_total, last_income) =
                economydb::get_daily_income(&pool, msg.from.as_ref().unwrap().id.0).await?;

            if let Err(why) =
                economydb::do_daily_income(&pool, msg.from.as_ref().unwrap().id.0).await
            {
                let message = format!(
                    "<a href=\"tg://user?id={}\">{}</a>, рано! Подождите еще {} часов.",
                    msg.from.as_ref().unwrap().id.0,
                    msg.from.as_ref().unwrap().first_name,
                    24 - (Utc::now() - last_income.unwrap()).num_hours()
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
            println!("Args: {} {}", mention, amount);
            let balance_host =
                economydb::get_balance(&pool, msg.from.as_ref().unwrap().id.0).await?;
            if balance_host < amount {
                todo!("Send error message not enogu money");
            }
            println!("here");
            let receiver_id = match userdb::id_by_username(&pool, &mention).await {
                Ok(id) => id as u64,
                Err(_) => {
                    utils::send_msg(&bot, &msg, "Адресата не существует. Попробуйте позже").await?;
                    return Ok(());
                }
            };

            economydb::sub_money(&pool, msg.from.as_ref().unwrap().id.0, amount).await?;
            economydb::add_money(&pool, receiver_id, amount).await?;

            utils::send_msg(&bot, &msg, &format!("Вы успешно перевели {}$", amount)).await?;
            return Ok(());
        }
    }
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

    use crate::handlers::articles;

    pub async fn inline_balance(bot: Bot, q: &InlineQuery, pool: &MySqlPool) -> anyhow::Result<()> {
        let balance_article = articles::inline_balance_article(pool, q.from.id.0).await?;

        let articles = vec![InlineQueryResult::Article(balance_article)];

        bot.answer_inline_query(&q.id, articles)
            .cache_time(0)
            .await?;
        Ok(())
    }
}
