use crate::entities::Account;

#[axum::async_trait]
pub trait Accounts {
    async fn find_by(&self, user_name: &str) -> Option<Account>;
    async fn store(&self, entity: &Account);
}
