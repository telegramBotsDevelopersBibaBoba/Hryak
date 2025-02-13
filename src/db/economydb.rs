use anyhow::anyhow;
use sqlx::{MySqlPool, Row};

use crate::controllers::shop::{OfferType, Offer};
use super::shopdb;

const BASE_INCOME: f64 = 1000f64;

async fn add_money(pool: &MySqlPool, user_id: u64, money: f64) -> anyhow::Result<()> {
    sqlx::query("UPDATE bank SET balance = balance + ? WHERE user_id = ?")
        .bind(money)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

async fn sub_money(pool: &MySqlPool, user_id: u64, money: f64) -> anyhow::Result<()> {
    let balance = get_balance(pool, user_id).await?;
    println!("{balance}, {}", money);

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

pub async fn daily_income(pool: &MySqlPool, user_id: u64) -> anyhow::Result<()> {
    sqlx::query("UPDATE bank SET balance = balance + ? * daily_income WHERE user_id = ?")
        .bind(BASE_INCOME)
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn try_to_buy(pool: &MySqlPool, user_id: u64, offer_id: u64, offer_type: OfferType) -> anyhow::Result<Offer> {
    let offer = match offer_type {
        OfferType::Food => Offer::Food(shopdb::get_food_offer_by_id(pool, offer_id).await?),
        OfferType::Improvement => Offer::Improvement(shopdb::get_improvement_offer_by_id(pool, offer_id).await?),
    };

    match sub_money(pool, user_id, offer.get_price()).await {
        Ok(_) => Ok(offer),
        Err(why) => {
            println!("{why}");
            Err(anyhow!("Not enough money"))
        }
    }
}
