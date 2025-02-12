use std::str::FromStr;

use crate::config::commands::InlineAdvCommands;
use crate::config::commands::InlineCommands;
use crate::db::economydb;
use crate::db::pigdb;
use crate::handlers::articles;
use futures::FutureExt;
use sqlx::MySqlPool;
use teloxide::payloads::AnswerInlineQuerySetters;
use teloxide::prelude::Request;
use teloxide::types::InlineQueryResultArticle;
use teloxide::types::InputMessageContent;
use teloxide::types::InputMessageContentText;
use teloxide::{
    prelude::Requester,
    types::{InlineQuery, InlineQueryResult},
    Bot, RequestError,
};
pub async fn filter_inline_commands(
    bot: Bot,
    q: InlineQuery,
    pool: MySqlPool,
) -> Result<(), RequestError> {
    // Called always

    if q.from.username.as_ref().is_none() {
        inline_error(
            bot,
            &q,
            "Не найдено имя пользователя. Без него бот работает некорректно",
            "Имя пользователя можно добавить в настройках аккаунта",
        )
        .await
        .unwrap();
        return Ok(());
    }

    let command_str = &q.query; // Extracting a command from the query (we'll have to parse it later for arguments I think tho)
    let command_data = &q.query.split_once(" ");

    // Storing a function based on what query is that, if empty -> show 'help'
    let function = match command_data {
        Some((command_str, data)) => match InlineAdvCommands::from_str(command_str) {
            Ok(command) => match command {
                InlineAdvCommands::ChangeName => inline_change_name(bot, &q, data).boxed(),
                InlineAdvCommands::Duel => inline_duel_bid(bot, &q, &pool, data).boxed(),
            },
            Err(_) => inline_all_commands(bot, &q, &pool).boxed(),
        },
        None => match InlineCommands::from_str(command_str) {
            Ok(command) => match command {
                InlineCommands::Hryak => inline_hryak_info(bot, &q, &pool).boxed(),
                InlineCommands::Shop => inline_shop(bot, &q, &pool).boxed(),
                InlineCommands::Name => inline_name(bot, &q).boxed(),
                InlineCommands::Duel => inline_duel(bot, &q, &pool).boxed(),
                InlineCommands::Balance => inline_balance(bot, &q, &pool).boxed(),
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

async fn inline_error(
    bot: Bot,
    q: &InlineQuery,
    descr: &str,
    response: &str,
) -> anyhow::Result<()> {
    let error = InlineQueryResultArticle::new(
        "error_inline".to_string(),
        "Ошибка!",
        InputMessageContent::Text(InputMessageContentText::new(response)),
    )
    .description(descr)
    .thumbnail_url(
        "https://www.shutterstock.com/image-photo/large-hairy-pig-pink-nose-600nw-642075343.jpg"
            .parse()
            .unwrap(),
    );

    let articles = vec![InlineQueryResult::Article(error)];

    bot.answer_inline_query(&q.id, articles).send().await?; // Showing all suitable inline buttons
    Ok(())
}

async fn inline_all_commands(bot: Bot, q: &InlineQuery, pool: &MySqlPool) -> anyhow::Result<()> {
    let hryak = articles::inline_hryak_info_article(q, pool).await?;
    let duel =
        articles::inline_duel_article(pool, q.from.id.0, q.from.mention().unwrap(), 1.0).await?;
    let help = articles::inline_help_article();
    let shop = articles::inline_shop_article(q, pool);
    let balance = articles::inline_balance_article(pool, q.from.id.0).await?;

    // Showing several articles at once
    let articles = vec![
        InlineQueryResult::Article(hryak),
        InlineQueryResult::Article(duel),
        InlineQueryResult::Article(balance),
        InlineQueryResult::Article(shop),
        InlineQueryResult::Article(help),
    ];

    bot.answer_inline_query(&q.id, articles)
        .cache_time(0)
        .await?; // Showing all suitable inline buttons
    Ok(())
}

async fn inline_hryak_info(bot: Bot, q: &InlineQuery, pool: &MySqlPool) -> anyhow::Result<()> {
    let hryak = articles::inline_hryak_info_article(q, pool).await?;

    let articles = vec![InlineQueryResult::Article(hryak)];

    bot.answer_inline_query(&q.id, articles)
        .cache_time(0)
        .await?; // Showing all suitable inline buttons
    Ok(())
}

async fn inline_shop(bot: Bot, q: &InlineQuery, pool: &MySqlPool) -> anyhow::Result<()> {
    let shop = articles::inline_shop_article(q, pool);

    let articles = vec![InlineQueryResult::Article(shop)];

    bot.answer_inline_query(&q.id, articles)
        .cache_time(0)
        .await?;
    Ok(())
}

async fn inline_name(bot: Bot, q: &InlineQuery) -> anyhow::Result<()> {
    let name = articles::inline_name_article();

    let articles = vec![InlineQueryResult::Article(name)];

    bot.answer_inline_query(&q.id, articles)
        .cache_time(0)
        .await?;
    Ok(())
}

async fn inline_change_name(bot: Bot, q: &InlineQuery, data: &str) -> anyhow::Result<()> {
    let changename = articles::inline_change_name_article(data);

    let articles = vec![InlineQueryResult::Article(changename)];
    bot.answer_inline_query(&q.id, articles)
        .cache_time(0)
        .await?;
    Ok(())
}

async fn inline_duel(bot: Bot, q: &InlineQuery, pool: &MySqlPool) -> anyhow::Result<()> {
    if !pigdb::pig_exists(pool, q.from.id.0).await {
        let no_pig = articles::inline_no_pig_article();
        let articles = vec![InlineQueryResult::Article(no_pig)];
        bot.answer_inline_query(&q.id, articles)
            .cache_time(0)
            .await?;
        return Ok(());
    }

    let duel =
        articles::inline_duel_article(&pool, q.from.id.0, q.from.mention().unwrap(), 1.0).await?;
    let articles = vec![InlineQueryResult::Article(duel)];

    bot.answer_inline_query(&q.id, articles)
        .cache_time(0)
        .await?;
    Ok(())
}
async fn inline_duel_bid(
    bot: Bot,
    q: &InlineQuery,
    pool: &MySqlPool,
    data: &str,
) -> anyhow::Result<()> {
    if !pigdb::pig_exists(pool, q.from.id.0).await {
        let no_pig = articles::inline_no_pig_article();
        let articles = vec![InlineQueryResult::Article(no_pig)];
        bot.answer_inline_query(&q.id, articles)
            .cache_time(0)
            .await?;
        return Ok(());
    }

    let bid = data.trim().parse::<f64>().unwrap_or(10.0);

    let duel =
        articles::inline_duel_article(&pool, q.from.id.0, q.from.mention().unwrap(), bid).await?;
    let articles = vec![InlineQueryResult::Article(duel)];

    bot.answer_inline_query(&q.id, articles)
        .cache_time(0)
        .await?;
    Ok(())
}

async fn inline_balance(bot: Bot, q: &InlineQuery, pool: &MySqlPool) -> anyhow::Result<()> {
    let balance_article = articles::inline_balance_article(pool, q.from.id.0).await?;

    let articles = vec![InlineQueryResult::Article(balance_article)];

    bot.answer_inline_query(&q.id, articles)
        .cache_time(0)
        .await?;
    Ok(())
}
