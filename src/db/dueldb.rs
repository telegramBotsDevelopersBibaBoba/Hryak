use sqlx::MySqlPool;

use crate::{controllers::duel::Duel, StoragePool};

pub async fn create_duel(
    pool: &StoragePool,
    host_id: u64,
    part_id: u64,
    host_health: f64,
    part_health: f64,
    bid: f64,
) -> anyhow::Result<()> {
    sqlx::query(
        "INSERT INTO duels (host_id, part_id, host_hp, part_hp, bid) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(host_id)
    .bind(part_id)
    .bind(host_health)
    .bind(part_health)
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
    let row =
        sqlx::query("SELECT host_id, part_id, host_hp, part_hp, bid FROM duels WHERE host_id = ?")
            .bind(host_id)
            .fetch_one(&pool.mysql_pool)
            .await?;
    Duel::from_mysql_row(&row)
}

pub async fn update_duel(pool: &StoragePool, duel: Duel) -> anyhow::Result<()> {
    let row = sqlx::query("UPDATE duels SET host_hp = ?, part_hp = ? WHERE host_id = ?")
        .bind(duel.host_hp)
        .bind(duel.part_hp)
        .bind(duel.host_id)
        .execute(&pool.mysql_pool)
        .await?;
    Ok(())
}
