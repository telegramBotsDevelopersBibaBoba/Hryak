use sqlx::{mysql::MySqlRow, MySqlPool, Row};
use teloxide::types::InlineKeyboardButton;
use crate::{db::pigdb, ser_command};
use std::fmt::Display;

pub struct FoodOffer {
    id: i64,
    price: f64,
    nutrition: f64,
    title: String,
    description: String,
}

impl FoodOffer {
    pub fn from_mysql_row(row: MySqlRow) -> anyhow::Result<Self> {
        let id = row.try_get::<i64, _>(0)?;
        let price = row.try_get::<f64, _>(2)?;
        let nutrition = row.try_get::<f64, _>(4)?;
        let title = row.try_get::<String, _>(1)?;
        let description = row.try_get::<String, _>(3)?;

        Ok(Self {
            id,
            price,
            nutrition,
            title,
            description,
        })
    }
}

impl Display for FoodOffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}$ - {} ({}Ккал): {}", self.price, self.title, self.nutrition, self.description)
    }
}

pub struct ImprovementOffer {
    id: i64,
    title: String,
    price: f64,
    description: String,
    improvement_type: String, // Используем String для enum
}

impl ImprovementOffer {
    pub fn from_mysql_row(row: MySqlRow) -> anyhow::Result<Self> {
        let id = row.try_get::<i64, _>(0)?;
        let title = row.try_get::<String, _>(1)?;
        let price = row.try_get::<f64, _>(2)?;
        let description: String = row.try_get::<Option<String>, _>(3)?
            .unwrap_or(String::from("Нет описания")); // Обрабатываем NULL
        let improvement_type = row.try_get::<String, _>(4)?;

        Ok(Self {
            id,
            title,
            price,
            description,
            improvement_type,
        })
    }
}

impl Display for ImprovementOffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}$ - {} ({}): {}", self.price, self.title, self.improvement_type, self.description)
    }
}

#[derive(Clone, Copy)]
pub enum OfferType {
    Improvement,
    Food,
}

impl From<&str> for OfferType {
    fn from(value: &str) -> Self {
        match value {
            "food" => Self::Food,
            "improvement" => Self::Improvement,
            val => panic!("incorrect value for OfferType: {}", val),
        }
    }
}

pub enum Offer {
    Improvement(ImprovementOffer),
    Food(FoodOffer),
}

impl Offer {
    pub fn get_button(&self, index: usize) -> InlineKeyboardButton {
        let (id, title, offer_type) = match self {
            Self::Food(item) => (item.id, item.title.clone(), "food"),
            Self::Improvement(item) => (item.id, item.title.clone(), "improvement"),
        };
        InlineKeyboardButton::callback(
            format!("{}) {}", index, title), 
            ser_command!("shop", offer_type, &id.to_string())
        )
    }

    pub fn get_info(&self, index: usize) -> String {
        match self {
            Self::Food(item) => format!("{}) {}\n", index, item),
            Self::Improvement(item) => format!("{}) {}\n", index, item),
        }
    }

    pub fn get_price(&self) -> f64 {
        match self {
            Self::Food(item) => item.price,
            Self::Improvement(item) => item.price,
        }
    }

    pub async fn use_item(&self, by_user: u64, pool: &MySqlPool) -> anyhow::Result<()>{
        match self {
            Self::Food(item) => pigdb::add_to_pig_weight(pool, item.nutrition, by_user).await,
            Self::Improvement(item) => Ok(())
        }
    }
}


pub fn get_daily_offers() -> Vec<(u64, OfferType)> {
    vec![
        (1, OfferType::Food),
        (2, OfferType::Improvement),
        (3, OfferType::Food),
        (4, OfferType::Improvement),
    ]
}