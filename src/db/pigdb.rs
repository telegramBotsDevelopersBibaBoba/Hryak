use sqlx::{mysql::MySqlRow, MySqlPool, Row};
use anyhow::anyhow;
use crate::controllers::pig::Pig;

pub async fn create_pig(pool: &MySqlPool, user_id: u64) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO pigs (user_id) VALUES (?)")
    .bind(user_id)
    .execute(pool).await?;

    Ok(())
}

pub async fn get_pig_weight(pool: &MySqlPool, user_id: u64) -> anyhow::Result<f32> {
    let row = sqlx::query("SELECT weight FROM pigs WHERE user_id = ?")
    .bind(user_id)
    .fetch_one(pool).await?;
    
    row.try_get::<f32, _>(0).map_err(|why| anyhow!("{}", why))
}

pub async fn get_pig_by_user_id(pool: &MySqlPool, user_id: u64) -> anyhow::Result<Pig> {
    let row = sqlx::query("SELECT * FROM pigs WHERE user_id = ?")
    .bind(user_id)
    .fetch_one(pool).await?;

    Pig::from_mysql_row(row)
}
