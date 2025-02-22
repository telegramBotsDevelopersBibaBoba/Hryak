use std::str::FromStr;

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
mod db;
mod handlers;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting inline bot...");

    let bot = Bot::from_env(); // Setting up bot from TELOXIDE_TOKEN env variable (P.S 'export TELOXIDE_TOKEN=<token>' in terminal)

    // Just boilerplate stuff
    let handler = dptree::entry()
        .branch(Update::filter_inline_query().endpoint(handlers::filter::filter_inline_commands));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
