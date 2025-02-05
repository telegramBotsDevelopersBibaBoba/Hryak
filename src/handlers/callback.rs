use std::str::FromStr;

use futures::FutureExt;
use teloxide::{
    payloads::AnswerCallbackQuerySetters, prelude::Requester, types::CallbackQuery, Bot,
    RequestError,
};

use crate::config::commands::CallbackCommands;

pub async fn filter_callback_commands(bot: Bot, q: CallbackQuery) -> Result<(), RequestError> {
    let data_vec = q
        .data
        .clone()
        .unwrap_or("nothing".to_owned())
        .split(" ")
        .map(|element| element.to_string())
        .collect::<Vec<String>>();

    let function = match CallbackCommands::from_str(&data_vec[0]) {
        Ok(command) => match command {
            CallbackCommands::Shop => test_shop_callback(&bot, &q, &data_vec[1..]).boxed(),
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

pub async fn test_shop_callback(
    bot: &Bot,
    q: &CallbackQuery,
    data: &[String],
) -> anyhow::Result<()> {
    bot.answer_callback_query(&q.id)
        .text(format!("Вы выбрали {}", data[0]))
        .await?;
    bot.edit_message_text_inline(
        q.inline_message_id.as_ref().unwrap(),
        "Покупка успешно совершена",
    )
    .await?;
    Ok(())
}
