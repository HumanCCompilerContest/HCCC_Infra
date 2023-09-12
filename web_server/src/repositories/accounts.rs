use crate::entities::Account;

/// A trait for defining requests about accounts to the database.
#[axum::async_trait]
pub trait Accounts {
    async fn find_by(&self, user_name: &str) -> Option<Account>;
    async fn store(&self, entity: &Account) -> Result<u64, tokio_postgres::Error>;
}
