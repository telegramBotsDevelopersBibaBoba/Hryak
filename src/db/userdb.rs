use redis::Commands;
use sqlx::Row;

use crate::StoragePool;

pub async fn create_user(pool: &StoragePool, user_id: u64, username: &str) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO users (id, username) VALUES (?, ?)")
        .bind(user_id)
        .bind(username)
        .execute(&pool.mysql_pool)
        .await?;

    let mut redis_con = pool.redis_pool.get()?;
    let _: () = redis_con.set(format!("user:{}", user_id), username)?;

    Ok(())
}

pub async fn exists(pool: &StoragePool, user_id: u64) -> bool {
    match sqlx::query("SELECT id FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&pool.mysql_pool)
        .await
    {
        // If the query fails => nothing found
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn username(pool: &StoragePool, user_id: u64) -> anyhow::Result<String> {
    let mut redis_con = pool.redis_pool.get()?;

    if let Ok(val) = redis_con.get::<_, String>(format!("user:{}", user_id)) {
        println!("username from cache");
        return Ok(val);
    }
    // If it isnt in cache
    let row = sqlx::query("SELECT username FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&pool.mysql_pool)
        .await?;
    let username = row.try_get::<String, _>(0)?; // Get from db

    // And cache it
    let _: () = redis_con.set(format!("user:{}", user_id), &username)?;
    Ok(username) // then return
}

pub async fn set_username(pool: &StoragePool, username: &str, user_id: u64) -> anyhow::Result<()> {
    sqlx::query("UPDATE users SET username = ? WHERE id = ?")
        .bind(username)
        .bind(user_id)
        .execute(&pool.mysql_pool)
        .await?;

    let mut redis_con = pool.redis_pool.get()?;
    let _: () = redis_con.set(format!("user:{}", user_id), &username)?;
    Ok(())
}

pub async fn id(pool: &StoragePool, username: &str) -> anyhow::Result<i64> {
    let row = sqlx::query("SELECT id FROM users WHERE username = ?")
        .bind(username)
        .fetch_one(&pool.mysql_pool)
        .await?;

    let id = row.try_get::<i64, _>(0)?;
    Ok(id)
}
