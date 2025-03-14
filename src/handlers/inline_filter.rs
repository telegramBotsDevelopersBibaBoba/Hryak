use std::str::FromStr;

use crate::config::commands::InlineAdvCommands;
use crate::config::commands::InlineCommands;
use crate::config::consts;
use crate::controllers;
use crate::handlers::articles;
use crate::StoragePool;
use futures::FutureExt;
use teloxide::payloads::AnswerInlineQuerySetters;
use teloxide::prelude::Request;
use teloxide::{
    prelude::Requester,
    types::{InlineQuery, InlineQueryResult},
    Bot,
};
type HandlerResult = anyhow::Result<()>;

pub async fn filter_inline_commands(bot: Bot, q: InlineQuery, pool: StoragePool) -> HandlerResult {
    let command_str = &q.query; // Extracting a command from the query (we'll have to parse it later for arguments I think tho)
    let command_data = &q.query.split_once(" ");

    // Storing a function based on what query is that, if empty -> show 'help'
    let function = match command_data {
        Some((command_str, data)) => match InlineAdvCommands::from_str(command_str) {
            Ok(command) => match command {
                InlineAdvCommands::ChangeName => {
                    controllers::pig::inline::inline_change_name(bot, &q, data).boxed()
                }
                InlineAdvCommands::Duel => {
                    let bid = data
                        .trim()
                        .parse::<f64>()
                        .unwrap_or(consts::DUEL_DEFAULT_BID);
                    controllers::duel::inline::inline_duel(bot, &q, &pool, bid).boxed()
                }
            },
            Err(_) => inline_all_commands(bot, &q, &pool).boxed(),
        },
        None => match InlineCommands::from_str(command_str) {
            Ok(command) => match command {
                InlineCommands::Hryak => {
                    controllers::pig::inline::inline_hryak_info(bot, &q, &pool).boxed()
                }
                InlineCommands::Shop => {
                    controllers::shop::inline::inline_shop(bot, &q, &pool).boxed()
                }
                InlineCommands::Name => controllers::pig::inline::inline_name(bot, &q).boxed(),
                InlineCommands::Duel => {
                    controllers::duel::inline::inline_duel(bot, &q, &pool, consts::DUEL_DEFAULT_BID)
                        .boxed()
                }
                InlineCommands::Balance => {
                    controllers::economy::inline::inline_balance(bot, &q, &pool).boxed()
                }
                InlineCommands::Gamble => {
                    controllers::gambling::inline::inline_gamble(bot, &q).boxed()
                }
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

// async fn inline_error(bot: Bot, q: &InlineQuery, response: &str) -> anyhow::Result<()> {
//     let error = articles::make_article(
//         "error_some",
//         "Ошибка!",
//         response,
//         response,
//         Some(
//             "https://cdn4.vectorstock.com/i/1000x1000/94/33/scared-pig-running-vector-22489433.jpg",
//         ),
//     );

//     let articles = vec![InlineQueryResult::Article(error)];

//     bot.answer_inline_query(&q.id, articles).send().await?; // Showing all suitable inline buttons
//     Ok(())
// }

async fn inline_all_commands(bot: Bot, q: &InlineQuery, pool: &StoragePool) -> anyhow::Result<()> {
    let hryak = articles::inline_hryak_info_article(pool, q.from.id.0).await?;
    let duel = articles::inline_duel_article(
        pool,
        q.from.id.0,
        q.from.mention().unwrap(),
        consts::DUEL_DEFAULT_BID,
    )
    .await?;
    let help = articles::inline_help_article();
    let shop = articles::inline_shop_article(pool).await?;
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
