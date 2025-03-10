use rand::Rng;
use sqlx::MySqlPool;
use std::fmt::{self};
use teloxide::prelude::Dialogue;
use teloxide::{dispatching::dialogue::InMemStorage, types::Message, Bot};

use crate::StoragePool;
use crate::{config::utils, db::economydb};

use super::{should_cancel_dialog, HandlerResult};

const RACE_BID_MULTIPLIER: f64 = 1.3;

#[derive(Clone)]
pub struct RacePig {
    name: String,
    speed: f32,
    stamina: f32,
}

impl fmt::Display for RacePig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Имя: {} | Скорость: {} | Выносливость: {}",
            self.name,
            self.speed.ceil(),
            self.stamina.ceil()
        )
    }
}

#[derive(Clone, Default)]
pub enum PigRaceState {
    #[default]
    Start,
    ReceiveBid,
    ReceiveChosenPig {
        pigs: Vec<RacePig>,
        bid: f64,
    },
}
pub type PigRaceDialogue = Dialogue<PigRaceState, InMemStorage<PigRaceState>>;

pub async fn race_bid(bot: Bot, msg: Message, dialogue: PigRaceDialogue) -> HandlerResult {
    utils::send_msg(&bot, &msg, "Введи свою ставку (Нужно ответить на сообщение):\nВведите отмена|cancel, чтобы прекратить выполнение команды досрочно").await?;
    dialogue.update(PigRaceState::ReceiveBid).await?;
    Ok(())
}

pub async fn race_receive_bid(
    bot: Bot,
    msg: Message,
    dialogue: PigRaceDialogue,
    pool: crate::StoragePool,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            if should_cancel_dialog(text) {
                utils::send_msg(&bot, &msg, "Диалог остановлен").await?;
                dialogue.exit().await?;
                return Ok(());
            }
            let bid = match text.parse::<f64>() {
                Ok(num) => num,
                Err(why) => {
                    eprintln!("Parsing bid failed: {}", why);
                    utils::send_msg(&bot, &msg, "Отправь число (например, 10.0)!").await?;
                    return Ok(());
                }
            };
            let balance = economydb::balance(&pool, msg.from.as_ref().unwrap().id.0)
                .await
                .unwrap_or(0.0);
            if balance < bid {
                utils::send_msg(&bot, &msg, "Недостаточно денег!").await?;
                dialogue.exit().await?;
                return Ok(());
            }

            let pig_first = RacePig {
                name: "Vicinity".to_string(),
                speed: rand::rng().random_range(1.0..=10.0),
                stamina: rand::rng().random_range(1.0..=5.0),
            };

            let pig_second = RacePig {
                name: "Afrodita".to_string(),
                speed: rand::rng().random_range(1.0..=10.0),
                stamina: rand::rng().random_range(1.0..=5.0),
            };
            let pig_third = RacePig {
                name: "Anal".to_string(),
                speed: rand::rng().random_range(1.0..=10.0),
                stamina: rand::rng().random_range(1.0..=5.0),
            };
            let pig_fourth = RacePig {
                name: "Niggler".to_string(),
                speed: rand::rng().random_range(1.0..=10.0),
                stamina: rand::rng().random_range(1.0..=5.0),
            };
            let pig_fifth = RacePig {
                name: "Lucifer".to_string(),
                speed: rand::rng().random_range(1.0..=10.0),
                stamina: rand::rng().random_range(1.0..=5.0),
            };

            let pigs = vec![pig_first, pig_second, pig_third, pig_fourth, pig_fifth];
            let mut msg_str = String::new();
            for (id, pig) in pigs.iter().enumerate() {
                msg_str += &format!("{}. {}\n", id + 1, pig.to_string());
            }
            msg_str += "Выбери свинью по номеру:";
            utils::send_msg(&bot, &msg, &msg_str).await?;
            dialogue
                .update(PigRaceState::ReceiveChosenPig { pigs: pigs, bid })
                .await?;
        }
        None => {
            todo!()
        }
    }
    Ok(())
}

pub async fn race_receive_number(
    bot: Bot,
    dialogue: PigRaceDialogue,
    (pigs, bid): (Vec<RacePig>, f64),
    msg: Message,
    pool: StoragePool,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            if should_cancel_dialog(text) {
                utils::send_msg(&bot, &msg, "Диалог остановлен").await?;
                dialogue.exit().await?;
                return Ok(());
            }

            let chosen_id = match text.trim().parse::<u8>() {
                Ok(id) => id,
                Err(why) => {
                    eprintln!("Could not parse chosen id: {}", why);
                    utils::send_msg(&bot, &msg, &format!("Введите число от 1 до {}", pigs.len()))
                        .await?;
                    return Ok(());
                }
            };

            if chosen_id < 1 || chosen_id as usize > pigs.len() {
                utils::send_msg(&bot, &msg, &format!("Введите число от 1 до {}", pigs.len()))
                    .await?;
                return Ok(());
            }

            // Алгоритм определения победителя
            let stages = 3; // Количество этапов забега
            let mut progress: Vec<f32> = vec![0.0; pigs.len()]; // Прогресс каждой свиньи

            // Симуляция забега
            for stage in 1..=stages {
                let mut stage_msg = format!("Этап {}:\n", stage);

                for (i, pig) in pigs.iter().enumerate() {
                    // Базовый прогресс = скорость * случайный множитель (0.8–1.2)
                    let random_factor = rand::rng().random_range(0.8..=1.2);
                    let mut stage_progress = pig.speed * random_factor;

                    // Штраф за усталость: чем ниже выносливость, тем больше снижение
                    let stamina_factor = pig.stamina as f32 / 5.0 as f32; // Нормализуем (предположим, stamina от 0 до 10)
                    let fatigue =
                        (stages as f32 - stage as f32 + 1.0) * (1.0 - stamina_factor) * 0.1;
                    stage_progress -= fatigue.max(0.0); // Усталость снижает прогресс

                    progress[i] += stage_progress;

                    // Добавляем информацию в сообщение
                    stage_msg.push_str(&format!("{}: прогресс {:.1}\n", pig.name, progress[i]));
                }

                // Отправляем промежуточный результат
                utils::send_msg(&bot, &msg, &stage_msg).await?;
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                // Пауза для эффекта
            }

            // Определяем победителя
            let winner_idx = progress
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .map(|(idx, _)| idx)
                .unwrap();
            let winner = &pigs[winner_idx];

            // Проверяем ставку пользователя
            let user_chose_winner = chosen_id as usize - 1 == winner_idx;
            let result_msg = if user_chose_winner {
                let winnings = bid * RACE_BID_MULTIPLIER; // Коэффициент выигрыша
                                                          // Здесь можно обновить баланс в базе данных (pool)
                economydb::add_money(&pool, msg.from.as_ref().unwrap().id.0, winnings - bid)
                    .await?;
                format!(
                    "Победила свинья {}! Ты выиграл {}$!",
                    winner.name,
                    winnings.floor()
                )
            } else {
                economydb::sub_money(&pool, msg.from.as_ref().unwrap().id.0, bid).await?;
                format!(
                    "Победила свинья {}! Ты проиграл ставку {}$.",
                    winner.name, bid
                )
            };
            dialogue.exit().await?;
            utils::send_msg(&bot, &msg, &result_msg).await?;
        }
        None => {
            utils::send_msg(&bot, &msg, &format!("Введите число от 1 до {}", pigs.len())).await?;
        }
    }

    Ok(())
}
