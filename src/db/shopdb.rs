use crate::controllers::shop::{BuffOffer, FoodOffer, ImprovementOffer, Offer, OfferType};
use sqlx::{MySqlPool, Row};

pub async fn get_food_offer_by_id(pool: &MySqlPool, offer_id: u64) -> anyhow::Result<FoodOffer> {
    let row = sqlx::query("SELECT * FROM shop_food WHERE id = ?")
        .bind(offer_id)
        .fetch_one(pool)
        .await?;

    Ok(FoodOffer::from_mysql_row(row)?)
}

pub async fn get_improvement_offer_by_id(
    pool: &MySqlPool,
    improvement_id: u64,
) -> anyhow::Result<ImprovementOffer> {
    let row = sqlx::query("SELECT * FROM shop_improvements WHERE id = ?")
        .bind(improvement_id)
        .fetch_one(pool)
        .await?;

    Ok(ImprovementOffer::from_mysql_row(row)?)
}

pub async fn get_buff_offer_by_id(pool: &MySqlPool, buff_id: u64) -> anyhow::Result<BuffOffer> {
    let row = sqlx::query("SELECT * FROM shop_buffs WHERE id = ?")
        .bind(buff_id)
        .fetch_one(pool)
        .await?;
    Ok(BuffOffer::from_mysql_row(row)?)
}

pub async fn get_offer(
    pool: &MySqlPool,
    offer_type: OfferType,
    item_id: u64,
) -> anyhow::Result<Offer> {
    Ok(match offer_type {
        OfferType::Food => Offer::Food(get_food_offer_by_id(pool, item_id).await?),
        OfferType::Improvement => {
            Offer::Improvement(get_improvement_offer_by_id(pool, item_id).await?)
        }
        OfferType::Buff => Offer::Buff(get_buff_offer_by_id(pool, item_id).await?),
    })
}
pub async fn get_usages_buff(pool: &MySqlPool, item_id: u64) -> anyhow::Result<i32> {
    let row = sqlx::query("SELECT usages FROM shop_buffs WHERE id = ?")
        .bind(item_id)
        .fetch_one(pool)
        .await?;
    let usages = row.try_get::<i32, _>(0)?;
    Ok(usages)
}
