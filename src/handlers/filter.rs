use std::{str::FromStr, time::Duration};

use futures::FutureExt;
use sqlx::MySqlPool;
use teloxide::{
    prelude::{Request, Requester},
    respond,
    types::{
        InlineQuery, InlineQueryResult, InlineQueryResultArticle, InputMessageContent,
        InputMessageContentText,
    },
    Bot, RequestError,
};
use tokio::time::sleep;

use crate::{config::commands::InlineCommands, db::{pigdb::get_pig_weight, userdb}};

pub async fn filter_inline_commands(bot: Bot, q: InlineQuery, pool: MySqlPool) -> Result<(), RequestError> {
    let command_str = &q.query; // Extracting a command from the query (we'll have to parse it later for arguments I think tho)

    // Storing a function based on what query is that, if empty -> show 'help'
    let function = match InlineCommands::from_str(&command_str) {
        Ok(command) => match command {
            InlineCommands::Hryak => inline_get_hryak_size(bot, &q, &pool).boxed(),
        },
        Err(_) => inline_help(bot, &q).boxed(),
    };

    // Executing the function
    let resp = function.await;

    // Checking for errors
    if let Err(why) = resp {
        println!("{}", why);
    }

    Ok(())
}

pub async fn inline_help(bot: Bot, q: &InlineQuery) -> Result<(), RequestError> {
    // Creating an inline button? 
    let help = InlineQueryResultArticle::new(
        "01".to_string(),
        "Узнать все доступные команды".to_string(),
        InputMessageContent::Text(InputMessageContentText::new("Вот список:")),
    )
    .description("Узнай все доступные команды")
    .thumbnail_url(
        "https://thumbs.dreamstime.com/z/lot-pigs-d-rendered-illustration-127843482.jpg"
            .parse()
            .unwrap(),
    );

    let results: Vec<InlineQueryResult> = vec![InlineQueryResult::Article(help)]; 
    let response = bot.answer_inline_query(&q.id, results).send().await?; // Showing all suitable inline buttons

    Ok(())
}

pub async fn inline_get_hryak_size(bot: Bot, q: &InlineQuery, pool: &MySqlPool) -> Result<(), RequestError> {
    let mass = get_pig_weight(pool, q.from.id.0).await.unwrap();
    // TODO: Better error handling

    println!("done that");
    let hrundel_weight = InlineQueryResultArticle::new(
        "02".to_string(),
        "Узнать массу хряка".to_string(),
        InputMessageContent::Text(InputMessageContentText::new(
            mass.to_string()
        )),
    )
    .description("Hryak")
    .thumbnail_url("https://sputnik.kz/img/858/06/8580645_0:0:3117:2048_600x0_80_0_1_81d5b1f42e05e39353aa388a4e55cb34.jpg".parse().unwrap());

    let results: Vec<InlineQueryResult> = vec![InlineQueryResult::Article(hrundel_weight)];

    let response = bot.answer_inline_query(&q.id, results).send().await?;
    Ok(())
}
