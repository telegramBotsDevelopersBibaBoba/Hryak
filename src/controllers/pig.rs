use rand::Rng;
use sqlx::{mysql::MySqlRow, MySqlPool, Row};

use crate::db::{economydb, pigdb, userdb};

use super::user;

pub struct Pig {
    id: i64,
    pub user_id: i64,
    pub weight: f64,
    pub attack: f64,
    pub defense: f64,
    pub name: String,
}

impl Pig {
    pub fn from_mysql_row(row: MySqlRow) -> anyhow::Result<Self> {
        let id = row.try_get::<i64, _>(0)?;
        let user_id = row.try_get::<i64, _>(1)?;
        let weight = row.try_get::<f64, _>(2)?;
        let attack = row.try_get::<f64, _>(3)?;
        let defense: f64 = row.try_get(4)?;
        let name = row.try_get::<String, _>(5)?;

        Ok(Self {
            id,
            user_id,
            weight,
            attack,
            defense,
            name,
        })
    }
    pub fn duel(&self, other_pig: &Pig) -> bool {
        let mass_weight = 0.3;
        let power_first = self.attack + mass_weight * self.weight;
        let power_second = other_pig.attack + mass_weight * other_pig.weight;

        let final_first = power_first * rand::rng().random_range(0.6..=1.1);
        let final_second = power_second * rand::rng().random_range(0.6..=1.1);
        println!("Host: {}\nPart: {}", final_first, final_second);
        final_first > final_second
    }
}

pub async fn get_pig(pool: &MySqlPool, user_id: u64) -> anyhow::Result<Pig> {
    return pigdb::get_pig_by_user_id(pool, user_id).await;
}

pub async fn proccess_duel_results(
    pool: &MySqlPool,
    winner_id: u64,
    loser_id: u64,
    bid: f64,
) -> anyhow::Result<()> {
    economydb::add_money(pool, winner_id, bid * 2.0).await?;
    economydb::sub_money(pool, loser_id, bid).await?;
    Ok(())
}
