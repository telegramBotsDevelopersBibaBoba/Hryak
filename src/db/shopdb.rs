use crate::controllers::shop::FoodOffer;
use sqlx::{MySqlPool, Row};

pub async fn get_food_offer_by_id(pool: &MySqlPool, offer_id: u64) -> anyhow::Result<FoodOffer> {
    let row = sqlx::query("SELECT * FROM shop_food WHERE id = ?")
        .bind(offer_id)
        .fetch_one(pool)
        .await?;

    Ok(FoodOffer::from_mysql_row(row)?)
}
