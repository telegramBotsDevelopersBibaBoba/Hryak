use sqlx::MySqlPool;
use sqlx::Row;

pub async fn create_user(pool: &MySqlPool, user_id: u64, username: &str) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO users (id, username) VALUES (?, ?)")
        .bind(user_id)
        .bind(username)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn exists(pool: &MySqlPool, user_id: u64) -> bool {
    match sqlx::query("SELECT id FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await
    {
        // If the query fails => nothing found
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn username(pool: &MySqlPool, user_id: u64) -> anyhow::Result<String> {
    let row = sqlx::query("SELECT username FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await?;
    let username = row.try_get::<String, _>(0)?;
    Ok(username)
}

pub async fn set_username(pool: &MySqlPool, username: &str, user_id: u64) -> anyhow::Result<()> {
    sqlx::query("UPDATE users SET username = ? WHERE id = ?")
        .bind(username)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn id(pool: &MySqlPool, username: &str) -> anyhow::Result<i64> {
    let row = sqlx::query("SELECT id FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(pool)
        .await?;

    let id = row.try_get::<i64, _>(0)?;
    Ok(id)
}
