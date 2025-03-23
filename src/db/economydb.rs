use anyhow::anyhow;
use redis::Commands;
use sqlx::{
    types::chrono::{DateTime, Utc},
    Row,
};

use crate::StoragePool;

pub async fn create_bank_account(pool: &StoragePool, user_id: u64) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO bank (user_id) VALUES (?)")
        .bind(user_id)
        .execute(&pool.mysql_pool)
        .await?;

    let mut redis_con = pool.redis_pool.get()?;
    let _: () = redis_con.hset_multiple(
        format!("bank:{}", user_id),
        &[("balance", 10.0), ("daily_income", 10.0)],
    )?;

    Ok(())
}

pub async fn add_money(pool: &StoragePool, user_id: u64, money: f64) -> anyhow::Result<()> {
    sqlx::query("UPDATE bank SET balance = balance + ? WHERE user_id = ?")
        .bind(money)
        .bind(user_id)
        .execute(&pool.mysql_pool)
        .await?;

    let mut redis_con = pool.redis_pool.get()?;
    let balance: f64 = redis_con.hget(format!("bank:{}", user_id), "balance")?;

    let _: () = redis_con.hset(
        format!("bank:{}", user_id),
        "balance",
        ((balance + money) * 100.0).floor() / 100.0,
    )?;

    Ok(())
}

pub async fn sub_money(pool: &StoragePool, user_id: u64, money: f64) -> anyhow::Result<()> {
    let balance = balance(pool, user_id).await?;
    println!("{balance}, {}", money);

    if balance < money {
        return Err(anyhow!("Недостаточно денег"));
    }

    sqlx::query("UPDATE bank SET balance = balance - ? WHERE user_id = ?")
        .bind(money)
        .bind(user_id)
        .execute(&pool.mysql_pool)
        .await?;

    let mut redis_con = pool.redis_pool.get()?;
    let balance: f64 = redis_con.hget(format!("bank:{}", user_id), "balance")?;

    let _: () = redis_con.hset(
        format!("bank:{}", user_id),
        "balance",
        ((balance - money) * 100.0).floor() / 100.0,
    )?;

    Ok(())
}

pub async fn balance(pool: &StoragePool, user_id: u64) -> anyhow::Result<f64> {
    let mut redis_con = pool.redis_pool.get()?;
    if let Ok(val) =
        redis_con.hget::<_, String, f64>(format!("bank:{}", user_id), "balance".to_string())
    {
        return Ok(val);
    }
    let row = sqlx::query("SELECT balance FROM bank WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(&pool.mysql_pool)
        .await?;
    let bal = row.try_get::<f64, _>(0).map_err(|why| anyhow!("{}", why))?;
    let _: () = redis_con.hset(format!("bank:{}", user_id), "balance", bal)?;
    Ok(bal)
}

pub async fn daily_income(pool: &StoragePool, user_id: u64) -> anyhow::Result<f64> {
    let mut redis_con = pool.redis_pool.get()?;

    if let Ok(val) =
        redis_con.hget::<_, _, f64>(format!("bank:{}", user_id), "daily_income".to_string())
    {
        return Ok(val);
    }

    let row = sqlx::query("SELECT daily_income FROM bank WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(&pool.mysql_pool)
        .await?;
    let daily_income = row.try_get::<f64, _>(0)?;

    let _: () = redis_con.hset(format!("bank:{}", user_id), "daily_income", daily_income)?;
    Ok(daily_income)
}

pub async fn income_time(
    pool: &StoragePool,
    user_id: u64,
) -> anyhow::Result<Option<DateTime<Utc>>> {
    let row = sqlx::query("SELECT income_time FROM bank WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(&pool.mysql_pool)
        .await?;

    let income_time = row.try_get::<Option<DateTime<Utc>>, _>(0)?;
    Ok(income_time)
}

pub async fn do_daily_income(pool: &StoragePool, user_id: u64) -> anyhow::Result<()> {
    let income_time = income_time(pool, user_id).await?;
    if income_time.is_some() && (Utc::now() - income_time.unwrap()).num_hours() < 16 {
        return Err(anyhow!("Рано"));
    }

    sqlx::query(
        "UPDATE bank SET balance = balance + daily_income, income_time = NOW() WHERE user_id = ?",
    )
    .bind(user_id)
    .execute(&pool.mysql_pool)
    .await?;

    let mut redis_con = pool.redis_pool.get()?;
    let _: () = redis_con.hdel(format!("bank:{}", user_id), "balance")?;
    Ok(())
}

pub async fn increase_daily_income(
    pool: &StoragePool,
    user_id: u64,
    add_income: f64,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE bank SET daily_income = daily_income + ? WHERE user_id = ?")
        .bind(add_income)
        .bind(user_id)
        .execute(&pool.mysql_pool)
        .await?;

    let mut redis_con = pool.redis_pool.get()?;
    let daily_income: f64 = redis_con.hget(format!("bank:{}", user_id), "daily_income")?;
    let _: () = redis_con.hset(
        format!("bank:{}", user_id),
        "daily_income",
        ((daily_income + add_income) * 100.0).floor() / 100.0,
    )?;
    Ok(())
}
