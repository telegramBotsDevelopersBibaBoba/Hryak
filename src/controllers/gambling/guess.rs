use rand::{rng, Rng};
use teloxide::prelude::Dialogue;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

use crate::config::utils;
use crate::db::economydb;
use crate::StoragePool;

use super::{should_cancel_dialog, HandlerResult};

pub const GUESS_BID_MULTIPLIER: f64 = 40.0;
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
    utils::send_msg(
        &bot,
        &msg,
        "Введи свою ставку:\n(Нужно ответить на сообщение)",
    )
    .await?;
    dialogue.update(GuessState::ReceiveBid).await?;

    Ok(())
}

pub async fn guess_number(
    bot: Bot,
    msg: Message,
    dialogue: GuessDialogue,
    pool: StoragePool,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            if should_cancel_dialog(text) {
                utils::send_msg(&bot, &msg, "Диалог остановлен").await?;
                dialogue.exit().await?;
                return Ok(());
            }
            let bid = match text.parse::<f64>() {
                Ok(bid) => bid,
                Err(why) => {
                    eprintln!("{}", why);
                    utils::send_msg(&bot, &msg, "Отправь число (например, 10)!").await?;
                    return Ok(());
                }
            };

            if let Err(why) =
                economydb::sub_money(&pool, msg.from.as_ref().unwrap().id.0, bid).await
            {
                eprintln!("Not enough money for guess: {}", why);
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
    pool: StoragePool,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            if should_cancel_dialog(text) {
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

pub async fn handle_guess_results(
    pool: &StoragePool,
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
        return Ok(answer_str);
    }

    let answer_str = format!("Вы выиграли {}$", bid * GUESS_BID_MULTIPLIER);
    economydb::add_money(pool, user_id, bid * GUESS_BID_MULTIPLIER).await?;
    Ok(answer_str)
}
