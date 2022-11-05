use axum::{
    extract::{Extension, Path},
    routing,
    Json,
    Router,
};

use crate::services;
use crate::entities::{User, AllUsers};
use crate::request::UserContext;
use crate::database::RepositoryProvider;

pub fn user() -> Router {
    Router::new()
        .route("/", routing::get(all_user))
        .route("/:id", routing::get(user_from_id))
        .route("/me", routing::get(me))
}

async fn all_user(
    _: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> Json<AllUsers> {
    let user_repo = repository_provider.user();
    Json(services::get_all_users(&user_repo).await)
}

async fn user_from_id(
    Path(id): Path<i32>,
    _: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> Json<User> {
    let user_repo = repository_provider.user();
    Json(services::get_user(&user_repo, id).await)
}

async fn me(
    user_context: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> Json<User> {
    let user_repo = repository_provider.user();
    Json(services::get_user(&user_repo, user_context.user_id).await)
}
