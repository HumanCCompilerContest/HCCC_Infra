use axum::{
    extract::{self, Extension, Path, Query},
    routing,
    Json,
    Router,
};
use serde::Deserialize;

use crate::services;
use crate::entities::{Submission, UserSubmissions};
use crate::request::UserContext;
use crate::database::RepositoryProvider;

pub fn submissions() -> Router {
    Router::new()
        .route("/", routing::get(from_user_id))
        .route("/:id", routing::get(from_submit_id))
}

async fn from_user_id(
    Query(param): Query<UserIdParam>,
    _: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> Json<UserSubmissions> {
    tracing::debug!("/api/submissions");
    let submission_repo = repository_provider.submission();
    Json(services::get_user_submissions(&submission_repo, param.user_id).await)
}

async fn from_submit_id(
    Path(id): Path<i32>,
    _: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> Json<Submission> {
    tracing::debug!("/api/submissions/:id");
    let submission_repo = repository_provider.submission();
    Json(services::get_submission(&submission_repo, id).await)
}

pub async fn submit(
    Path(id): Path<i32>,
    extract::Json(req): extract::Json<SubmitReq>,
    user_context: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> Json<Submission> {
    tracing::debug!("/api/problems/:id/submissions");
    let user_repo = repository_provider.user();
    let problem_repo = repository_provider.problem();
    let submission_repo = repository_provider.submission();
    Json(
        services::submit_asm(
            &user_repo,
            &problem_repo,
            &submission_repo,
            user_context.user_id,
            id,
            req.asm,
        ).await
    )
}

#[derive(Deserialize)]
pub struct SubmitReq {
    asm: String,
}

#[derive(Deserialize)]
struct UserIdParam {
    user_id: i32,
}
