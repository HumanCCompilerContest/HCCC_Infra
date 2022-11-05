use axum::{
    extract::Extension,
    routing,
    Json,
    Router,
};

use crate::services;
use crate::entities::User;
use crate::request::UserContext;
use crate::database::RepositoryProvider;

pub fn user() -> Router {
    Router::new()
        .route("/me", routing::get(me))
}

async fn me(
    user_context: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> Json<User> {
    let user_data = repository_provider.user();
    Json(services::get_me(&user_data, user_context.user_id).await)
}
