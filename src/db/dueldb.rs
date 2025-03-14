use crate::{controllers::duel::Duel, StoragePool};

pub async fn create_duel(
    pool: &StoragePool,
    host_id: u64,
    part_id: u64,
    host_health: f64,
    part_health: f64,
    host_attack: f64,
    host_defense: f64,
    part_attack: f64,
    part_defense: f64,
    bid: f64,
) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO duels (host_id, part_id, host_hp, part_hp, host_attack, host_defense, part_attack, part_defense, bid) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(host_id)
    .bind(part_id)
    .bind(host_health)
    .bind(part_health)
    .bind(host_attack)
    .bind(host_defense)
    .bind(part_attack)
    .bind(part_defense)
    .bind(bid)
    .execute(&pool.mysql_pool)
    .await?;
    Ok(())
}

pub async fn remove_duel(pool: &StoragePool, host_id: u64) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM duels WHERE host_id = ?")
        .bind(host_id)
        .execute(&pool.mysql_pool)
        .await?;
    Ok(())
}

pub async fn duel(pool: &StoragePool, host_id: u64) -> anyhow::Result<Duel> {
    let row: sqlx::mysql::MySqlRow =
        sqlx::query("SELECT host_id, part_id, host_hp, part_hp, bid, host_attack, host_defense, part_attack, part_defense FROM duels WHERE host_id = ?")
            .bind(host_id)
            .fetch_one(&pool.mysql_pool)
            .await?;
    Duel::from_mysql_row(&row)
}

pub async fn update_duel(pool: &StoragePool, duel: &Duel) -> anyhow::Result<()> {
    sqlx::query("UPDATE duels SET host_hp = ?, part_hp = ?, host_attack = ?, host_defense = ?, part_attack = ?, part_defense = ? WHERE host_id = ?")
        .bind(duel.host_hp)
        .bind(duel.part_hp)
        .bind(duel.host_attack)
        .bind(duel.host_defense)
        .bind(duel.part_attack)
        .bind(duel.part_defense)
        .bind(duel.host_id)
        .execute(&pool.mysql_pool)
        .await?;
    Ok(())
}
