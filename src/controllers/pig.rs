use sqlx::{mysql::MySqlRow, Row};

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
        let weight = row.try_get::<f64, _>(3)?;
        let attack = row.try_get::<f64, _>(4)?;
        let name = row.try_get::<String, _>(2)?;

        Ok(Self {
            id,
            user_id,
            weight,
            attack,
            name,
        })
    }
}
