use crate::{
    db::{economydb, pigdb, userdb},
    StoragePool,
};

pub async fn create_user(pool: &StoragePool, user_id: u64, username: &str) -> anyhow::Result<()> {
    userdb::create_user(pool, user_id, username).await?;
    pigdb::create_pig(pool, user_id).await?;
    economydb::create_bank_account(pool, user_id).await?;
    Ok(())
}
