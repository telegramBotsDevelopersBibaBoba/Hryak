use std::{str::FromStr, time::Duration};

use crate::handlers::articles;
use crate::{
    config::commands::InlineCommands,
    db::{pigdb::get_pig_weight, userdb},
};
use anyhow::anyhow;
use futures::FutureExt;
use sqlx::MySqlPool;
use teloxide::payloads::AnswerInlineQuerySetters;
use teloxide::{
    prelude::{Request, Requester},
    respond,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQuery, InlineQueryResult,
        InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    },
    Bot, RequestError,
};
use tokio::time::sleep;

pub async fn filter_inline_commands(
    bot: Bot,
    q: InlineQuery,
    pool: MySqlPool,
) -> Result<(), RequestError> {
    let command_str = &q.query; // Extracting a command from the query (we'll have to parse it later for arguments I think tho)

    // Storing a function based on what query is that, if empty -> show 'help'
    let function = match InlineCommands::from_str(&command_str) {
        Ok(command) => match command {
            InlineCommands::Hryak => inline_hryak_weight(bot, &q, &pool).boxed(),
            InlineCommands::Shop => inline_shop(bot, &q, &pool).boxed(),
        },
        Err(_) => inline_all_commands(bot, &q, &pool).boxed(),
    };
    // Executing the function
    let resp = function.await;

    // Checking for errors
    if let Err(why) = resp {
        println!("{}", why);
    }

    Ok(())
}

pub async fn inline_all_commands(
    bot: Bot,
    q: &InlineQuery,
    pool: &MySqlPool,
) -> anyhow::Result<()> {
    let hryak = articles::inline_hryak_weight_article(q, pool).await?;
    let help = articles::inline_help_article(q, pool).await.unwrap();
    let test_shop = articles::TEST_inline_shop_article(q, pool).await.unwrap();

    // Showing several articles at once
    let articles = vec![
        InlineQueryResult::Article(hryak),
        InlineQueryResult::Article(help),
        InlineQueryResult::Article(test_shop),
    ];

    let response = bot
        .answer_inline_query(&q.id, articles)
        .cache_time(0)
        .await?; // Showing all suitable inline buttons
    Ok(())
}

pub async fn inline_hryak_weight(
    bot: Bot,
    q: &InlineQuery,
    pool: &MySqlPool,
) -> anyhow::Result<()> {
    let hryak = articles::inline_hryak_weight_article(q, pool).await?;

    let articles = vec![InlineQueryResult::Article(hryak)];

    let response = bot
        .answer_inline_query(&q.id, articles)
        .cache_time(0)
        .await?; // Showing all suitable inline buttons
    Ok(())
}

pub async fn inline_shop(bot: Bot, q: &InlineQuery, pool: &MySqlPool) -> anyhow::Result<()> {
    let shop = articles::TEST_inline_shop_article(q, pool).await?;

    let articles = vec![InlineQueryResult::Article(shop)];

    let response = bot.answer_inline_query(&q.id, articles).send().await?;
    Ok(())
}
