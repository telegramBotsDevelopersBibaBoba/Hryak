use sqlx::{mysql::MySqlRow, Row};

pub struct Pig {
    id: u64,
    user_id: u64,
    weight: f64,
    attack: f64,
    name: String,
}

impl Pig {
    pub fn from_mysql_row(row: MySqlRow) -> anyhow::Result<Self> {
        let id = row.try_get::<u64, _>(0)?;
        let user_id = row.try_get::<u64, _>(1)?;
        let weight = row.try_get::<f64, _>(2)?;
        let attack = row.try_get::<f64, _>(3)?;

        Ok(Self {
            id,
            user_id,
            weight,
            attack,
            name: String::new(),
        })
    }
}