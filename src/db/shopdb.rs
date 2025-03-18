use crate::{
    controllers::shop::{
        BuffOffer, FoodOffer, ImprovementOffer, Offer, OfferType, BUFF_OFFERS, FOOD_OFFERS,
        IMPROVEMENT_OFFERS,
    },
    StoragePool,
};
use sqlx::Row;
pub async fn food_offer(pool: &StoragePool, offer_id: u64) -> anyhow::Result<FoodOffer> {
    {
        let cache = FOOD_OFFERS
            .read()
            .map_err(|e| anyhow::anyhow!("RwLock read error: {}", e))?;

        if let Some(offer) = cache.get(&offer_id) {
            return Ok(offer.clone());
        }
    } // Drop read lock before acquiring write lock

    let row = sqlx::query("SELECT * FROM shop_food WHERE id = ?")
        .bind(offer_id)
        .fetch_one(&pool.mysql_pool)
        .await?;

    let off = FoodOffer::from_mysql_row(row)?;

    {
        let mut cache = FOOD_OFFERS
            .write()
            .map_err(|e| anyhow::anyhow!("RwLock write error: {}", e))?;
        cache.insert(offer_id, off.clone());
    }

    Ok(off)
}

pub async fn improvement_offer(
    pool: &StoragePool,
    improvement_id: u64,
) -> anyhow::Result<ImprovementOffer> {
    {
        let cache = IMPROVEMENT_OFFERS
            .read()
            .map_err(|e| anyhow::anyhow!("RwLock read error: {}", e))?;
        if let Some(offer) = cache.get(&improvement_id) {
            return Ok(offer.clone());
        }
    }

    let row = sqlx::query("SELECT * FROM shop_improvements WHERE id = ?")
        .bind(improvement_id)
        .fetch_one(&pool.mysql_pool)
        .await?;

    let off = ImprovementOffer::from_mysql_row(row)?;

    {
        let mut cache = IMPROVEMENT_OFFERS
            .write()
            .map_err(|e| anyhow::anyhow!("RwLock write error: {}", e))?;
        cache.insert(improvement_id, off.clone());
    }

    Ok(off)
}
pub async fn buff_offer(pool: &StoragePool, buff_id: u64) -> anyhow::Result<BuffOffer> {
    {
        let cache = BUFF_OFFERS
            .read()
            .map_err(|e| anyhow::anyhow!("RwLock read error: {}", e))?;

        if let Some(offer) = cache.get(&buff_id) {
            return Ok(offer.clone());
        }
    } // Drop read lock before acquiring write lock

    let row = sqlx::query("SELECT * FROM shop_buffs WHERE id = ?")
        .bind(buff_id)
        .fetch_one(&pool.mysql_pool)
        .await?;

    let off = BuffOffer::from_mysql_row(row)?;

    {
        let mut cache = BUFF_OFFERS
            .write()
            .map_err(|e| anyhow::anyhow!("RwLock write error: {}", e))?;
        cache.insert(buff_id, off.clone());
    }

    Ok(off)
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
pub async fn get_usages_buff(pool: &StoragePool, invslot_id: u64) -> anyhow::Result<i32> {
    let row = sqlx::query("SELECT usages FROM shop_buffs WHERE id = ?")
        .bind(invslot_id)
        .fetch_one(&pool.mysql_pool)
        .await?;
    let usages = row.try_get::<i32, _>(0)?;
    Ok(usages)
}
