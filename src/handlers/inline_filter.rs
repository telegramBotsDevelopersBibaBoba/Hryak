use std::{str::FromStr, time::Duration};

use crate::config::commands::InlineAdvCommands;
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

    let command_data = &q.query.split_once(" ");

    // Storing a function based on what query is that, if empty -> show 'help'
    //
    let function = match command_data {
        Some((command_str, data)) => match InlineAdvCommands::from_str(command_str) {
            Ok(command) => match command {
                InlineAdvCommands::ChangeName => inline_change_name(bot, &q, data).boxed(),
            },
            Err(_) => inline_all_commands(bot, &q, &pool).boxed(),
        },
        None => match InlineCommands::from_str(command_str) {
            Ok(command) => match command {
                InlineCommands::Hryak => inline_hryak_weight(bot, &q, &pool).boxed(),
                InlineCommands::Shop => inline_shop(bot, &q, &pool).boxed(),
                InlineCommands::Name => inline_name(bot, &q).boxed(),
            },
            Err(_) => inline_all_commands(bot, &q, &pool).boxed(),
        },
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
        InlineQueryResult::Article(test_shop),
        InlineQueryResult::Article(help),
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

    let response = bot
        .answer_inline_query(&q.id, articles)
        .cache_time(0)
        .await?;
    Ok(())
}

pub async fn inline_name(bot: Bot, q: &InlineQuery) -> anyhow::Result<()> {
    let name = articles::inline_name_article().await?;

    let articles = vec![InlineQueryResult::Article(name)];

    let response = bot
        .answer_inline_query(&q.id, articles)
        .cache_time(0)
        .await?;
    Ok(())
}

pub async fn inline_change_name(bot: Bot, q: &InlineQuery, data: &str) -> anyhow::Result<()> {
    let changename = articles::inline_change_name_article(data).await?;

    let articles = vec![InlineQueryResult::Article(changename)];
    let response = bot
        .answer_inline_query(&q.id, articles)
        .cache_time(0)
        .await?;
    Ok(())
}
