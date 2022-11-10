use axum::{
    extract::{Extension, Path, Query},
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
    param: Query<UserIdParam>,
    _: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> Json<UserSubmissions> {
    let submission_repo = repository_provider.submission();
    Json(services::get_user_submissions(&submission_repo, param.user_id).await)
}

async fn from_submit_id(
    Path(id): Path<i32>,
    _: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> Json<Submission> {
    let submission_repo = repository_provider.submission();
    Json(services::get_submission(&submission_repo, id).await)
}

#[derive(Deserialize)]
struct UserIdParam {
    user_id: i32,
}
