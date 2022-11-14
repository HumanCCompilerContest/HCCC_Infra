use crate::entities::{JudgeResult, Submit};

#[axum::async_trait]
pub trait Submits {
    async fn get_pending_submit(&self) -> Option<Submit>;
    async fn store_result(&self, result: JudgeResult, submit_id: i32);
}
