use anyhow::anyhow;
use sqlx::MySqlPool;
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardMarkup, InlineQuery, InlineQueryResultArticle,
    InputMessageContent, InputMessageContentText,
};

use crate::db::{pigdb::get_pig_weight, userdb};
use crate::handlers::keyboard;

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
    .reply_markup(keyboard::make_more_info_keyboard()); // Showing a 'keyboard' with all the additional inline queries
    Ok(help)
}

pub async fn TEST_inline_shop_article(
    q: &InlineQuery,
    pool: &MySqlPool,
) -> anyhow::Result<InlineQueryResultArticle> {
    let shop = InlineQueryResultArticle::new(
        "03".to_string(),
        "Закупки".to_string(),
        InputMessageContent::Text(InputMessageContentText::new("Покупай:")),
    )
    .description("Шоп")
    .thumbnail_url(
        "https://mr-7.ru/static/previews/2010/09/30/khriushi-boriutsia-so-svinstvom-magazinov.jpeg?v=1"
            .parse()
            .unwrap(),
    )
    .reply_markup(keyboard::TEST_make_shop()); // Showing a 'keyboard' with all the additional inline queries
    Ok(shop)
}
