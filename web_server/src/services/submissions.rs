use crate::entities::{Submission, UserSubmissions};
use crate::repositories::Submissions;

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

