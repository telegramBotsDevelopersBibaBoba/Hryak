use std::fmt::Display;

use rand::Rng;
use sqlx::types::chrono::Utc;
use teloxide::prelude::Dialogue;
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

use crate::config::utils;
use crate::db::{economydb, gamblingdb};
use crate::StoragePool;

use super::{should_cancel_dialog, HandlerResult};

const LOCATIONS: [&str; 9] = [
    "Грязевая долина",
    "БГИТУ",
    "БГТУ",
    "Владивосток",
    "Фокино",
    "ВУЦ при БГИТУ",
    "Курск",
    "Киев",
    "ВУЦ при ДВФУ",
];

#[derive(Clone)]
pub enum TreasureDifficulty {
    Easy,
    Medium,
    Hard,
}
impl TreasureDifficulty {
    fn get_random_diff() -> Self {
        match rand::rng().random_range(0..3) {
            0 => Self::Easy,
            1 => Self::Medium,
            2 => Self::Hard,
            _ => Self::Medium,
        }
    }
}
impl Display for TreasureDifficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Easy => write!(f, "(Легко)"),
            Self::Medium => write!(f, "(Средне)"),
            Self::Hard => write!(f, "(Сложно)"),
        }
    }
}

#[derive(Clone)]
pub struct TreasureLocation {
    name: String,
    difficulty: TreasureDifficulty,
}

impl Display for TreasureLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.difficulty)
    }
}

#[derive(Clone, Default)]
pub enum TreasureState {
    #[default]
    Start,
    ReceiveBid,
    ReceiveLocation {
        bid: f64,
        locations: Vec<TreasureLocation>,
    },
}
pub type TreasureDialogue = Dialogue<TreasureState, InMemStorage<TreasureState>>;

pub async fn treasure_bid(
    bot: Bot,
    msg: Message,
    dialogue: TreasureDialogue,
    pool: StoragePool,
) -> HandlerResult {
    let last_time_played =
        gamblingdb::treasurehunt_last_time(&pool, msg.from.as_ref().unwrap().id.0).await?;
    if last_time_played.is_some() && (Utc::now() - last_time_played.unwrap()).num_hours() < 2 {
        utils::send_msg(
            &bot,
            &msg,
            &format!(
                "Попробуйте сыграть снова через {} часов.",
                2 - (Utc::now() - last_time_played.unwrap()).num_hours()
            ),
        )
        .await?;

        return Ok(());
    }

    utils::send_msg(
        &bot,
        &msg,
        "Введи свою ставку:\n(Нужно ответить на сообщение)",
    )
    .await?;
    dialogue.update(TreasureState::ReceiveBid).await?;

    Ok(())
}

pub async fn treasure_receive_bid(
    bot: Bot,
    msg: Message,
    dialogue: TreasureDialogue,
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

            if let Err(_) = economydb::sub_money(&pool, msg.from.as_ref().unwrap().id.0, bid).await
            {
                utils::send_msg(&bot, &msg, "Недостаточно денег!").await?;
                dialogue.exit().await?;
                return Ok(());
            }

            let generate_unique_location = |max: u64, exclude: &Vec<u64>| {
                let mut id;
                loop {
                    id = rand::rng().random_range(0..max);
                    if !exclude.contains(&id) {
                        break id;
                    }
                }
            };

            let mut locs: Vec<TreasureLocation> = Vec::new();

            let mut msg_text = format!("Куда отправить хряка? Выберите число\n");
            let mut idxs = vec![];
            for i in 0..4 {
                let idx = generate_unique_location(LOCATIONS.len() as u64, &idxs);
                idxs.push(idx);
                locs.push(TreasureLocation {
                    name: LOCATIONS[idx as usize].to_string(),
                    difficulty: TreasureDifficulty::get_random_diff(),
                });
                msg_text += &format!("{}. {}\n", i + 1, locs[i].to_string());
            }

            utils::send_msg(&bot, &msg, &msg_text).await?;
            dialogue
                .update(TreasureState::ReceiveLocation {
                    bid,
                    locations: locs,
                })
                .await?;
        }
        None => utils::send_msg(&bot, &msg, "Отправь число (например, 10)!").await?,
    }
    Ok(())
}

pub async fn location_chosen(
    bot: Bot,
    msg: Message,
    (bid, locations): (f64, Vec<TreasureLocation>),
    dialogue: TreasureDialogue,
    pool: StoragePool,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            if should_cancel_dialog(text) {
                utils::send_msg(&bot, &msg, "Диалог остановлен").await?;
                dialogue.exit().await?;
                return Ok(());
            }

            let chosen_id = match text.trim().parse::<usize>() {
                Ok(id) if id >= 1 && id <= locations.len() => id - 1, // Индекс с 0
                _ => {
                    utils::send_msg(
                        &bot,
                        &msg,
                        &format!("Введите число от 1 до {}", locations.len()),
                    )
                    .await?;
                    return Ok(());
                }
            };

            let chosen_location = &locations[chosen_id];
            let result = simulate_treasure_hunt(chosen_location, bid);

            match result {
                TreasureResult::Coins(amount) => {
                    let total_amount = bid + amount; // Учитываем ставку
                    economydb::add_money(&pool, msg.from.as_ref().unwrap().id.0, total_amount)
                        .await?;
                    utils::send_msg(
                        &bot,
                        &msg,
                        &format!("Хряк нашёл {} монет в {:.2}! 💹", amount, chosen_location),
                    )
                    .await?;
                }
                TreasureResult::Nothing => {
                    utils::send_msg(
                        &bot,
                        &msg,
                        &format!("Хряк вернулся из {} с пустыми копытами 📉", chosen_location),
                    )
                    .await?;
                }
            }

            gamblingdb::treasurehunt_played(&pool, msg.from.as_ref().unwrap().id.0).await?;
            dialogue.exit().await?;
        }
        None => {
            utils::send_msg(
                &bot,
                &msg,
                &format!("Отправь число от 1 до {}", locations.len()),
            )
            .await?
        }
    }
    Ok(())
}

#[derive(Clone)]
enum TreasureResult {
    Coins(f64), // Найдено монет
    Nothing,    // Ничего не найдено
}
fn simulate_treasure_hunt(location: &TreasureLocation, bid: f64) -> TreasureResult {
    let mut rng = rand::rng();

    match location.difficulty {
        TreasureDifficulty::Easy => {
            if rng.random_bool(0.6) {
                let amount = rng.random_range(20.0..=40.0);
                TreasureResult::Coins(bid * (amount / 100.0))
            } else {
                TreasureResult::Nothing
            }
        }
        TreasureDifficulty::Medium => {
            if rng.random_bool(0.4) {
                let amount = rng.random_range(40.0..=80.0);
                TreasureResult::Coins(bid * (amount / 100.0))
            } else {
                TreasureResult::Nothing
            }
        }
        TreasureDifficulty::Hard => {
            if rng.random_bool(0.2) {
                let amount = rng.random_range(80.0..=150.0);
                TreasureResult::Coins(bid * (amount / 100.0))
            } else {
                TreasureResult::Nothing
            }
        }
    }
}
