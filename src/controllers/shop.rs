use crate::{
    db::{economydb, inventorydb, pigdb},
    ser_command,
};
use anyhow::anyhow;
use sqlx::{mysql::MySqlRow, MySqlPool, Row};
use std::fmt::{format, Display};
use teloxide::types::InlineKeyboardButton;

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
        write!(
            f,
            "{}$ - {} ({}Ккал): {}",
            self.price, self.title, self.nutrition, self.description
        )
    }
}

pub struct ImprovementOffer {
    id: i64,
    title: String,
    price: f64,
    description: String,
    improvement_type: String, // Используем String для enum
    strength: f64,
}

impl ImprovementOffer {
    pub fn from_mysql_row(row: MySqlRow) -> anyhow::Result<Self> {
        let id = row.try_get::<i64, _>(0)?;
        let title = row.try_get::<String, _>(1)?;
        let price = row.try_get::<f64, _>(2)?;
        let description: String = row
            .try_get::<Option<String>, _>(3)?
            .unwrap_or(String::from("Нет описания")); // Обрабатываем NULL
        let improvement_type = row.try_get::<String, _>(4)?;
        let strength = row.try_get::<f64, _>(5)?;

        Ok(Self {
            id,
            title,
            price,
            description,
            improvement_type,
            strength,
        })
    }
}

impl Display for ImprovementOffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}$ - {} ({}): {}",
            self.price, self.title, self.improvement_type, self.description
        )
    }
}

pub struct BuffOffer {
    id: i64,
    title: String,
    price: f64,
    description: String,
    usages: i32,
    buff_type: String,
    strength: f64,
}

impl BuffOffer {
    pub fn from_mysql_row(row: MySqlRow) -> anyhow::Result<Self> {
        let id: i64 = row.try_get(0)?;
        let title: String = row.try_get(1)?;
        let price: f64 = row.try_get(2)?;
        let description: String = row.try_get(3)?;
        let usages: i32 = row.try_get(4)?;
        let buff_type: String = row.try_get(5)?;
        let strength: f64 = row.try_get(6)?;

        Ok(Self {
            id,
            title,
            price,
            description,
            usages,
            buff_type,
            strength,
        })
    }
}

impl Display for BuffOffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}$ - {} ({}): {}",
            self.price, self.title, self.buff_type, self.description
        )
    }
}

#[derive(Clone, Copy)]
pub enum OfferType {
    Improvement,
    Food,
    Buff,
}

impl From<&str> for OfferType {
    fn from(value: &str) -> Self {
        match value {
            "food" => Self::Food,
            "improvement" => Self::Improvement,
            "buff" => Self::Buff,
            _ => panic!("Incorrect value for offerType from"),
        }
    }
}

pub enum Offer {
    Improvement(ImprovementOffer),
    Food(FoodOffer),
    Buff(BuffOffer),
}

impl Offer {
    pub fn get_button(&self, index: usize) -> InlineKeyboardButton {
        let (id, title, offer_type) = match self {
            Self::Food(item) => (item.id, item.title.clone(), "food"),
            Self::Improvement(item) => (item.id, item.title.clone(), "improvement"),
            Self::Buff(item) => (item.id, item.title.clone(), "buff"),
        };
        InlineKeyboardButton::callback(
            format!("{}) {}", index, title),
            ser_command!("shop", offer_type, &id.to_string()),
        )
    }

    pub fn get_info(&self, index: usize) -> String {
        match self {
            Self::Food(item) => format!("{}) {}\n", index, item),
            Self::Improvement(item) => format!("{}) {}\n", index, item),
            Self::Buff(item) => format!("{}) {}\n", index, item),
        }
    }

    pub fn get_price(&self) -> f64 {
        match self {
            Self::Food(item) => item.price,
            Self::Improvement(item) => item.price,
            Self::Buff(item) => item.price,
        }
    }

    pub async fn use_item(&self, by_user: u64, pool: &MySqlPool) -> anyhow::Result<()> {
        match self {
            Self::Food(item) => pigdb::feed_pig(pool, item.nutrition, by_user).await,
            Self::Improvement(item) => {
                match item.improvement_type.as_str() {
                    "attack" => {
                        pigdb::increase_attack(pool, item.strength, by_user).await?;
                    }
                    "defense" => {
                        pigdb::increase_defense(pool, item.strength, by_user).await?;
                    }
                    "income" => {
                        economydb::increase_daily_income(pool, by_user, item.strength).await?;
                    }
                    _ => return Err(anyhow!("ImprovOffer cant be this type")),
                }
                return Ok(());
            }
            Self::Buff(item) => {
                inventorydb::add_item(pool, item.id as u64, by_user).await?;
                return Ok(());
            }
        }
    }
}

pub fn get_daily_offers() -> Vec<(u64, OfferType)> {
    vec![
        (1, OfferType::Food),
        (2, OfferType::Improvement),
        (3, OfferType::Food),
        (4, OfferType::Improvement),
        (4, OfferType::Buff),
    ]
}
