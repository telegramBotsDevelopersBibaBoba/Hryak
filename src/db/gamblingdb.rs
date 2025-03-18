use crate::StoragePool;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::Row;

pub async fn pigrace_played(pool: &StoragePool, user_id: u32) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO arcade (user_id, pigrace_last)
    VALUES (?, NOW())
    ON DUPLICATE KEY UPDATE pigrace_last = NOW();",
    )
    .bind(user_id)
    .execute(&pool.mysql_pool)
    .await?;

    Ok(())
}
pub async fn pigrace_last_time(
    pool: &StoragePool,
    user_id: u64,
) -> anyhow::Result<Option<DateTime<Utc>>> {
    let row = sqlx::query("SELECT pigrace_last FROM arcade WHERE user_id = ?")
        .bind(user_id)
        .fetch_optional(&pool.mysql_pool)
        .await?;

    if row.is_none() {
        return Ok(None);
    }

    let income_time = row.unwrap().try_get::<Option<DateTime<Utc>>, _>(0)?;
    Ok(income_time)
}

pub async fn treasurehunt_last_time(
    pool: &StoragePool,
    user_id: u64,
) -> anyhow::Result<Option<DateTime<Utc>>> {
    let row = sqlx::query("SELECT treasurehunt_last FROM arcade WHERE user_id = ?")
        .bind(user_id)
        .fetch_optional(&pool.mysql_pool)
        .await?;

    if row.is_none() {
        return Ok(None);
    }

    let income_time = row.unwrap().try_get::<Option<DateTime<Utc>>, _>(0)?;
    Ok(income_time)
}

pub async fn treasurehunt_played(pool: &StoragePool, user_id: u32) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO arcade (user_id, treasurehunt_last)
    VALUES (?, NOW())
    ON DUPLICATE KEY UPDATE treasurehunt_last = NOW();",
    )
    .bind(user_id)
    .execute(&pool.mysql_pool)
    .await?;

    Ok(())
}
