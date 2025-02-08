use rand::Rng;
use sqlx::{mysql::MySqlRow, MySqlPool, Row};

use crate::db::pigdb;

pub struct Pig {
    id: i64,
    pub user_id: i64,
    pub weight: f64,
    pub attack: f64,
    pub name: String,
}

impl Pig {
    pub fn from_mysql_row(row: MySqlRow) -> anyhow::Result<Self> {
        let id = row.try_get::<i64, _>(0)?;
        let user_id = row.try_get::<i64, _>(1)?;
        let weight = row.try_get::<f64, _>(2)?;
        let attack = row.try_get::<f64, _>(3)?;
        let name = row.try_get::<String, _>(4)?;

        Ok(Self {
            id,
            user_id,
            weight,
            attack,
            name,
        })
    }
    pub fn duel(&self, other_pig: &Pig) -> bool {
        let mass_weight = 0.3;
        let power_first = self.attack + mass_weight * self.weight;
        let power_second = other_pig.attack + mass_weight * other_pig.weight;

        let final_first = power_first * rand::rng().random_range(0.9..=1.1);
        let final_second = power_second * rand::rng().random_range(0.9..=1.1);

        final_first > final_second
    }
}

pub async fn proccess_duel_results(
    pool: &MySqlPool,
    pig_winner: &Pig,
    pig_loser: &Pig,
    winner_id: u64,
    loser_id: u64,
) -> anyhow::Result<()> {
    let new_weight = pig_winner.weight + pig_loser.weight * 0.1;
    pigdb::set_pig_weight(pool, new_weight, winner_id).await?;

    let mut new_loser_weight = pig_loser.weight * 0.9;
    if new_loser_weight < 10.0 {
        new_loser_weight = 10.0;
    }

    // TODO: econmy stuff (like take n% amount of money from loser and give it to winner)

    pigdb::set_pig_weight(pool, new_loser_weight, loser_id).await?;

    Ok(())
}
