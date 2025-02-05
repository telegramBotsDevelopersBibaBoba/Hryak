use std::str::FromStr;

use futures::FutureExt;
use sqlx::MySqlPool;
use teloxide::{types::ChosenInlineResult, Bot, RequestError};

use crate::{config::commands::FeedbackCommands, db::pigdb};

pub async fn filter_inline_chosen_command(
    bot: Bot,
    q: ChosenInlineResult,
    pool: MySqlPool,
) -> Result<(), RequestError> {
    let mut params = q
        .query
        .clone()
        .split(" ")
        .map(|el| el.to_string())
        .collect::<Vec<String>>();

    let function = match FeedbackCommands::from_str(&params[0]) {
        Ok(com) => match com {
            FeedbackCommands::ChangeName => {
                feedback_rename_hryak(bot, &q, &params[1..], &pool).boxed()
            }
        },
        Err(why) => return Ok(()),
    };

    let resp = function.await;
    if let Err(why) = resp {
        println!("{}", why);
    }
    Ok(())
}

pub async fn feedback_rename_hryak(
    bot: Bot,
    q: &ChosenInlineResult,
    args: &[String],
    pool: &MySqlPool,
) -> anyhow::Result<()> {
    println!("here");
    if args.is_empty() {
        println!("Emtpy");
        todo!("Error");
    }
    println!("Name {:?}", args[0]);
    pigdb::set_pig_name(pool, &args[0], q.from.id.0).await?;
    Ok(())
}
