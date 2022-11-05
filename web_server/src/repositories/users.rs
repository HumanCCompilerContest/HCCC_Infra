use crate::entities::{User, UserObject};

#[axum::async_trait]
pub trait Users {
    async fn find_user(&self, user_id: i32) -> Option<User>;
    async fn all_users(&self) -> Vec<UserObject>;
}

