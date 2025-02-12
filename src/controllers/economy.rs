use sqlx::{
    mysql::MySqlRow,
    types::chrono::{DateTime, Utc},
    MySqlPool, Row,
};
use teloxide::{prelude::ResponseResult, types::Message, Bot};

use crate::{config::commands::EconomyCommands, db::economydb};
use crate::{config::utils, db::userdb};

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

pub async fn economy_handle(
    bot: Bot,
    msg: Message,
    cmd: EconomyCommands,
    pool: MySqlPool,
) -> ResponseResult<()> {
    match cmd {
        EconomyCommands::DailyIncome => {
            let (daily_multiplier, last_income) =
                economydb::get_daily_income(&pool, msg.from.as_ref().unwrap().id.0)
                    .await
                    .unwrap();
            let income_total = daily_multiplier * economydb::BASE_INCOME;

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
            let balance_host = economydb::get_balance(&pool, msg.from.as_ref().unwrap().id.0)
                .await
                .unwrap();
            if balance_host < amount {
                todo!("Send error message not enogu money");
            }

            let receiver_id = match userdb::id_by_username(&pool, &mention).await {
                Ok(id) => id as u64,
                Err(_) => {
                    utils::send_msg(&bot, &msg, "Адресата не существует. Попробуйте позже")
                        .await
                        .unwrap();
                    return Ok(());
                }
            };
            let balance_receiver = economydb::get_balance(&pool, receiver_id).await.unwrap();

            economydb::sub_money(&pool, msg.from.as_ref().unwrap().id.0, amount)
                .await
                .unwrap(); // Shall not fail cuz of the check before
            economydb::add_money(&pool, receiver_id, amount)
                .await
                .unwrap(); // Shall not fail because why would it fail

            utils::send_msg(&bot, &msg, &format!("Вы успешно перевели {}$", amount)).await?;
            return Ok(());
        }
    }
    Ok(())
}
