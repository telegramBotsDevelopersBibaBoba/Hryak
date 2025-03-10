use std::str::FromStr;

use anyhow::anyhow;
use futures::FutureExt;

use teloxide::{
    dispatching::dialogue::InMemStorage, prelude::Dialogue, types::ChosenInlineResult, Bot,
    RequestError,
};

use crate::{
    config::commands::FeedbackCommands, controllers, db::pigdb, deser_command, StoragePool,
};
type HandlerResult = anyhow::Result<()>;
pub async fn filter_inline_chosen_command(
    // Called when you click on a query
    bot: Bot,
    q: ChosenInlineResult,
    pool: StoragePool,
) -> HandlerResult {
    if q.query.is_empty() {
        return Ok(());
    }
    let args = deser_command!(q.query);

    let function = match FeedbackCommands::from_str(args[0]) {
        Ok(com) => match com {
            FeedbackCommands::ChangeName => {
                controllers::pig::feedback::feedback_rename_hryak(bot, &q, &args[1..], &pool)
                    .boxed() // args are <new_name>
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
