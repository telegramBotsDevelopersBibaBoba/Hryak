use std::{str::FromStr, time::Duration};

use config::{commands, utils};

use controllers::gambling::{self, GambleComamnds, GuessDialogue, GuessState};
use controllers::shop::{self, OfferType};
use handlers::keyboard;
use rand::{rng, Rng};
use sqlx::MySqlPool;
use teloxide::dispatching::dialogue::{self, InMemStorage};
use teloxide::dispatching::UpdateHandler;
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

    Dispatcher::builder(bot, scheme())
        .dependencies(dptree::deps![pool, InMemStorage::<GuessState>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn scheme() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    let economy_commands = Update::filter_message()
        .filter_command::<commands::EconomyCommands>()
        .endpoint(controllers::economy::economy_handle);

    let callback_handler =
        Update::filter_callback_query().endpoint(handlers::callback::filter_callback_commands);

    let inline_handler =
        Update::filter_inline_query().endpoint(handlers::inline_filter::filter_inline_commands);

    let feedback_handler = Update::filter_chosen_inline_result()
        .endpoint(handlers::feedback::filter_inline_chosen_command);

    let gamble_commands = teloxide::filter_command::<GambleComamnds, _>().branch(
        dptree::case![GuessState::Start]
            .branch(dptree::case![GambleComamnds::Guess].endpoint(gambling::guess_bid)),
    );
    let gamble_handler = Update::filter_message()
        .branch(gamble_commands)
        .branch(dptree::case![GuessState::ReceiveBid].endpoint(gambling::guess_number))
        .branch(
            dptree::case![GuessState::ReceiveNumber { bid }]
                .endpoint(gambling::guess_number_entered),
        );

    let dialogue_handler =
        dialogue::enter::<Update, InMemStorage<GuessState>, GuessState, _>().branch(gamble_handler);
    dptree::entry()
        .branch(economy_commands)
        .branch(inline_handler)
        .branch(callback_handler)
        .branch(feedback_handler)
        .branch(dialogue_handler)
}
