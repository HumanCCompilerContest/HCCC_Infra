use crate::entities::{JudgeResult, Problem, Submission, User, UserSubmissions};
use crate::repositories::{Problems, Submissions, Users};
use chrono::Local;

/// Get a submission by id.
pub async fn get_submission(repo: &impl Submissions, submit_id: i32) -> Submission {
    repo.find_submission(submit_id)
        .await
        .unwrap_or(Submission::error())
}

/// Get all submissions.
pub async fn get_all_users_submissions(repo: &impl Submissions) -> UserSubmissions {
    UserSubmissions::new("ok".to_string(), repo.get_all_submissions().await, None)
}

/// Get all submissions made by the specified user.
pub async fn get_user_submissions(repo: &impl Submissions, user_id: i32) -> UserSubmissions {
    UserSubmissions::new("ok".to_string(), repo.user_submitted(user_id).await, None)
}

/// Submit the answer assembly.
pub async fn submit_asm(
    repo_user: &impl Users,
    repo_prob: &impl Problems,
    repo_submit: &impl Submissions,
    user_id: i32,
    problem_id: i32,
    asm: String,
    is_ce: bool,
) -> Submission {
    let submit_time = Local::now();
    let Some(submission_id) = repo_submit
        .store_submission(
            user_id,
            problem_id,
            submit_time,
            &asm,
            is_ce,
            JudgeResult::Pending,
        )
        .await
    else {
        return Submission::error();
    };
    let user_obj = repo_user
        .find_user(user_id)
        .await
        .unwrap_or(User::error("user not found"));
    let problem_obj = repo_prob
        .find_problem(problem_id)
        .await
        .unwrap_or(Problem::error("ng", "problem not found"));
    Submission::new(
        submission_id,
        submit_time,
        asm,
        is_ce,
        JudgeResult::Pending,
        user_obj.get_object(),
        problem_obj.get_object(),
    )
}
