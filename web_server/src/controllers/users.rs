use axum::{
    extract::{Extension, Path},
    routing, Json, Router,
};

use crate::database::RepositoryProvider;
use crate::entities::{AllUsers, User};
use crate::request::UserContext;
use crate::services;

/// Return `/api/users/*` api.
pub fn user() -> Router {
    Router::new()
        .route("/", routing::get(all_user))
        .route("/:id", routing::get(user_from_id))
        .route("/me", routing::get(me))
}

/// Return all users.
async fn all_user(
    _: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>,
) -> Json<AllUsers> {
    tracing::debug!("/api/user");
    let user_repo = repository_provider.user();
    Json(services::get_all_users(&user_repo).await)
}

/// Retrieve the specified user by their user id.
async fn user_from_id(
    Path(id): Path<i32>,
    _: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>,
) -> Json<User> {
    tracing::debug!("/api/user/:id");
    let user_repo = repository_provider.user();
    Json(services::get_user(&user_repo, id).await)
}

/// Return the registered user.
async fn me(
    user_context: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>,
) -> Json<User> {
    tracing::debug!("/api/user/me");
    let user_repo = repository_provider.user();
    Json(services::get_user(&user_repo, user_context.user_id()).await)
}
