use axum::{
    extract::{Extension, Query},
    response::IntoResponse,
    routing,
    Json,
    Router
};
use serde::Deserialize;

use crate::controllers::{accounts, submits, users, problems, submissions};
use crate::database::{self, RepositoryProvider};
use crate::request::UserContext;
use crate::response;
use crate::services;
use crate::entities::Ranking;
use crate::views::{SignIn, SignUp};

pub async fn app() -> Router {
    let database_layer = database::layer().await;
    Router::new()
        .route("/", routing::get(get))
        .nest("/api/login", routing::get(login))
        .nest("/api/register", routing::get(register))
        .nest("/api/ranking", routing::get(ranking))
        .nest("/api/users", users::user())
        .nest("/api/problems", problems::problem())
        .nest("/api/submit", submits::submit())
        .nest("/api/accounts", accounts::accounts())
        .nest("/api/submissions", submissions::submissions())
        .layer(database_layer)
}

async fn get(
    _: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> impl IntoResponse {
    let submit_repo = repository_provider.submit();
    let home = services::list_submit(&submit_repo).await;
    response::from_template(home)
}

async fn login(query: Query<LoginQuery>) -> impl IntoResponse {
    response::from_template(
        SignIn {
            error: query.error.is_some(),
        }
    )
}

async fn register() -> impl IntoResponse {
    response::from_template(SignUp)
}

async fn ranking(
    user_context: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> Json<Ranking> {
    let user_repo = repository_provider.user();
    Json(services::get_ranking(&user_repo).await)
}

#[derive(Deserialize)]
struct LoginQuery {
    error: Option<String>,
}
