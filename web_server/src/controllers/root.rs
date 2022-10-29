use axum::{
    extract::Extension,
    response::IntoResponse,
    routing,
    Router
};

use crate::controllers::submit;
use crate::database::{self, RepositoryProvider};
use crate::response;
use crate::services;

pub async fn app() -> Router {
    let database_layer = database::layer().await;
    Router::new()
        .route("/", routing::get(get))
        .nest("/submit", submit::submit())
        .layer(database_layer)
}

async fn get(Extension(repository_provider): Extension<RepositoryProvider>) -> impl IntoResponse {
    let submit_repo = repository_provider.submit();
    let home = services::list_submit(&submit_repo).await;
    response::from_template(home)
}
