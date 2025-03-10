use crate::{
    controllers::pig::{Pig, DEFAULT_ATTACK, DEFAULT_DEFENSE, DEFAULT_WEIGHT},
    StoragePool,
};
use redis::Commands;
use sqlx::Row;

pub async fn create_pig(pool: &StoragePool, user_id: u64) -> anyhow::Result<()> {
    sqlx::query("INSERT INTO pigs (user_id) VALUES (?)")
        .bind(user_id)
        .execute(&pool.mysql_pool)
        .await?;

    let mut redis_con = pool.redis_pool.get()?;
    let _: () = redis_con.hset_multiple(
        format!("pig:{}", user_id),
        &[
            ("weight", DEFAULT_WEIGHT.to_string()),
            ("attack", DEFAULT_ATTACK.to_string()),
            ("defense", DEFAULT_DEFENSE.to_string()),
            ("user_id", user_id.to_string()),
            ("name", String::from("Unnamed")),
        ],
    )?;

    Ok(())
}

pub async fn exists(pool: &StoragePool, user_id: u64) -> bool {
    match sqlx::query("SELECT * FROM pigs WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(&pool.mysql_pool)
        .await
    {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn weight(pool: &StoragePool, user_id: u64) -> anyhow::Result<f64> {
    let mut redis_con = pool.redis_pool.get()?;
    if let Ok(val) = redis_con.hget::<_, _, f64>(format!("pig:{}", user_id), "weight") {
        println!("weight from cache");
        return Ok(val);
    }

    let row = sqlx::query("SELECT weight FROM pigs WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(&pool.mysql_pool)
        .await?; // If throws -> user does not exist
    let weight = row.try_get::<f64, _>(0)?;

    let _: () = redis_con.hset(format!("pig:{}", user_id), "weight", weight)?;
    Ok(weight)
}

pub async fn pig_by_userid(pool: &StoragePool, user_id: u64) -> anyhow::Result<Pig> {
    let mut redis_con = pool.redis_pool.get()?;

    if let Ok(pig_str) = redis_con.hgetall::<_, Vec<(String, String)>>(format!("pig:{}", user_id)) {
        let pig = Pig {
            weight: pig_str[0].1.parse::<f64>()?,
            attack: pig_str[1].1.parse::<f64>()?,
            defense: pig_str[2].1.parse::<f64>()?,
            user_id: pig_str[3].1.parse::<i64>()?,
            name: pig_str[4].1.clone(),
            id: 0,
        };
        println!("pig struct from cache");
        return Ok(pig);
    }

    let row = sqlx::query("SELECT * FROM pigs WHERE user_id = ?")
        .bind(user_id)
        .fetch_one(&pool.mysql_pool)
        .await?;

    let pig_struct = Pig::from_mysql_row(row)?;
    let _: () = redis_con.hset_multiple(
        format!("pig:{}", user_id),
        &[
            ("weight", pig_struct.weight.to_string()),
            ("attack", pig_struct.attack.to_string()),
            ("defense", pig_struct.defense.to_string()),
            ("user_id", pig_struct.user_id.to_string()),
            ("name", String::from("Unnamed")),
        ],
    )?;

    Ok(pig_struct)
}

pub async fn set_name(pool: &StoragePool, name: &str, user_id: u64) -> anyhow::Result<()> {
    let name = if name.is_empty() { "Unnamed" } else { name };
    sqlx::query("UPDATE pigs SET name = ? WHERE user_id = ?")
        .bind(name)
        .bind(user_id)
        .execute(&pool.mysql_pool)
        .await?;

    let mut redis_con = pool.redis_pool.get()?;
    let _: () = redis_con.hset(format!("pig:{}", user_id), "name", name)?;

    Ok(())
}

pub async fn set_weight(pool: &StoragePool, new_weight: f64, user_id: u64) -> anyhow::Result<()> {
    sqlx::query("UPDATE pigs SET weight = ? WHERE user_id = ?")
        .bind(new_weight)
        .bind(user_id)
        .execute(&pool.mysql_pool)
        .await?;

    let mut redis_con = pool.redis_pool.get()?;
    let _: () = redis_con.hset(format!("pig:{}", user_id), "weight", new_weight)?;

    Ok(())
}

pub async fn increase_attack(
    pool: &StoragePool,
    add_attack: f64,
    user_id: u64,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE pigs SET attack = attack + ? WHERE user_id = ?")
        .bind(add_attack)
        .bind(user_id)
        .execute(&pool.mysql_pool)
        .await?;

    let mut redis_con = pool.redis_pool.get()?;
    let cur_attack: f64 = redis_con.hget(format!("pig:{}", user_id), "attack")?;
    let _: () = redis_con.hset(
        format!("pig:{}", user_id),
        "attack",
        ((cur_attack + add_attack) * 100.0).floor() / 100.0,
    )?;

    Ok(())
}

pub async fn increase_defense(
    pool: &StoragePool,
    add_def: f64,
    user_id: u64,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE pigs SET defense = defense + ? WHERE user_id = ?")
        .bind(add_def)
        .bind(user_id)
        .execute(&pool.mysql_pool)
        .await?;

    let mut redis_con = pool.redis_pool.get()?;
    let cur_attack: f64 = redis_con.hget(format!("pig:{}", user_id), "defense")?;
    let _: () = redis_con.hset(
        format!("pig:{}", user_id),
        "defense",
        ((cur_attack + add_def) * 100.0).floor() / 100.0,
    )?;

    Ok(())
}

pub async fn feed(pool: &StoragePool, nutrition: f64, user_id: u64) -> anyhow::Result<()> {
    let mass = nutrition / 1000.0;

    sqlx::query("UPDATE pigs SET weight = weight + ? WHERE user_id = ?")
        .bind(mass)
        .bind(user_id)
        .execute(&pool.mysql_pool)
        .await?;

    let mut redis_con = pool.redis_pool.get()?;
    let cur_weight: f64 = redis_con.hget(format!("pig:{}", user_id), "weight")?;
    let _: () = redis_con.hset(
        format!("pig:{}", user_id),
        "weight",
        ((cur_weight + mass) * 100.0).floor() / 100.0,
    )?;

    Ok(())
}
