use axum::{
    extract::Extension,
    http::{StatusCode, HeaderValue, Method},
    http::header::CONTENT_TYPE,
    routing,
    Json,
    Router
};
use tower_http::cors::CorsLayer;

use crate::controllers::{accounts, users, problems, submissions};
use crate::database::{self, RepositoryProvider};
use crate::request::UserContext;
use crate::services;
use crate::entities::Ranking;

pub async fn app() -> Router {
    let cors_layer = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::HEAD, Method::OPTIONS])
        .allow_headers([CONTENT_TYPE]);
    let database_layer = database::layer().await;

    Router::new()
        .route("/", routing::get(get))
        .nest("/api/login", routing::post(accounts::login))
        .nest("/api/register", routing::post(accounts::register))
        .nest("/api/ranking", routing::get(ranking))
        .nest("/api/users", users::user())
        .nest("/api/problems", problems::problem())
        .nest("/api/submissions", submissions::submissions())
        .layer(cors_layer)
        .layer(database_layer)
}

async fn get() -> StatusCode {
    StatusCode::NOT_FOUND
}

async fn ranking(
    Extension(repository_provider): Extension<RepositoryProvider>
) -> Json<Ranking> {
    tracing::debug!("/api/ranking");
    let user_repo = repository_provider.user();
    Json(services::get_ranking(&user_repo).await)
}
