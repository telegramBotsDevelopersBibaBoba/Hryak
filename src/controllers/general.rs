use sqlx::MySqlPool;
use teloxide::{
    prelude::Requester,
    types::{InlineQuery, Message, Update},
    Bot,
};

use crate::{config::utils, db::userdb, handlers::articles};

use super::user;

pub async fn handle_message(bot: Bot, msg: Message, pool: MySqlPool) -> bool {
    if msg.from.as_ref().unwrap().username.is_none() {
        utils::send_msg(
            &bot,
            &msg,
            "Бот не работает с пользователями без username.\nДобавьте его в настройках аккаунта",
        )
        .await
        .unwrap();
        return false;
    }
    let user_id = msg.from.as_ref().unwrap().id.0;
    let username = msg.from.as_ref().unwrap().username.as_ref().unwrap();

    if !userdb::exists(&pool, user_id).await {
        user::create_user(&pool, user_id, "None").await.unwrap();
    }
    userdb::set_username(&pool, &username, user_id)
        .await
        .unwrap();
    true
}

pub async fn handle_other(_: Bot, update: Update, pool: MySqlPool) -> bool {
    if update.from().as_ref().unwrap().username.is_none() {
        println!("user doesnt have a nickname");
        return false;
    }
    let user_id = update.from().as_ref().unwrap().id.0;
    let username = update.from().as_ref().unwrap().username.as_ref().unwrap();

    if !userdb::exists(&pool, user_id).await {
        user::create_user(&pool, user_id, "None").await.unwrap()
    }
    userdb::set_username(&pool, &username, user_id)
        .await
        .unwrap();
    true
}

pub async fn handle_inline(bot: Bot, q: InlineQuery, pool: MySqlPool) -> bool {
    if q.from.username.is_none() {
        let article = articles::make_article(
            "no_username",
            "Ошибка!",
            "Добавить имя пользователя можно в настройках аккаунта",
            "У вас нет имени пользователя! Добавьте в настройках",
            "https://i.ytimg.com/vi/dT_brFfiHHA/maxresdefault.jpg".into(),
        );
        bot.answer_inline_query(q.id, [article.into()])
            .await
            .unwrap();
        return false;
    }
    let user_id = q.from.id.0;
    let username = q.from.username.as_ref().unwrap();

    if !userdb::exists(&pool, user_id).await {
        user::create_user(&pool, user_id, "None").await.unwrap();
    }
    userdb::set_username(&pool, &username, user_id)
        .await
        .unwrap();
    true
}
