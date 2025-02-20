use sqlx::mysql::MySqlRow;
use sqlx::Row;

pub struct InventorySlot {
    id: i64,
    item_id: i64,
    title: String,
    buff_type: String,
    strength: f64,
    usages: i32,
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
