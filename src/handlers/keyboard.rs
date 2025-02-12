use sqlx::MySqlPool;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use crate::controllers::shop::OfferType;
use crate::db::shopdb;

use crate::ser_command;

pub fn make_more_info_keyboard() -> InlineKeyboardMarkup {
    let button = InlineKeyboardButton::switch_inline_query_current_chat("Узнать про хряка", "хряк");
    let button2 =
        InlineKeyboardButton::switch_inline_query_current_chat("Открыть магазин", "магазин");
    let button3 =
        InlineKeyboardButton::switch_inline_query_current_chat("Сменить имя хряка", "имя");
    // todo more

    InlineKeyboardMarkup::new([[button, button2, button3]])
}

pub async fn make_shop(shop_items_indexes: &[(u64, OfferType)], pool: &MySqlPool) -> anyhow::Result<(InlineKeyboardMarkup, String)> {
    // Make different buttons
    let mut buttons = Vec::new();
    let mut text = String::new();

    for (i, (item_id, offer_type)) in shop_items_indexes.iter().enumerate() {
        let item = shopdb::get_offer(pool, *offer_type, *item_id).await?;
        buttons.push(vec![item.get_button(i + 1)]);
        text.push_str(&item.get_info(i + 1));
    }

    Ok((InlineKeyboardMarkup::new(buttons), text))
}

pub fn make_duel(duel_maker_id: u64, duel_maker_mention: String) -> InlineKeyboardMarkup {
    let buttons = vec![
        // Perhaps  should store duel sender id
        InlineKeyboardButton::callback(
            "Начать дуэль",
            ser_command!(
                "duel",
                &duel_maker_id.to_string(),
                &duel_maker_mention.to_string()
            ),
        ),
    ];

    InlineKeyboardMarkup::new([buttons])
}
