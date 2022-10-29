use axum::{
    routing,
    Router
};

use crate::controllers::submit;
use crate::database;

pub async fn app() -> Router {
    let database_layer = database::layer().await;
    Router::new()
        .route("/", routing::get(get))
        .nest("/submit", submit::submit())
        .layer(database_layer)
}

async fn get() -> &'static str {
    "Ok"
}
