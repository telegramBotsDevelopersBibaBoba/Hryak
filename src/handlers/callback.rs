use std::{str::FromStr, time::Duration};

use futures::FutureExt;
use rand::Rng;
use sqlx::MySqlPool;
use teloxide::{
    payloads::{AnswerCallbackQuerySetters, EditMessageTextInlineSetters},
    prelude::{Request, Requester},
    types::CallbackQuery,
    Bot, RequestError,
};
use tokio::time::sleep;

use crate::{config::commands::CallbackCommands, db::pigdb};

use super::keyboard::make_shop;

pub async fn filter_callback_commands(
    bot: Bot,
    q: CallbackQuery,
    pool: MySqlPool,
) -> Result<(), RequestError> {
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
            CallbackCommands::Shop => callback_shop(&bot, &q, &data_vec[1..]).boxed(), // args are <type> <name>
            CallbackCommands::StartDuel => {
                callbak_start_duel(&bot, &q, &data_vec[1..], q.from.id.0, &pool).boxed()
            }
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

pub async fn callback_shop(bot: &Bot, q: &CallbackQuery, data: &[String]) -> anyhow::Result<()> {
    bot.answer_callback_query(&q.id)
        .text(format!("Покупка была успешно совершена!"))
        .await?;

    // bot.edit_message_text_inline(q.inline_message_id.as_ref().unwrap(), "cock")
    //     .text("fuckme")
    //     .reply_markup(make_shop())
    //     .await?;

    Ok(())
}

pub async fn callbak_start_duel(
    bot: &Bot,
    q: &CallbackQuery,
    data: &[String],
    part_id: u64,
    pool: &MySqlPool,
) -> anyhow::Result<()> {
    if data.is_empty() {
        bot.edit_message_text_inline(
            q.inline_message_id.as_ref().unwrap(),
            "Ошибка при дуэли. Отмена",
        )
        .send()
        .await?;
        return Ok(());
    }

    let host_id = data[0].trim().parse::<u64>().unwrap();
    println!("{} {}", part_id, host_id);
    if !pigdb::pig_exists(pool, host_id).await || !pigdb::pig_exists(pool, part_id).await {
        bot.edit_message_text_inline(
            q.inline_message_id.as_ref().unwrap(),
            "Ошибка при дуэли. Отмена. У кого-то из дуэлянтов нет свиней. How?",
        )
        .send()
        .await?;
        return Ok(());
    }

    if host_id == part_id {
        bot.answer_callback_query(&q.id)
            .text("Нельзя дуэлить себя идиот гадэмн бля")
            .send()
            .await?;
    }

    bot.edit_message_text_inline(
        q.inline_message_id.as_ref().unwrap(),
        "Готовимся к дуэли...",
    )
    .send()
    .await?;

    sleep(Duration::from_millis(400)).await;

    bot.edit_message_text_inline(
        q.inline_message_id.as_ref().unwrap(),
        "Рассчитываем шансы...",
    )
    .send()
    .await?;

    let pig_first = pigdb::get_pig_by_user_id(pool, host_id).await?;
    let pig_second = pigdb::get_pig_by_user_id(pool, part_id).await?;

    sleep(Duration::from_secs(1)).await;

    bot.edit_message_text_inline(q.inline_message_id.as_ref().unwrap(), "Битва началась...")
        .send()
        .await?;

    if pig_first.duel(&pig_second) {
        let new_weight = pig_first.weight + pig_second.weight * 0.1;
        pigdb::set_pig_weight(pool, new_weight, host_id).await?;

        // Do something to loser

        // Host won Setup result message
        let msg = format!("Победителем оказался: {}", data[1]);
        bot.edit_message_text_inline(q.inline_message_id.as_ref().unwrap(), msg)
            .send()
            .await?;

        bot.answer_callback_query(&q.id)
            .text("Вы проиграли!")
            .send()
            .await?;
    } else {
        let new_weight = pig_second.weight + pig_first.weight * 0.1;
        pigdb::set_pig_weight(pool, new_weight, part_id).await?;

        // Do something to user

        // Setup result message
        let msg = format!("Победителем оказался: {}", q.from.mention().unwrap());
        bot.edit_message_text_inline(q.inline_message_id.as_ref().unwrap(), msg)
            .send()
            .await?;
        bot.answer_callback_query(&q.id)
            .text("Вы выиграли!")
            .send()
            .await?;
    }

    Ok(())
}
