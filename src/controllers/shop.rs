use sqlx::{mysql::MySqlRow, Row};
use teloxide::types::InlineKeyboardButton;
use crate::ser_command;
use std::fmt::Display;

pub struct FoodOffer {
    id: u64,
    price: f64,
    nutrition: f64,
    title: String,
    description: String,
}

impl FoodOffer {
    pub fn from_mysql_row(row: MySqlRow) -> anyhow::Result<Self> {
        let id = row.try_get::<u64, _>(0)?;
        let price = row.try_get::<f64, _>(1)?;
        let nutrition = row.try_get::<f64, _>(2)?;
        let title = row.try_get::<String, _>(3)?;
        let description = row.try_get::<String, _>(4)?;

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
    id: u64,
    title: String,
    price: f64,
    description: String,
    improvement_type: String, // Используем String для enum
}

impl ImprovementOffer {
    pub fn from_mysql_row(row: MySqlRow) -> anyhow::Result<Self> {
        let id = row.try_get::<u64, _>(0)?;
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
        write!(f, "{} - {} ({}): {}", self.price, self.title, self.improvement_type, self.description)
    }
}

pub enum OfferType {
    Improvement,
    Food,
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
            ser_command!("shop", "food", &id.to_string())
        )
    }

    pub fn get_info(&self, index: usize) -> String {
        match self {
            Self::Food(item) => format!("{}) {}", index, item),
            Self::Improvement(item) => format!("{}) {}", index, item),
        }
    }
}