use rand::{rng, Rng};
use sqlx::MySqlPool;
use teloxide::macros::BotCommands;
use teloxide::prelude::Dialogue;
use teloxide::utils::command;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

use crate::config::utils;
use crate::db::economydb;

use super::HandlerResult;

const GUESS_BID_MULTIPLIER: f64 = 1.8;
#[derive(Clone, Default)]
pub enum GuessState {
    #[default]
    Start,
    ReceiveBid,
    ReceiveNumber {
        bid: f64,
    },
}
pub type GuessDialogue = Dialogue<GuessState, InMemStorage<GuessState>>;

pub async fn guess_bid(bot: Bot, msg: Message, dialogue: GuessDialogue) -> HandlerResult {
    utils::send_msg(&bot, &msg, "Введи свою ставку (Нужно ответить на сообщение):\nВведите отмена|cancel на любое из сообщений, чтобы прекратить выполнение команды досрочно").await?;
    dialogue.update(GuessState::ReceiveBid).await?;

    Ok(())
}

pub async fn guess_number(
    bot: Bot,
    msg: Message,
    dialogue: GuessDialogue,
    pool: MySqlPool,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            if text.to_lowercase() == "отмена" || text.to_lowercase() == "cancel" {
                utils::send_msg(&bot, &msg, "Диалог остановлен").await?;
                dialogue.exit().await?;
                return Ok(());
            }
            let bid = match text.parse::<f64>() {
                Ok(bid) => bid,
                Err(why) => {
                    eprintln!("{}", why);
                    utils::send_msg(&bot, &msg, "Отправь число (например, 10.0)!").await?;
                    return Ok(());
                }
            };

            let balance = economydb::get_balance(&pool, msg.from.as_ref().unwrap().id.0)
                .await
                .unwrap_or(0.0);
            if balance < bid {
                utils::send_msg(&bot, &msg, "Недостаточно денег!").await?;
                dialogue.exit().await?;
                return Ok(());
            }

            utils::send_msg(&bot, &msg, "Введи загаданное число от 0 до 100").await?;
            dialogue.update(GuessState::ReceiveNumber { bid }).await?;
        }
        None => utils::send_msg(&bot, &msg, "Отправь число (например, 10.0)!").await?,
    }
    Ok(())
}

pub async fn guess_number_entered(
    bot: Bot,
    msg: Message,
    bid: f64,
    dialogue: GuessDialogue,
    pool: MySqlPool,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            if text.to_lowercase() == "отмена" || text.to_lowercase() == "cancel" {
                utils::send_msg(&bot, &msg, "Диалог остановлен").await?;
                dialogue.exit().await?;
                return Ok(());
            }

            let guessed_number = match text.parse::<u8>() {
                Ok(num) => num,
                Err(why) => {
                    eprintln!("{}", why);
                    utils::send_msg(&bot, &msg, "Отправь число от 1 до 100").await?;
                    return Ok(());
                }
            };
            match handle_guess_results(&pool, bid, guessed_number, msg.from.as_ref().unwrap().id.0)
                .await
            {
                Ok(result_str) => {
                    utils::send_msg(&bot, &msg, &result_str).await?;
                    dialogue.exit().await?;
                }
                Err(why) => {
                    eprintln!("{}", why);
                    utils::send_msg(&bot, &msg, "Произошла ошибка. Пошелнаху").await?;
                    dialogue.exit().await?;
                    return Ok(());
                }
            }
        }
        None => utils::send_msg(&bot, &msg, "Отправь число от 0 до 100").await?,
    }
    Ok(())
}
pub async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Unable to handle the message. Type /help to see the usage.",
    )
    .await?;
    Ok(())
}
pub async fn handle_guess_results(
    pool: &MySqlPool,
    bid: f64,
    guessed_number: u8,
    user_id: u64,
) -> anyhow::Result<String> {
    let number_result = rng().random_range(1..=100);
    if number_result != guessed_number {
        let answer_str = format!(
            "Вы проиграли {}$\nПравильный ответ был: {}",
            bid, number_result
        );
        match economydb::sub_money(pool, user_id, bid).await {
            Ok(_) => {}
            Err(_) => {
                return Ok(String::from("Недостаточно денег!"));
            }
        };
        return Ok(answer_str);
    }

    let answer_str = format!("Вы выиграли {}$", bid * GUESS_BID_MULTIPLIER);
    economydb::add_money(pool, user_id, bid * GUESS_BID_MULTIPLIER - bid).await?;

    Ok(answer_str)
}
