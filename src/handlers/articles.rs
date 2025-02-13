use sqlx::MySqlPool;
use teloxide::types::{
    InlineQuery, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
};

use crate::controllers::pig::{self, Pig};
use crate::db::{economydb, userdb};
use crate::handlers::keyboard;

use super::keyboard::make_duel;

pub async fn inline_hryak_info_article(
    pool: &MySqlPool,
    username: &Option<String>,
    user_id: u64,
) -> anyhow::Result<InlineQueryResultArticle> {
    let pig = pig::get_pig(pool, user_id).await?;

    let hrundel_weight = make_article("hryak", "Узнать инфу о хряке",
        &format!("Имя хряка: {}\nРазмер хряка: {} кг.", pig.name, pig.weight),
        "Посмотрите подробную информацию о вашей свинке",
        "https://sputnik.kz/img/858/06/8580645_0:0:3117:2048_600x0_80_0_1_81d5b1f42e05e39353aa388a4e55cb34.jpg".into());

    Ok(hrundel_weight)
}
pub fn inline_help_article() -> InlineQueryResultArticle {
    let help = InlineQueryResultArticle::new(
        "help".to_string(),
        "Помощь".to_string(),
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
    help
}

pub fn inline_shop_article() -> InlineQueryResultArticle {
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
    shop
}

pub fn inline_name_article() -> InlineQueryResultArticle {
    make_article(
        "name",
        "Поменять имя у хряка",
        "Чтобы сменить имя, нужно ввести 'имя новое_имя'",
        "Введите пробел и имя",
        "https://www.lifewithpigs.com/uploads/7/7/7/1/77712458/published/luckpig.png?1518827974"
            .into(),
    )
}

pub fn inline_change_name_article(new_name: &str) -> InlineQueryResultArticle {
    let new_name = if new_name.is_empty() {
        "Unnamed"
    } else {
        new_name
    };
    make_article(
        "change_name",
        "Меняем имя у хряка...",
        &format!("Имя хрюнделя было изменено на {}", new_name),
        "Нажмите на кнопку, чтобы сменить имя",
        "https://media.licdn.com/dms/image/v2/C4E12AQHOTlp8TuFzxg/article-inline_image-shrink_1000_1488/article-inline_image-shrink_1000_1488/0/1520148182297?e=1743033600&v=beta&t=3zE1S7YVIL8QQ7JCyuSvy6Flj9Bm_27l6mRLJmU3Lzo".into(),
    )
}

pub async fn inline_duel_article(
    pool: &MySqlPool,
    duel_host_id: u64,
    duel_host_mention: String,
    bid: f64,
) -> anyhow::Result<InlineQueryResultArticle> {
    let user_balance = economydb::get_balance(pool, duel_host_id).await?;
    if user_balance < bid {
        let message = "Недостаточно денег для создания дуэли!";

        let n_money = make_article("not_enough_money",
            "Ошибка!",
            "Недостаточно денег для создания дуэли!",
            "Недостаточно денег для создания дуэли!",
            "https://avatars.mds.yandex.net/get-shedevrum/11552302/b56a5e87c2af11ee8ba7be62f04505c7/orig".into());

        return Ok(n_money);
    }

    let name = InlineQueryResultArticle::new(
        "duel",
        "Дуэль",
        InputMessageContent::Text(InputMessageContentText::new(
            format!("Нажмите на кнопку, чтобы начать дуэль!\nСтавка {}$", bid)
        )),
    )
    .description(format!("Свинодуэль. Ставка {}$", bid))
    .thumbnail_url(
        "https://avatars.mds.yandex.net/get-shedevrum/11552302/b56a5e87c2af11ee8ba7be62f04505c7/orig"
            .parse()
            .unwrap(),
    )
    .reply_markup(make_duel(duel_host_id, duel_host_mention, bid));
    Ok(name)
}

pub async fn inline_balance_article(
    pool: &MySqlPool,
    user_id: u64,
) -> anyhow::Result<InlineQueryResultArticle> {
    let balance = economydb::get_balance(pool, user_id).await?;

    let message = format!("Ваш баланс: {}$\nХохлы пидоры", balance);

    let balance_article = InlineQueryResultArticle::new(
        "balance",
        "Ваш баланс",
        InputMessageContent::Text(InputMessageContentText::new(message)),
    )
    .description("Нажмите сюда, чтобы увидеть ваш баланс")
    .thumbnail_url(
        "https://ih1.redbubble.net/image.5250551209.9937/flat,750x,075,f-pad,750x1000,f8f8f8.webp"
            .parse()
            .unwrap(),
    );

    Ok(balance_article)
}

#[inline]
pub fn make_article(
    id: &str,
    title: &str,
    content: &str,
    description: &str,
    url: Option<&str>,
) -> InlineQueryResultArticle {
    InlineQueryResultArticle::new(
        id,
        title,
        InputMessageContent::Text(InputMessageContentText::new(content)),
    )
    .description(description)
    .thumbnail_url(url.unwrap_or("https://media.istockphoto.com/id/956025942/photo/newborn-piglet-on-spring-green-grass-on-a-farm.jpg?s=612x612&w=0&k=20&c=H01c3cbV4jozkEHvyathjQL1DtKx6mOd5s7NwACUJwA=").parse().unwrap())
}
