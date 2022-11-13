use tokio::process::Command;
use chrono::Local;
use crate::entities::{
    Submission,
    UserSubmissions,
    JudgeResult,
    User,
    Problem
};
use crate::repositories::{Users, Problems, Submissions};

pub async fn get_submission(repo: &impl Submissions, submit_id: i32) -> Submission {
    repo.find_submission(submit_id)
        .await
        .unwrap_or(Submission::error())
}

pub async fn get_user_submissions(repo: &impl Submissions, user_id: i32) -> UserSubmissions {
    UserSubmissions::new(
        "ok".to_string(),
        repo.user_submitted(user_id).await,
        None,
    )
}

pub async fn submit_asm(
    repo_user: &impl Users,
    repo_prob: &impl Problems,
    repo_submit: &impl Submissions,
    user_id: i32,
    problem_id: i32,
    asm: String
) -> Submission {
    let submit_time = Local::now();
    let result = Command::new("bash")
        .arg("-c")
        .arg(format!(
            "sudo docker exec judge_system /work/judge_system {}",
            base64::encode(&asm)
        ))
        .output()
        .await;

    if let Ok(result) = result {
        let judge_result = match result.status.code().unwrap_or(6) {
            0 => JudgeResult::AC,
            1 => JudgeResult::WA,
            2 => JudgeResult::AE,
            3 => JudgeResult::LE,
            4 => JudgeResult::RE,
            5 => JudgeResult::TLE,
            _ => JudgeResult::SystemError,
        };

        let submission_id = match repo_submit.store_submission(user_id, problem_id, submit_time, &asm, judge_result).await {
            Some(id) => id,
            None => return Submission::error(),
        };
        let user_obj = repo_user.find_user(user_id).await.unwrap_or(User::error("user not found"));
        let problem_obj = repo_prob.find_problem(problem_id).await.unwrap_or(Problem::error());
        let submission = Submission::new(
            submission_id,
            submit_time,
            asm,
            judge_result,
            user_obj.get_object(),
            problem_obj.get_object()
        );

        submission
    } else {
        Submission::error()
    }
}

