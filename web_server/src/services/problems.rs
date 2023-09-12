use crate::entities::{AllProblems, Problem};
use crate::repositories::Problems;

/// Get a problem by id.
pub async fn get_problem(repo: &impl Problems, problem_id: i32) -> Problem {
    repo.find_problem(problem_id)
        .await
        .unwrap_or(Problem::error("ng", "problem not found"))
}

/// Get all problems.
pub async fn get_all_problems(repo: &impl Problems) -> AllProblems {
    AllProblems::new("ok".to_string(), repo.all_problems().await, None)
}
