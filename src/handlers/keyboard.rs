use crate::controllers::duel::DuelActionType;
use crate::controllers::{duel::Duelist, shop::OfferType};
use crate::db::{inventorydb, shopdb};
use sqlx::MySqlPool;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::{ser_command, StoragePool};
pub fn make_more_info_keyboard() -> InlineKeyboardMarkup {
    let button = InlineKeyboardButton::switch_inline_query_current_chat("Узнать про хряка", "хряк");
    let button2 = InlineKeyboardButton::switch_inline_query_current_chat("Открыть магазин", "магазин");
    let button3 = InlineKeyboardButton::switch_inline_query_current_chat("Сменить имя хряка", "имя");
    let button4 = InlineKeyboardButton::switch_inline_query_current_chat("Своя ставка в дуэли", "дуэль 999");

    InlineKeyboardMarkup::new([
        [button],
        [button2],
        [button3],
        [button4],
    ])
}

pub async fn make_shop(
    shop_items_indexes: &Vec<(u64, OfferType)>,
    pool: &StoragePool,
) -> anyhow::Result<(InlineKeyboardMarkup, String)> {
    // Make different buttons
    let mut buttons = Vec::new();
    let mut text = String::new();

    for (i, (item_id, offer_type)) in shop_items_indexes.iter().enumerate() {
        let item = shopdb::offer(pool, *offer_type, *item_id).await?;
        buttons.push(vec![item.get_button(i + 1)]);
        text.push_str(&item.get_info(i + 1));
    }

    Ok((InlineKeyboardMarkup::new(buttons), text))
}

pub fn make_duel(duel_maker_id: u64, duel_maker_mention: String, bid: f64) -> InlineKeyboardMarkup {
    let buttons = vec![
        // Perhaps  should store duel sender id
        InlineKeyboardButton::callback(
            "Начать дуэль",
            ser_command!(
                "duel",
                &duel_maker_id.to_string(),
                &duel_maker_mention.to_string(),
                &bid.to_string()
            ),
        ),
    ];

    InlineKeyboardMarkup::new([buttons])
}

pub async fn make_duel_action(pool: &StoragePool, host_id: u64, part_id: u64, duelist: Duelist) -> InlineKeyboardMarkup {
    println!("here");
    
    let mut rows = vec![
        vec![
            InlineKeyboardButton::callback(
                "Атаковать",
                ser_command!(
                    "action",
                    &host_id.to_string(),
                    &DuelActionType::Attack.to_string(),
                    &duelist.to_string()
                ),
            ),
            InlineKeyboardButton::callback(
                "Защищаться",
                ser_command!(
                    "action",
                    &host_id.to_string(),
                    &DuelActionType::Defense.to_string(),
                    &duelist.to_string()
                ),
            ),
        ],
    ];

    let user_id = match duelist {
        Duelist::Host => host_id,
        Duelist::Part => part_id,
    };

    let invslots = inventorydb::invslots(pool, user_id).await.unwrap();

    for invslot in invslots {
        println!("{}", invslot.id);
        let button = InlineKeyboardButton::callback(
            invslot.title,
            ser_command!("buff", &host_id.to_string(), &user_id.to_string(), &invslot.id.to_string())
        );
        rows.push(vec![button]);
    }

    println!("here 2");
    // Create the InlineKeyboardMarkup with the rows
    InlineKeyboardMarkup::new(rows)
}

