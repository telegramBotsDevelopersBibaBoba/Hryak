use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

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

pub fn make_shop() -> InlineKeyboardMarkup {
    // Make different buttons
    let buttons = vec![
        InlineKeyboardButton::callback(
            "Купить боярышник х1",
            ser_command!("shop", "food", "boyarishnik"),
        ),
        InlineKeyboardButton::callback("Купить ничего", ser_command!("shop", "food", "nothing")),
    ];

    InlineKeyboardMarkup::new([buttons])
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
