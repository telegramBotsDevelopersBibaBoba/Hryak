use std::{str::FromStr, time::Duration};

use teloxide::{
    payloads::EditMessageText,
    prelude::*,
    sugar::bot::BotMessagesExt,
    types::{
        InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
        ParseMode,
    },
    RequestError,
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

    // Just boilerplate stuff
    let handler = dptree::entry()
        .branch(Update::filter_inline_query().endpoint(handlers::filter::filter_inline_commands));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
