use std::{str::FromStr, time::Duration};

use anyhow::anyhow;
use futures::FutureExt;
use crate::{config::consts, controllers::{shop::Offer, user}};

use sqlx::MySqlPool;
use teloxide::{
    payloads::AnswerCallbackQuerySetters,
    prelude::{Request, Requester},
    types::CallbackQuery,
    Bot, RequestError,
};
use tokio::time::sleep;

use crate::{
    config::commands::CallbackCommands, controllers::{pig::proccess_duel_results, shop::OfferType}, db::{pigdb, shopdb, economydb},
    deser_command,
};

pub async fn filter_callback_commands(
    bot: Bot,
    q: CallbackQuery,
    pool: MySqlPool,
) -> Result<(), RequestError> {
    // Called usually when you click on a button
    if q.data.is_none() {
        callback_error(&bot, &q).await.unwrap();
    }

    let data_vec = deser_command!(q.data.as_ref().unwrap());

    // Parsing query and figuring out a command based on it
    let function = match CallbackCommands::from_str(data_vec[0]) {
        Ok(command) => match command {
            CallbackCommands::Shop => callback_shop(&bot, &q, &data_vec[1..], &pool).boxed(), // args are <type> <id>
            CallbackCommands::StartDuel => {
                callbak_start_duel(&bot, &q, &data_vec[1..], q.from.id.0, &pool).boxed()
                // Args are <host-id> <host-mention> <bid>
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

async fn callback_error(bot: &Bot, q: &CallbackQuery) -> anyhow::Result<()> {
    bot.answer_callback_query(&q.id)
        .text("Ошибка, неизвестный коллбэк")
        .await?;
    Ok(())
}

async fn callback_shop(bot: &Bot, q: &CallbackQuery, data: &[&str], pool: &MySqlPool) -> anyhow::Result<()> {
    
    // Todo finish you know
    // bot.edit_message_text_inline(q.inline_message_id.as_ref().unwrap(), "cock")
    //     .text("fuckme")
    //     .reply_markup(make_shop())
    //     .await?;

    if let [offer_type, offer_id] = *data {
        let offer_type = OfferType::from(offer_type);
        let offer_id = offer_id.parse().unwrap();
        let user_id = q.from.id.0;

        let answer = match economydb::try_to_buy(pool, user_id, offer_id, offer_type).await {
            Ok(item) => {
                item.use_item(user_id, pool).await?;
                "Успех"
            }
            _ => "Недостаточно рупий",
        };
        bot.answer_callback_query(&q.id)
            .text(answer)
            .await?;
        Ok(())
    } else {
        return Err(anyhow!("incorrect data {:?}", data))
    }
}

async fn callbak_start_duel(
    bot: &Bot,
    q: &CallbackQuery,
    data: &[&str],
    part_id: u64,
    pool: &MySqlPool,
) -> anyhow::Result<()> {
    if data.is_empty() {
        bot.edit_message_text_inline(
            q.inline_message_id
                .as_ref()
                .ok_or(anyhow!("No data in start duel"))?,
            "Ошибка при дуэли. Отмена",
        )
        .send()
        .await?;
        return Ok(());
    }

    let host_id = data[0].trim().parse::<u64>()?;

    if host_id == part_id {
        bot.answer_callback_query(&q.id)
            .text("Нельзя дуэлить себя идиот гадэмн бля")
            .send()
            .await?;
        return Ok(());
    }

    if !pigdb::pig_exists(pool, part_id).await {
        bot.answer_callback_query(&q.id)
            .text("У тебя нет свиньи! Подуэлиться не получиться.\nИспользуй бота, чтобы она создалась автоматически")
            .send()
            .await?;

        return Ok(());
    }

    let bid = data[2]
        .trim()
        .parse::<f64>()
        .unwrap_or(consts::DUEL_DEFAULT_BID);
    let part_balance = economydb::get_balance(pool, part_id).await?;
    if part_balance < bid {
        bot.answer_callback_query(&q.id)
            .text("Недостаточно денег!")
            .send()
            .await?;
        return Ok(());
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

    bot.edit_message_text_inline(
        q.inline_message_id.as_ref().ok_or(anyhow!("Error"))?,
        "Битва началась...",
    )
    .send()
    .await?;

    if pig_first.duel(&pig_second) {
        proccess_duel_results(pool, host_id, part_id, bid).await?;

        let msg = format!("Победителем оказался: {}", data[1]);
        bot.edit_message_text_inline(q.inline_message_id.as_ref().ok_or(anyhow!("Error"))?, msg)
            .send()
            .await?;

        bot.answer_callback_query(&q.id)
            .text("Вы проиграли!")
            .send()
            .await?;
    } else {
        proccess_duel_results(pool, part_id, host_id, bid).await?;

        let msg = format!("Победителем оказался: {}", q.from.mention().unwrap());
        bot.edit_message_text_inline(q.inline_message_id.as_ref().ok_or(anyhow!("Error"))?, msg)
            .send()
            .await?;
        bot.answer_callback_query(&q.id)
            .text("Вы выиграли!")
            .send()
            .await?;
    }

    Ok(())
}
