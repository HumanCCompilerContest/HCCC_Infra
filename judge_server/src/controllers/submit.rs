use axum::{
    extract::{Extension, Form},
    routing,
    Router,
};
use serde::Deserialize;

use crate::entities::JudgeResult;
use crate::database::RepositoryProvider;
use crate::services;

pub fn submit() -> Router {
    Router::new()
        .route("/", routing::post(post))
}

async fn post(form: Form<SubmitForm>, Extension(repository_provider): Extension<RepositoryProvider>) -> &'static str {
    let submit_repo = repository_provider.submit();
    let judge_result = services::accept_submit(&submit_repo, form.user_id, form.submit_id).await;

    match judge_result {
        JudgeResult::Pending => "Pending",
        JudgeResult::AC => "AC",
        JudgeResult::WA => "WA",
        JudgeResult::AE => "AE",
        JudgeResult::LE => "LE",
        JudgeResult::TLE => "TLE",
        JudgeResult::SystemError => "SystemError",
    }
}

#[derive(Deserialize)]
struct SubmitForm {
    user_id: u32,
    submit_id: u32,
}

