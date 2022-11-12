use chrono::{DateTime, Local};
use crate::entities::{Submission, SubmissionObject, JudgeResult};

#[axum::async_trait]
pub trait Submissions {
    async fn find_submission(&self, problem_id: i32) -> Option<Submission>;
    async fn user_submitted(&self, user_id: i32) -> Vec<SubmissionObject>;
    async fn store_submission<'a>(
        &self,
        user_id: i32,
        problem_id: i32,
        submit_time: DateTime<Local>,
        asm: &'a str,
        judge_result: JudgeResult,
    ) -> Option<i32>;
}


