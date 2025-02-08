use anyhow::anyhow;
use sqlx::MySqlPool;
use teloxide::types::{
    InlineKeyboardButton, InlineKeyboardMarkup, InlineQuery, InlineQueryResultArticle,
    InputMessageContent, InputMessageContentText,
};

use crate::db::pigdb::get_pig_by_user_id;
use crate::db::{pigdb::get_pig_weight, userdb};
use crate::handlers::keyboard;

use super::keyboard::make_duel;

pub async fn inline_hryak_info_article(
    q: &InlineQuery,
    pool: &MySqlPool,
) -> anyhow::Result<InlineQueryResultArticle> {
    let pig = match get_pig_by_user_id(pool, q.from.id.0).await {
        Ok(mass) => mass,
        Err(why) => {
            eprintln!("{}", why);
            userdb::create_user(pool, q.from.id.0, &q.from.first_name).await?;

            let hrundel_weight = InlineQueryResultArticle::new(
                "hryak".to_string(),
                "Ваш первый хрюндель был создан".to_string(),
                InputMessageContent::Text(InputMessageContentText::new(
                    format!("Введите команду еще раз")
                )),
            )
            .description("Для корректного отображения введите команду еще раз")
            .thumbnail_url("https://sputnik.kz/img/858/06/8580645_0:0:3117:2048_600x0_80_0_1_81d5b1f42e05e39353aa388a4e55cb34.jpg".parse().unwrap());

            return Ok(hrundel_weight);
        }
    };

    let hrundel_weight = InlineQueryResultArticle::new(
        "hryak".to_string(),
        "Узнать инфу о хряке".to_string(),
        InputMessageContent::Text(InputMessageContentText::new(
            format!("Имя хряка: {}\nРазмер хряка: {} кг.", pig.name, pig.weight)
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
        "help".to_string(),
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

pub async fn inline_shop_article(
    q: &InlineQuery,
    pool: &MySqlPool,
) -> anyhow::Result<InlineQueryResultArticle> {
    let shop = InlineQueryResultArticle::new(
        "shop".to_string(),
        "Закупки".to_string(),
        InputMessageContent::Text(InputMessageContentText::new("Покупай:")),
    )
    .description("Шоп")
    .thumbnail_url(
        "https://mr-7.ru/static/previews/2010/09/30/khriushi-boriutsia-so-svinstvom-magazinov.jpeg?v=1"
            .parse()
            .unwrap(),
    )
    .reply_markup(keyboard::make_shop()); // Showing a 'keyboard' with all the additional inline queries
    Ok(shop)
}

pub async fn inline_name_article() -> anyhow::Result<InlineQueryResultArticle> {
    let name = InlineQueryResultArticle::new(
        "name",
        "Поменять имя у хряка",
        InputMessageContent::Text(InputMessageContentText::new(
            "Чтобы сменить имя, нужно ввести 'имя новое_имя'",
        )),
    )
    .description("Введите пробел и имя")
    .thumbnail_url(
        "https://www.lifewithpigs.com/uploads/7/7/7/1/77712458/published/luckpig.png?1518827974"
            .parse()
            .unwrap(),
    );
    Ok(name)
}

pub async fn inline_change_name_article(
    new_name: &str,
) -> anyhow::Result<InlineQueryResultArticle> {
    let name = InlineQueryResultArticle::new(
        "change_name",
        "Меняем имя у хряка...",
        InputMessageContent::Text(InputMessageContentText::new(
            format!("Имя хрюнделя было изменено на {}", new_name)
        )),
    )
    .description("Нажмите на кнопку, чтобы сменить имя")
    .thumbnail_url(
        "https://media.licdn.com/dms/image/v2/C4E12AQHOTlp8TuFzxg/article-inline_image-shrink_1000_1488/article-inline_image-shrink_1000_1488/0/1520148182297?e=1743033600&v=beta&t=3zE1S7YVIL8QQ7JCyuSvy6Flj9Bm_27l6mRLJmU3Lzo"
            .parse()
            .unwrap(),
    );
    Ok(name)
}

pub async fn inline_duel_article(
    duel_host_id: u64,
    duel_host_mention: String,
) -> anyhow::Result<InlineQueryResultArticle> {
    let name = InlineQueryResultArticle::new(
        "duel",
        "Нажмите, чтобы выслать приглашение на дуэль",
        InputMessageContent::Text(InputMessageContentText::new(
            format!("Нажмите на кнопку, чтобы начать дуэль!")
        )),
    )
    .description("Свинодуэль")
    .thumbnail_url(
        "https://avatars.mds.yandex.net/get-shedevrum/11552302/b56a5e87c2af11ee8ba7be62f04505c7/orig"
            .parse()
            .unwrap(),
    )
    .reply_markup(make_duel(duel_host_id, duel_host_mention));
    Ok(name)
}
