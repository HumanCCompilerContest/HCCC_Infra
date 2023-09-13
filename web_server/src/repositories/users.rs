use crate::entities::{Rank, User, UserObject};

/// A trait for defining requests about user accounts to the database.
#[axum::async_trait]
pub trait Users {
    async fn find_user(&self, user_id: i32) -> Option<User>;
    async fn all_users(&self) -> Vec<UserObject>;
    async fn create_ranking(&self) -> Vec<Rank>;
}
