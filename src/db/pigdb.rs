use crate::controllers::pig::Pig;
use sqlx::types::BigDecimal;
use sqlx::MySqlPool;
use sqlx::Row;

pub async fn create_pig(pool: &MySqlPool, user_id: u64) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO pigs (user_id) VALUES (?)")
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn pig_exists(pool: &MySqlPool, user_id: u64) -> bool {
    match sqlx::query("SELECT * FROM pigs WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await
    {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn get_pig_weight(pool: &MySqlPool, user_id: u64) -> anyhow::Result<f32> {
    if !pig_exists(pool, user_id).await {
        create_pig(pool, user_id).await?;
    }

    let row = sqlx::query("SELECT weight FROM pigs WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await?; // If throws -> user does not exist

    let weight = row.try_get::<f32, _>(0)?;

    Ok(weight)
}

pub async fn get_pig_by_user_id(pool: &MySqlPool, user_id: u64) -> anyhow::Result<Pig> {
    let row = sqlx::query("SELECT * FROM pigs WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    Pig::from_mysql_row(row)
}

pub async fn set_pig_name(pool: &MySqlPool, name: &str, user_id: u64) -> anyhow::Result<()> {
    sqlx::query("UPDATE pigs SET name = ? WHERE user_id = ?")
        .bind(name)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}
