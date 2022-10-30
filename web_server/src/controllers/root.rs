use axum::{
    extract::{Extension, Query},
    response::IntoResponse,
    routing,
    Router
};
use serde::Deserialize;

use crate::controllers::{accounts, submits};
use crate::database::{self, RepositoryProvider};
use crate::request::UserContext;
use crate::response;
use crate::services;
use crate::views::{SignIn, SignUp};

pub async fn app() -> Router {
    let database_layer = database::layer().await;
    Router::new()
        .route("/", routing::get(get))
        .nest("/login", routing::get(login))
        .nest("/register", routing::get(register))
        .nest("/submit", submits::submit())
        .nest("/accounts", accounts::accounts())
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

#[derive(Deserialize)]
struct LoginQuery {
    error: Option<String>,
}
