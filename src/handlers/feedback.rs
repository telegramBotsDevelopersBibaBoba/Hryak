use std::str::FromStr;

use futures::FutureExt;

use teloxide::{types::ChosenInlineResult, Bot};

use crate::{config::commands::FeedbackCommands, controllers, deser_command, StoragePool};
type HandlerResult = anyhow::Result<()>;
pub async fn filter_inline_chosen_command(
    // Called when you click on a query
    _: Bot,
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
                controllers::pig::feedback::feedback_rename_hryak(&q, &args[1..], &pool).boxed()
                // args are <new_name>
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
