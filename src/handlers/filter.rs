use std::{str::FromStr, time::Duration};

use anyhow::anyhow;
use futures::FutureExt;
use sqlx::MySqlPool;
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

use crate::{
    config::commands::InlineCommands,
    db::{pigdb::get_pig_weight, userdb},
};

#[derive(thiserror::Error, Debug)]
pub enum MyError {
    #[error(transparent)]
    RequestError(#[from] teloxide::RequestError),

    #[allow(unused)]
    #[error("unknown error: {0}")]
    Unknown(String),
}

pub async fn filter_inline_commands(
    bot: Bot,
    q: InlineQuery,
    pool: MySqlPool,
) -> Result<(), RequestError> {
    let command_str = &q.query; // Extracting a command from the query (we'll have to parse it later for arguments I think tho)

    // Storing a function based on what query is that, if empty -> show 'help'
    let function = match InlineCommands::from_str(&command_str) {
        Ok(command) => match command {
            InlineCommands::Hryak => inline_all_commands(bot, &q, &pool).boxed(),
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
    let hryak = inline_hryak_weight_article(q, pool).await.unwrap();
    let help = inline_help_article(q, pool).await.unwrap();

    let articles = vec![
        InlineQueryResult::Article(hryak),
        InlineQueryResult::Article(help),
    ];

    let response = bot.answer_inline_query(&q.id, articles).send().await?; // Showing all suitable inline buttons
    Ok(())
}

pub async fn inline_hryak_weight_article(
    q: &InlineQuery,
    pool: &MySqlPool,
) -> anyhow::Result<InlineQueryResultArticle> {
    let mass = match get_pig_weight(pool, q.from.id.0).await {
        Ok(mass) => mass,
        Err(why) => {
            userdb::create_user(pool, q.from.id.0, &q.from.first_name).await?;
            return Err(anyhow!("{}", why));
        }
    };

    let hrundel_weight = InlineQueryResultArticle::new(
        "02".to_string(),
        "Узнать массу хряка".to_string(),
        InputMessageContent::Text(InputMessageContentText::new(
            format!("Размер хряка: {} кг.", mass.to_string())
        )),
    )
    .description("Hryak")
    .thumbnail_url("https://sputnik.kz/img/858/06/8580645_0:0:3117:2048_600x0_80_0_1_81d5b1f42e05e39353aa388a4e55cb34.jpg".parse().unwrap());

    Ok(hrundel_weight)
}
pub async fn inline_help_article(
    q: &InlineQuery,
    pool: &MySqlPool,
) -> anyhow::Result<InlineQueryResultArticle> {
    let button =
        InlineKeyboardButton::switch_inline_query_current_chat("Узнать массу хряка", "hryak");
    let help = InlineQueryResultArticle::new(
        "01".to_string(),
        "Узнать все доступные команды".to_string(),
        InputMessageContent::Text(InputMessageContentText::new(
            "Вот список доступных комманд:",
        )),
    )
    .description("Узнай все доступные команды")
    .thumbnail_url(
        "https://thumbs.dreamstime.com/z/lot-pigs-d-rendered-illustration-127843482.jpg"
            .parse()
            .unwrap(),
    )
    .reply_markup(InlineKeyboardMarkup::new([[button]]));
    Ok(help)
}
