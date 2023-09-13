use crate::entities::{JudgeResult, Submit};

/// A trait for database connection.
#[axum::async_trait]
pub trait Submits {
    async fn get_pending_submits(&self) -> Vec<Submit>;
    async fn store_result(&self, result: JudgeResult, error_message: String, submit_id: i32);
}
