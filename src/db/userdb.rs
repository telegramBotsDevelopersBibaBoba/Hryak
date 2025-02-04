use sqlx::MySqlPool;

use crate::db::pigdb;

pub async fn create_user(pool: &MySqlPool, user_id: u64, display_name: &str) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO users (id, display_name) VALUES (?, ?)")
        .bind(user_id)
        .bind(display_name)
        .execute(pool)
        .await?;

    pigdb::create_pig(pool, user_id).await?;

    Ok(())
}

pub async fn user_exists(pool: &MySqlPool, user_id: u64) -> bool {
    match sqlx::query("SELECT id FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await
    { // If the query fails => nothing found
        Ok(_) => true,
        Err(_) => false,
    }
}
