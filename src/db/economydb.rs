use anyhow::anyhow;
use sqlx::{MySqlPool, Row};

const BASE_INCOME: f64 = 1000f64;

async fn add_money(pool: &MySqlPool, user_id: u64, money: f64) -> anyhow::Result<()> {
    sqlx::query("UPDATE TABLE bank SET balance = balance + ? WHERE user_id = ?")
        .bind(money)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

async fn sub_money(pool: &MySqlPool, user_id: u64, money: f64) -> anyhow::Result<()> {
    let balance = get_balance(pool, user_id).await?;

    if balance < money {
        return Err(anyhow!("not enough money"))
    }

    sqlx::query("UPDATE TABLE bank SET balance = balance - ? WHERE user_id = ?")
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

pub async fn daily_income(pool: &MySqlPool, user_id: u64) -> anyhow::Result<()> {
    sqlx::query("UPDATE TABLE bank SET balance = balance + ? * daily_income WHERE user_id = ?")
        .bind(BASE_INCOME)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}