use sqlx::{
    mysql::MySqlRow,
    types::chrono::{DateTime, Utc},
    Row,
};

pub struct BankAccount {
    balance: f64,
    daily_income: f64,
    income_time: DateTime<Utc>,
}

impl BankAccount {
    fn from_mysql_row(row: &MySqlRow) -> anyhow::Result<Self> {
        // Предполагаем, что в таблице есть поля для bank_account
        let balance = row.try_get::<f64, _>("balance")?;
        let daily_income = row.try_get::<f64, _>("daily_income")?;
        let income_time = row.try_get::<DateTime<Utc>, _>("income_time")?;

        Ok(Self {
            balance,
            daily_income,
            income_time,
        })
    }
}
