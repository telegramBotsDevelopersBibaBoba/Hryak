use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

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
        InlineKeyboardButton::callback("Купить боярышник х1", "shop food boyarishnik"),
        InlineKeyboardButton::callback("Купить ничего", "shop food nothing"),
    ];

    InlineKeyboardMarkup::new([buttons])
}

pub fn make_duel(/* args list */) -> InlineKeyboardMarkup {
    let buttons = vec![
        // Perhaps  should store duel sender id
        InlineKeyboardButton::callback("Начать дуэль", "duel this_id that_id"),
    ];

    InlineKeyboardMarkup::new([buttons])
}
