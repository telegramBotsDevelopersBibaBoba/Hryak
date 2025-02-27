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

pub mod inline {
    use sqlx::MySqlPool;
    use teloxide::{
        payloads::AnswerInlineQuerySetters,
        prelude::Requester,
        types::{InlineQuery, InlineQueryResult},
        Bot,
    };

    use crate::handlers::articles;

    pub async fn inline_change_name(bot: Bot, q: &InlineQuery, data: &str) -> anyhow::Result<()> {
        let changename = articles::inline_change_name_article(data);

        let articles = vec![InlineQueryResult::Article(changename)];
        bot.answer_inline_query(&q.id, articles)
            .cache_time(0)
            .await?;
        Ok(())
    }

    pub async fn inline_name(bot: Bot, q: &InlineQuery) -> anyhow::Result<()> {
        let name = articles::inline_name_article();

        let articles = vec![InlineQueryResult::Article(name)];

        bot.answer_inline_query(&q.id, articles)
            .cache_time(0)
            .await?;
        Ok(())
    }

    pub async fn inline_hryak_info(
        bot: Bot,
        q: &InlineQuery,
        pool: &MySqlPool,
    ) -> anyhow::Result<()> {
        let hryak =
            articles::inline_hryak_info_article(pool, &q.from.username, q.from.id.0).await?;

        let articles = vec![InlineQueryResult::Article(hryak)];

        bot.answer_inline_query(&q.id, articles)
            .cache_time(0)
            .await?; // Showing all suitable inline buttons
        Ok(())
    }
}

pub mod feedback {
    use sqlx::MySqlPool;
    use teloxide::{types::ChosenInlineResult, Bot};

    use crate::db::pigdb;

    pub async fn feedback_rename_hryak(
        bot: Bot,
        q: &ChosenInlineResult,
        args: &[&str],
        pool: &MySqlPool,
    ) -> anyhow::Result<()> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("Rename hryak args are emtpy"));
        }
        pigdb::set_pig_name(pool, &args[0], q.from.id.0).await?;
        Ok(())
    }
}
