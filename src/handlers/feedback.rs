use std::str::FromStr;

use anyhow::anyhow;
use futures::FutureExt;
use sqlx::MySqlPool;
use teloxide::{
    dispatching::dialogue::InMemStorage, prelude::Dialogue, types::ChosenInlineResult, Bot,
    RequestError,
};

use crate::{
    config::commands::FeedbackCommands, controllers::gambling::GuessState, db::pigdb, deser_command,
};
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
pub async fn filter_inline_chosen_command(
    // Called when you click on a query
    bot: Bot,
    q: ChosenInlineResult,
    pool: MySqlPool,
) -> HandlerResult {
    println!("sdfkjsdklfsdjklsdfkjl;ds");
    if q.query.is_empty() {
        return Ok(());
    }
    let args = deser_command!(q.query);

    let function = match FeedbackCommands::from_str(args[0]) {
        Ok(com) => match com {
            FeedbackCommands::ChangeName => {
                feedback_rename_hryak(bot, &q, &args[1..], &pool).boxed() // args are <new_name>
            }
        },
        Err(_) => return Ok(()), // If it's not any command it's just better to skip it (return Ok) since it may have not been intended to come here
    };

    let resp = function.await;
    if let Err(why) = resp {
        println!("{}", why);
    }
    Ok(())
}

async fn feedback_rename_hryak(
    bot: Bot,
    q: &ChosenInlineResult,
    args: &[&str],
    pool: &MySqlPool,
) -> anyhow::Result<()> {
    if args.is_empty() {
        return Err(anyhow!("Rename hryak args are emtpy"));
    }
    pigdb::set_pig_name(pool, &args[0], q.from.id.0).await?;
    Ok(())
}

async fn feedback_guess_game(
    bot: Bot,
    q: &ChosenInlineResult,
    dialogue: Dialogue<GuessState, InMemStorage<GuessState>>,
) -> anyhow::Result<()> {
    Ok(())
}
