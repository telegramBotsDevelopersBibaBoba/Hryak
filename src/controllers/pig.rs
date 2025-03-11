use rand::Rng;
use sqlx::{mysql::MySqlRow, Row};

use crate::{
    db::{economydb, pigdb, userdb},
    StoragePool,
};

pub const DEFAULT_WEIGHT: f64 = 50.0;
pub const DEFAULT_ATTACK: f64 = 5.0;
pub const DEFAULT_DEFENSE: f64 = 5.0;

pub struct Pig {
    pub id: i64,
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
}

pub async fn get_pig(pool: &StoragePool, user_id: u64) -> anyhow::Result<Pig> {
    return pigdb::pig_by_userid(pool, user_id).await;
}

pub mod inline {
    use teloxide::{
        payloads::AnswerInlineQuerySetters,
        prelude::Requester,
        types::{InlineQuery, InlineQueryResult},
        Bot,
    };

    use crate::{handlers::articles, StoragePool};

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

        bot.answer_inline_query(&q.id, articles).await?;
        Ok(())
    }

    pub async fn inline_hryak_info(
        bot: Bot,
        q: &InlineQuery,
        pool: &StoragePool,
    ) -> anyhow::Result<()> {
        let hryak = articles::inline_hryak_info_article(pool, q.from.id.0).await?;

        let articles = vec![InlineQueryResult::Article(hryak)];

        bot.answer_inline_query(&q.id, articles)
            .cache_time(0)
            .await?; // Showing all suitable inline buttons
        Ok(())
    }
}

pub mod feedback {
    use teloxide::{types::ChosenInlineResult, Bot};

    use crate::{db::pigdb, StoragePool};

    pub async fn feedback_rename_hryak(
        bot: Bot,
        q: &ChosenInlineResult,
        args: &[&str],
        pool: &StoragePool,
    ) -> anyhow::Result<()> {
        if args.is_empty() {
            return Err(anyhow::anyhow!("Rename hryak args are emtpy"));
        }
        pigdb::set_name(pool, &args[0], q.from.id.0).await?;
        Ok(())
    }
}
