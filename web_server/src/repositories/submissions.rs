use crate::entities::{Submission, SubmissionObject};

#[axum::async_trait]
pub trait Submissions {
    async fn find_submission(&self, problem_id: i32) -> Option<Submission>;
    async fn user_submitted(&self, user_id: i32) -> Vec<SubmissionObject>;
}


