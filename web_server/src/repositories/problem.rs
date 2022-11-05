use crate::entities::{Problem, ProblemObject};

#[axum::async_trait]
pub trait Problems {
    async fn find_problem(&self, problem_id: i32) -> Option<Problem>;
    async fn all_problems(&self) -> Vec<ProblemObject>;
}


