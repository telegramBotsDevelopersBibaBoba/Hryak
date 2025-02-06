use sqlx::{mysql::MySqlRow, Row};

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
