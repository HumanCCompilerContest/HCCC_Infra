use axum::{
    extract::{Extension, Form},
    response::{IntoResponse, Redirect},
    routing,
    Router,
};
use serde::Deserialize;

use crate::request::UserContext;
use crate::database::RepositoryProvider;
use crate::services;

pub fn submit() -> Router {
    Router::new()
        .route("/", routing::post(post))
}

async fn post(
    user_context: UserContext,
    form: Form<SubmitForm>,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> impl IntoResponse {
    let submit_repo = repository_provider.submit();
    services::create_submit(&submit_repo, user_context.user_id, &form.asem).await;
    Redirect::to("/")
}

#[derive(Deserialize)]
struct SubmitForm {
    asem: String,
}

