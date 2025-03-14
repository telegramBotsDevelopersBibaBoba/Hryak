use anyhow::anyhow;
use sqlx::mysql::MySqlRow;
use sqlx::{MySqlPool, Row};

use crate::db::{dueldb, inventorydb};
use crate::db::shopdb::get_usages_buff;
use crate::StoragePool;

pub struct InventorySlot {
    pub id: i64,
    pub item_id: i64,
    pub title: String,
    pub buff_type: String,
    pub strength: f64,
    pub usages: i32,
}

impl InventorySlot {
    pub fn from_mysql_row(row: MySqlRow) -> anyhow::Result<Self> {
        let id: i64 = row.try_get("id")?;
        let item_id: i64 = row.try_get("item_id")?;
        let title: String = row.try_get("title")?;
        let buff_type: String = row.try_get("type")?;
        let strength: f64 = row.try_get("strength")?;
        let usages: i32 = row.try_get("usages")?;

        Ok(Self {
            id,
            item_id,
            title,
            buff_type,
            strength,
            usages,
        })
    }
}

pub async fn add_item(pool: &StoragePool, item_id: u64, user_id: u64) -> anyhow::Result<()> {
    let usages = get_usages_buff(pool, item_id).await?;
    if inventorydb::item_exists(pool, item_id, user_id).await {
        inventorydb::increase_item_usages(pool, item_id, user_id, usages).await?;
        return Ok(());
    }

    inventorydb::add_item(pool, item_id, user_id, usages).await?;
    Ok(())
}

pub async fn use_item(pool: &StoragePool, invslot_id: u64) -> anyhow::Result<(String, f64)> {
    let item = inventorydb::invslot(pool, invslot_id).await?;
    if item.usages <= 1 {
        inventorydb::delete_item(pool, invslot_id).await?;
    }
    inventorydb::decrease_item_usages(pool, invslot_id, 1).await?;

    Ok((item.buff_type, item.strength))
}
