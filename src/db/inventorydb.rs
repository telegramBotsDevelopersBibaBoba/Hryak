use sqlx::MySqlPool;

use crate::controllers::inventory::InventorySlot;

pub async fn invslot(
    pool: &MySqlPool,
    buff_id: u64,
    user_id: u64,
) -> anyhow::Result<InventorySlot> {
    let row = sqlx::query("SELECT * FROM inventory_slot_view WHERE user_id = ? AND item_id = ?")
        .bind(user_id)
        .bind(buff_id)
        .fetch_one(pool)
        .await?;
    Ok(InventorySlot::from_mysql_row(row)?)
}

pub async fn item_exists(pool: &MySqlPool, item_id: u64, user_id: u64) -> bool {
    match sqlx::query("SELECT * FROM inventory WHERE user_id = ? AND item_id = ?")
        .bind(user_id)
        .bind(item_id)
        .fetch_one(pool)
        .await
    {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn increase_item_usages(
    pool: &MySqlPool,
    item_id: u64,
    user_id: u64,
    add_usages: i32,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE inventory SET usages = usages + ? WHERE user_id = ? AND item_id = ?")
        .bind(add_usages)
        .bind(user_id)
        .bind(item_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_item_usages(
    pool: &MySqlPool,
    item_id: i64,
    user_id: i64,
    usages: i32,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE inventory SET usages = ? WHERE user_id = ? AND item_id = ?")
        .bind(usages)
        .bind(user_id)
        .bind(item_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn add_item(
    pool: &MySqlPool,
    item_id: u64,
    user_id: u64,
    usages: i32,
) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO inventory (user_id, item_id, usages) VALUES (?, ?, ?)")
        .bind(user_id)
        .bind(item_id)
        .bind(usages)
        .execute(pool)
        .await?;
    Ok(())
}
