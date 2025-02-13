use anyhow::anyhow;
use sqlx::{
    types::chrono::{DateTime, Utc},
    MySqlPool, Row,
};

pub async fn create_bank_account(pool: &MySqlPool, user_id: u64) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO bank (user_id) VALUES (?)")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn add_money(pool: &MySqlPool, user_id: u64, money: f64) -> anyhow::Result<()> {
    sqlx::query("UPDATE bank SET balance = balance + ? WHERE user_id = ?")
        .bind(money)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn sub_money(pool: &MySqlPool, user_id: u64, money: f64) -> anyhow::Result<()> {
    let balance = get_balance(pool, user_id).await?;

    if balance < money {
        return Err(anyhow!("not enough money"));
    }

    sqlx::query("UPDATE bank SET balance = balance - ? WHERE user_id = ?")
        .bind(money)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_balance(pool: &MySqlPool, user_id: u64) -> anyhow::Result<f64> {
    let row = sqlx::query("SELECT balance FROM bank WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    row.try_get::<f64, _>(0).map_err(|why| anyhow!("{}", why))
}

pub async fn get_daily_income(
    pool: &MySqlPool,
    user_id: u64,
) -> anyhow::Result<(f64, Option<DateTime<Utc>>)> {
    let row = sqlx::query("SELECT daily_income, income_time FROM bank WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await?;
    let daily_income = row.try_get::<f64, _>(0)?;
    let income_time = row.try_get::<Option<DateTime<Utc>>, _>(1)?;
    Ok((daily_income, income_time))
}

pub async fn get_income_time(
    pool: &MySqlPool,
    user_id: u64,
) -> anyhow::Result<Option<DateTime<Utc>>> {
    let row = sqlx::query("SELECT income_time FROM bank WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    let income_time = row.try_get::<Option<DateTime<Utc>>, _>(0)?;
    Ok(income_time)
}

pub async fn do_daily_income(pool: &MySqlPool, user_id: u64) -> anyhow::Result<()> {
    let income_time = get_income_time(pool, user_id).await?;
    if income_time.is_some() && (Utc::now() - income_time.unwrap()).num_hours() < 24 {
        return Err(anyhow!("Рано"));
    }

    sqlx::query("UPDATE bank SET balance = balance + ? * daily_income, income_time = NOW() WHERE user_id = ?")
        .bind(crate::config::consts::BASE_INCOME)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}
