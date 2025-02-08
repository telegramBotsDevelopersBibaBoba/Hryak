use std::str::FromStr;

use futures::FutureExt;
use sqlx::MySqlPool;
use teloxide::{types::ChosenInlineResult, Bot, RequestError};

use crate::{config::commands::FeedbackCommands, db::pigdb, deser_command};

pub async fn filter_inline_chosen_command(
    // Called when you click on a query
    bot: Bot,
    q: ChosenInlineResult,
    pool: MySqlPool,
) -> Result<(), RequestError> {
    if q.query.is_empty() {
        return Ok(());
    }
    let args = deser_command!(q.query);

    let function = match FeedbackCommands::from_str(args[0]) {
        Ok(com) => match com {
            FeedbackCommands::ChangeName => {
                feedback_rename_hryak(bot, &q, &args[1..], &pool).boxed()
            }
        },
        Err(why) => return Ok(()), // If it's not any command it's just better to skip it (return Ok) since it may have not been intended to come here
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
    args: &[&str],
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
