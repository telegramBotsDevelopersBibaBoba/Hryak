use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn make_more_info_keyboard() -> InlineKeyboardMarkup {
    let button =
        InlineKeyboardButton::switch_inline_query_current_chat("Узнать массу хряка", "хряк");
    let button2 =
        InlineKeyboardButton::switch_inline_query_current_chat("Открыть магазин", "магазин");
    let button3 =
        InlineKeyboardButton::switch_inline_query_current_chat("Сменить имя хряка", "имя");
    // todo more

    InlineKeyboardMarkup::new([[button, button2, button3]])
}

pub fn TEST_make_shop() -> InlineKeyboardMarkup {
    let buttons = vec![
        InlineKeyboardButton::callback("Buy some food", "shop food"),
        InlineKeyboardButton::callback("Suck a cock", "nothing"),
    ];

    InlineKeyboardMarkup::new([buttons])
}
