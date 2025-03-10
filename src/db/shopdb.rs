use crate::{
    controllers::shop::{BuffOffer, FoodOffer, ImprovementOffer, Offer, OfferType},
    StoragePool,
};
use sqlx::Row;

pub async fn food_offer(pool: &StoragePool, offer_id: u64) -> anyhow::Result<FoodOffer> {
    let row = sqlx::query("SELECT * FROM shop_food WHERE id = ?")
        .bind(offer_id)
        .fetch_one(&pool.mysql_pool)
        .await?;

    Ok(FoodOffer::from_mysql_row(row)?)
}

pub async fn improvement_offer(
    pool: &StoragePool,
    improvement_id: u64,
) -> anyhow::Result<ImprovementOffer> {
    let row = sqlx::query("SELECT * FROM shop_improvements WHERE id = ?")
        .bind(improvement_id)
        .fetch_one(&pool.mysql_pool)
        .await?;

    Ok(ImprovementOffer::from_mysql_row(row)?)
}

pub async fn buff_offer(pool: &StoragePool, buff_id: u64) -> anyhow::Result<BuffOffer> {
    let row = sqlx::query("SELECT * FROM shop_buffs WHERE id = ?")
        .bind(buff_id)
        .fetch_one(&pool.mysql_pool)
        .await?;
    Ok(BuffOffer::from_mysql_row(row)?)
}

pub async fn offer(
    pool: &StoragePool,
    offer_type: OfferType,
    item_id: u64,
) -> anyhow::Result<Offer> {
    Ok(match offer_type {
        OfferType::Food => Offer::Food(food_offer(pool, item_id).await?),
        OfferType::Improvement => Offer::Improvement(improvement_offer(pool, item_id).await?),
        OfferType::Buff => Offer::Buff(buff_offer(pool, item_id).await?),
    })
}
pub async fn get_usages_buff(pool: &StoragePool, item_id: u64) -> anyhow::Result<i32> {
    let row = sqlx::query("SELECT usages FROM shop_buffs WHERE id = ?")
        .bind(item_id)
        .fetch_one(&pool.mysql_pool)
        .await?;
    let usages = row.try_get::<i32, _>(0)?;
    Ok(usages)
}
