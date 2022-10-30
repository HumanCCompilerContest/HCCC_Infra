use axum::{
    extract::{Extension, Form},
    response::{AppendHeaders, IntoResponse, Redirect},
    http::header::SET_COOKIE,
    routing,
    Router,
};
use serde::Deserialize;

use crate::database::RepositoryProvider;
use crate::services::{self, SessionToken};

pub fn accounts() -> Router {
    Router::new()
        .route("/new", routing::post(post))
        .route("/session", routing::post(new_session))
}

async fn post(
    form: Form<SignUpForm>,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> impl IntoResponse {
    let account_repo = repository_provider.accounts();
    services::create_account(
        &account_repo,
        &form.user_name,
        &form.password,
    )
    .await;
    let session_token = services::create_session(&account_repo, &form.user_name, &form.password).await;
    redirect_with_session(session_token)
}

async fn new_session(
    form: Form<SignInForm>,
    Extension(repository_provider): Extension<RepositoryProvider>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let account_repo = repository_provider.accounts();
    let session_token = services::create_session(&account_repo, &form.user_name, &form.password);
    redirect_with_session(session_token.await)
}

fn redirect_with_session(
    session: Option<SessionToken>
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Some(session_token) = session {
        let headers = AppendHeaders([(SET_COOKIE, session_token.cookie())]);
        let response = Redirect::to("/");
        Ok((headers, response))
    } else {
        Err(Redirect::to("/login?error=invalid"))
    }
}

#[derive(Deserialize)]
struct SignInForm {
    user_name: String,
    password: String,
}

#[derive(Deserialize)]
struct SignUpForm {
    user_name: String,
    password: String,
}

