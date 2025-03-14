use crate::{controllers::inventory::InventorySlot, StoragePool};

pub async fn invslot(
    pool: &StoragePool,
    invslot_id: u64
) -> anyhow::Result<InventorySlot> {
    println!("invslot id {}", invslot_id);
    let row = sqlx::query("SELECT * FROM inventory_slot_view WHERE id = ?")
        .bind(invslot_id)
        .fetch_one(&pool.mysql_pool)
        .await?;
    Ok(InventorySlot::from_mysql_row(row)?)
}

pub async fn invslots(
    pool: &StoragePool,
    user_id: u64,
) -> anyhow::Result<Vec<InventorySlot>> {
    let rows = sqlx::query("SELECT * FROM inventory_slot_view WHERE user_id = ? LIMIT 5")
        .bind(user_id)
        .fetch_all(&pool.mysql_pool)
        .await?;

    let mut slots = Vec::new();
    for row in rows {
        slots.push(InventorySlot::from_mysql_row(row)?);
    }

    Ok(slots)
}


pub async fn item_exists(pool: &StoragePool, item_id: u64, user_id: u64) -> bool {
    match sqlx::query("SELECT * FROM inventory WHERE user_id = ? AND item_id = ?")
        .bind(user_id)
        .bind(item_id)
        .fetch_one(&pool.mysql_pool)
        .await
    {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn increase_item_usages(
    pool: &StoragePool,
    item_id: u64,
    user_id: u64,
    add_usages: i32,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE inventory SET usages = usages + ? WHERE user_id = ? AND item_id = ?")
        .bind(add_usages)
        .bind(user_id)
        .bind(item_id)
        .execute(&pool.mysql_pool)
        .await?;
    Ok(())
}

pub async fn decrease_item_usages(
    pool: &StoragePool,
    invslot_id: u64,
    rem_usages: i32,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE inventory SET usages = usages - ? WHERE id = ?")
        .bind(rem_usages)
        .bind(invslot_id)
        .execute(&pool.mysql_pool)
        .await?;
    Ok(())
}



pub async fn set_item_usages(
    pool: &StoragePool,
    item_id: i64,
    user_id: i64,
    usages: i32,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE inventory SET usages = ? WHERE user_id = ? AND item_id = ?")
        .bind(usages)
        .bind(user_id)
        .bind(item_id)
        .execute(&pool.mysql_pool)
        .await?;
    Ok(())
}

pub async fn add_item(
    pool: &StoragePool,
    item_id: u64,
    user_id: u64,
    usages: i32,
) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO inventory (user_id, item_id, usages) VALUES (?, ?, ?)")
        .bind(user_id)
        .bind(item_id)
        .bind(usages)
        .execute(&pool.mysql_pool)
        .await?;
    Ok(())
}

pub async fn delete_item(
    pool: &StoragePool,
    invslot_id: u64,
) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM inventory WHERE id = ?")
        .bind(invslot_id)
        .execute(&pool.mysql_pool)
        .await?;
    Ok(())
}



