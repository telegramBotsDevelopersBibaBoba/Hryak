use std::str::FromStr;

use crate::{
    controllers::{self},
    StoragePool,
};

use futures::FutureExt;

use teloxide::{
    payloads::AnswerCallbackQuerySetters, prelude::Requester, types::CallbackQuery, Bot,
};

use crate::{config::commands::CallbackCommands, deser_command};
type HandlerResult = anyhow::Result<()>;
pub async fn filter_callback_commands(
    bot: Bot,
    q: CallbackQuery,
    pool: StoragePool,
) -> HandlerResult {
    // Called usually when you click on a button
    if q.data.is_none() {
        callback_error(&bot, &q).await.unwrap();
    }

    let data_vec = deser_command!(q.data.as_ref().unwrap());

    // Parsing query and figuring out a command based on it
    let function = match CallbackCommands::from_str(data_vec[0]) {
        Ok(command) => match command {
            CallbackCommands::Shop => {
                controllers::shop::callback::callback_shop(&bot, &q, &data_vec[1..], &pool).boxed()
            } // args are <type> <id>
            CallbackCommands::DuelStart => {
                controllers::duel::callback::callbak_start_duel(
                    &bot,
                    &q,
                    &data_vec[1..],
                    q.from.id.0,
                    &pool,
                )
                .boxed()
                // Args are <host-id> <host-mention> <bid>
            }
            CallbackCommands::DuelAction => {
                controllers::duel::callback::callback_duel_action(&bot, &q, &data_vec[1..], &pool)
                    .boxed()
            }
            CallbackCommands::BuffUse => {
                controllers::duel::callback::callback_use_buff(&bot, &q, &data_vec[1..], &pool)
                    .boxed()
            }
            CallbackCommands::SwitchPageDuel => {
                controllers::duel::callback::callback_switch_page(&bot, &q, &data_vec[1..], &pool)
                    .boxed()
            }
        },
        Err(_) => callback_error(&bot, &q).boxed(),
    };
    let resp = function.await;

    // Checking for errors
    if let Err(why) = resp {
        println!("{}", why);
    }
    Ok(())
}

async fn callback_error(bot: &Bot, q: &CallbackQuery) -> anyhow::Result<()> {
    bot.answer_callback_query(&q.id)
        .text("Ошибка, неизвестный коллбэк")
        .await?;
    Ok(())
}
