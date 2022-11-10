use axum::{
    extract::{Extension, Form},
    response::{AppendHeaders, IntoResponse},
    http::header::SET_COOKIE,
    Json,
};
use serde::Deserialize;

use crate::entities::AccountResponse;
use crate::database::RepositoryProvider;
use crate::services;

pub async fn register(
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
    let (id, session_token) = services::create_session(&account_repo, &form.user_name, &form.password).await;

    match session_token {
        Some(session_token) => {
            let headers = AppendHeaders([(SET_COOKIE, session_token.cookie())]);
            let response = Json(AccountResponse::new(id.unwrap(), form.user_name.clone()));
            Ok((headers, response))
        },
        None => Err(Json(AccountResponse::error()))
    }
}

pub async fn login(
    form: Form<SignInForm>,
    Extension(repository_provider): Extension<RepositoryProvider>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let account_repo = repository_provider.accounts();
    let (id, session_token) = services::create_session(&account_repo, &form.user_name, &form.password).await;

    match session_token {
        Some(session_token) => {
            let headers = AppendHeaders([(SET_COOKIE, session_token.cookie())]);
            let response = Json(AccountResponse::new(id.unwrap(), form.user_name.clone()));
            Ok((headers, response))
        },
        None => Err(Json(AccountResponse::error()))
    }
}

#[derive(Deserialize)]
pub struct SignInForm {
    user_name: String,
    password: String,
}

#[derive(Deserialize)]
pub struct SignUpForm {
    user_name: String,
    password: String,
}

