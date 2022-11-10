use axum::{
    extract::Extension,
    http::StatusCode,
    routing,
    Json,
    Router
};

use crate::controllers::{accounts, users, problems, submissions};
use crate::database::{self, RepositoryProvider};
use crate::request::UserContext;
use crate::services;
use crate::entities::Ranking;

pub async fn app() -> Router {
    let database_layer = database::layer().await;
    Router::new()
        .route("/", routing::get(get))
        .nest("/api/login", routing::get(accounts::login))
        .nest("/api/register", routing::get(accounts::register))
        .nest("/api/ranking", routing::get(ranking))
        .nest("/api/users", users::user())
        .nest("/api/problems", problems::problem())
        .nest("/api/submissions", submissions::submissions())
        .layer(database_layer)
}

async fn get() -> StatusCode {
    StatusCode::NOT_FOUND
}

async fn ranking(
    _: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> Json<Ranking> {
    let user_repo = repository_provider.user();
    Json(services::get_ranking(&user_repo).await)
}
