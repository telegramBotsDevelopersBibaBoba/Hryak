use std::{str::FromStr, time::Duration};

use config::{commands, utils};

use controllers::gambling::{self, GuessDialogue, GuessState};
use controllers::shop::{self, OfferType};
use handlers::keyboard;
use sqlx::MySqlPool;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::utils::command::BotCommands;
use teloxide::{
    payloads::EditMessageText,
    prelude::*,
    sugar::bot::BotMessagesExt,
    types::{
        InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
        ParseMode,
    },
};

mod config;
mod controllers;
mod db;
mod handlers;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting inline bot...");

    let con_str = "mysql://klewy:root@localhost:3306/hryak";

    let pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&con_str)
        .await
        .expect("Cant connect fuck it");

    let bot = Bot::from_env(); // Setting up bot from TELOXIDE_TOKEN env variable (P.S 'export TELOXIDE_TOKEN=<token>' in terminal)

    tokio::spawn(shop::generate_new_offers());

    // Just boilerplate stuff
    let handler = dptree::entry()
        .branch(
            Update::filter_inline_query().endpoint(handlers::inline_filter::filter_inline_commands), // TODO: split this in different inline handlers (if possible),
        )
        .branch(
            Update::filter_callback_query().endpoint(handlers::callback::filter_callback_commands),
        )
        .branch(
            Update::filter_chosen_inline_result()
                .enter_dialogue::<Update, InMemStorage<GuessState>, GuessState>()
                .branch(dptree::case![GuessState::Start])
                .endpoint(start)
                .branch(dptree::case![GuessState::ReceiveBid])
                .endpoint(get_bid)
                .filter_async(handlers::feedback::filter_inline_chosen_command),
        )
        .branch(
            Update::filter_message()
                .filter_command::<commands::EconomyCommands>()
                .endpoint(controllers::economy::economy_handle),
        );
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![pool, InMemStorage::<GuessState>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

pub async fn start(bot: Bot, msg: Message, dialogue: GuessDialogue) -> ResponseResult<()> {
    utils::send_msg(&bot, &msg, "Введи свою ставку:").await?;
    dialogue.update(GuessState::ReceiveBid).await.unwrap();

    Ok(())
}

pub async fn get_bid(bot: Bot, msg: Message, dialogue: GuessDialogue) -> ResponseResult<()> {
    match msg.text() {
        Some(text) => {
            utils::send_msg(&bot, &msg, "Введи загаданное число от 0 до 100").await?;
            dialogue
                .update(GuessState::ReceiveNumber {
                    bid: text.parse::<f64>().unwrap(),
                })
                .await
                .unwrap();
        }
        None => utils::send_msg(&bot, &msg, "Отправь число (например, 10.0)!").await?,
    }
    Ok(())
}
