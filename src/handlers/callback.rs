use std::str::FromStr;

use futures::FutureExt;
use teloxide::{
    payloads::{AnswerCallbackQuerySetters, EditMessageTextInlineSetters},
    prelude::{Request, Requester},
    types::CallbackQuery,
    Bot, RequestError,
};

use crate::config::commands::CallbackCommands;

use super::keyboard::make_shop;

pub async fn filter_callback_commands(bot: Bot, q: CallbackQuery) -> Result<(), RequestError> {
    let data_vec = q
        .data
        .clone()
        .unwrap_or("nothing".to_owned())
        .split(" ")
        .map(|element| element.to_string())
        .collect::<Vec<String>>();

    // Parsing query and figuring out a command based on it
    let function = match CallbackCommands::from_str(&data_vec[0]) {
        Ok(command) => match command {
            CallbackCommands::Shop => inline_shop_callback(&bot, &q, &data_vec[1..]).boxed(), // args are <type> <name>
        },
        Err(why) => callback_error(&bot, &q).boxed(),
    };
    let resp = function.await;

    // Checking for errors
    if let Err(why) = resp {
        println!("{}", why);
    }
    Ok(())
}

pub async fn callback_error(bot: &Bot, q: &CallbackQuery) -> anyhow::Result<()> {
    bot.answer_callback_query(&q.id)
        .text("Ошибка, неизвестный коллбэк")
        .await?;
    Ok(())
}

pub async fn inline_shop_callback(
    bot: &Bot,
    q: &CallbackQuery,
    data: &[String],
) -> anyhow::Result<()> {
    bot.answer_callback_query(&q.id)
        .text(format!("Покупка была успешно совершена!"))
        .await?;

    // bot.edit_message_text_inline(q.inline_message_id.as_ref().unwrap(), "cock")
    //     .text("fuckme")
    //     .reply_markup(make_shop())
    //     .await?;

    Ok(())
}
