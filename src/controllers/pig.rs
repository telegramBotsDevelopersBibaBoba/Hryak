use rand::Rng;
use sqlx::{mysql::MySqlRow, MySqlPool, Row};

use crate::db::{economydb, pigdb};

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
        println!("Host: {}\nPart: {}", final_first, final_second);
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
    let mut winner_balance = economydb::get_balance(pool, winner_id).await?;
    let mut loser_balance = economydb::get_balance(pool, loser_id).await?;
    let money_bid = (winner_balance / 10.0).min(loser_balance / 10.0);

    economydb::add_money(pool, winner_id, money_bid).await?;
    economydb::sub_money(pool, loser_id, money_bid).await?;

    Ok(())
}
