use std::time::Duration;

use config::commands;

use controllers::gambling::guess::GuessState;
use controllers::gambling::pigrace::PigRaceState;
use controllers::gambling::treasurehunt::TreasureState;
use controllers::gambling::{self, GambleCommands};
use controllers::shop::{self};

use r2d2::Pool;
use redis::Client;
use sqlx::MySqlPool;
use teloxide::dispatching::dialogue::{self, InMemStorage};
use teloxide::dispatching::UpdateHandler;
use teloxide::dptree::filter_async;
use teloxide::prelude::*;

mod config;
mod controllers;
mod db;
mod handlers;

#[derive(Clone, Debug)]
struct StoragePool {
    mysql_pool: MySqlPool,
    redis_pool: Pool<Client>,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting inline bot...");

    let con_str = "mysql://root:root@localhost:3306/hryak";

    let mysql_pool = sqlx::mysql::MySqlPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&con_str)
        .await
        .expect("Cant connect fuck it");

    let bot = Bot::from_env(); // Setting up bot from TELOXIDE_TOKEN env variable (P.S 'export TELOXIDE_TOKEN=<token>' in terminal)

    let redis_connection = redis::Client::open("redis://127.0.0.1").unwrap();
    let redis_pool = r2d2::Pool::builder().build(redis_connection).unwrap();

    let storage_pool = StoragePool {
        mysql_pool,
        redis_pool,
    };

    tokio::spawn(shop::generate_new_offers());

    Dispatcher::builder(bot, scheme())
        .dependencies(dptree::deps![
            storage_pool,
            InMemStorage::<PigRaceState>::new(),
            InMemStorage::<GuessState>::new(),
            InMemStorage::<TreasureState>::new()
        ])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn scheme() -> UpdateHandler<anyhow::Error> {
    let general_handler =
        Update::filter_message().branch(filter_async(controllers::general::handle_message));

    let economy_commands = Update::filter_message()
        .filter_command::<commands::EconomyCommands>()
        .endpoint(controllers::economy::economy_handle);

    let callback_handler = Update::filter_callback_query()
        .filter_async(controllers::general::handle_other)
        .endpoint(handlers::callback::filter_callback_commands);

    let inline_handler = Update::filter_inline_query()
        .filter_async(controllers::general::handle_inline)
        .endpoint(handlers::inline_filter::filter_inline_commands);

    let feedback_handler = Update::filter_chosen_inline_result()
        .filter_async(controllers::general::handle_other)
        .endpoint(handlers::feedback::filter_inline_chosen_command);

    let guess_commands = teloxide::filter_command::<GambleCommands, _>().branch(
        dptree::case![GuessState::Start]
            .branch(dptree::case![GambleCommands::Guess].endpoint(gambling::guess::guess_bid)),
    );
    let guess_handler = Update::filter_message()
        .branch(guess_commands)
        .branch(dptree::case![GuessState::ReceiveBid].endpoint(gambling::guess::guess_number))
        .branch(
            dptree::case![GuessState::ReceiveNumber { bid }]
                .endpoint(gambling::guess::guess_number_entered),
        );

    let race_commands = teloxide::filter_command::<GambleCommands, _>().branch(
        dptree::case![PigRaceState::Start]
            .branch(dptree::case![GambleCommands::Race].endpoint(gambling::pigrace::race_bid)),
    );

    let pigrace_handler = Update::filter_message()
        .branch(race_commands)
        .branch(
            dptree::case![PigRaceState::ReceiveBid].endpoint(gambling::pigrace::race_receive_bid),
        )
        .branch(
            dptree::case![PigRaceState::ReceiveChosenPig { pigs, bid }]
                .endpoint(gambling::pigrace::race_receive_number),
        );

    let treasure_commands = teloxide::filter_command::<GambleCommands, _>().branch(
        dptree::case![TreasureState::Start].branch(
            dptree::case![GambleCommands::TreasureHunt]
                .endpoint(gambling::treasurehunt::treasure_bid),
        ),
    );
    let treasure_handler = Update::filter_message()
        .branch(treasure_commands)
        .branch(
            dptree::case![TreasureState::ReceiveBid]
                .endpoint(gambling::treasurehunt::treasure_receive_bid),
        )
        .branch(
            dptree::case![TreasureState::ReceiveLocation { bid, locations }]
                .endpoint(gambling::treasurehunt::location_chosen),
        );

    let guess_dialogue =
        dialogue::enter::<Update, InMemStorage<GuessState>, GuessState, _>().branch(guess_handler);
    let pigrace_dialogue = dialogue::enter::<Update, InMemStorage<PigRaceState>, PigRaceState, _>()
        .branch(pigrace_handler);
    let treasure_dialogue =
        dialogue::enter::<Update, InMemStorage<TreasureState>, TreasureState, _>()
            .branch(treasure_handler);

    dptree::entry()
        .branch(general_handler)
        .branch(economy_commands)
        .branch(inline_handler)
        .branch(callback_handler)
        .branch(feedback_handler)
        .branch(guess_dialogue)
        .branch(pigrace_dialogue)
        .branch(treasure_dialogue)
}
