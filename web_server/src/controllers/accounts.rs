use axum::{
    extract::Extension,
    response::{AppendHeaders, IntoResponse},
    http::header::SET_COOKIE,
    Json,
};
use serde::Deserialize;

use crate::entities::AccountResponse;
use crate::database::RepositoryProvider;
use crate::services;

pub async fn register(
    form: Json<SignUp>,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> impl IntoResponse {
    tracing::debug!("/api/register");
    let account_repo = repository_provider.accounts();
    services::create_account(
        &account_repo,
        &form.name,
        &form.password,
    )
    .await;
    let (id, session_token) = services::create_session(&account_repo, &form.name, &form.password).await;

    match session_token {
        Some(session_token) => {
            let headers = AppendHeaders([(SET_COOKIE, session_token.cookie())]);
            let response = Json(AccountResponse::new(id.unwrap(), form.name.clone()));
            Ok((headers, response))
        },
        None => Err(Json(AccountResponse::error()))
    }
}

pub async fn login(
    form: Json<SignIn>,
    Extension(repository_provider): Extension<RepositoryProvider>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    tracing::debug!("/api/login");
    let account_repo = repository_provider.accounts();
    let (id, session_token) = services::create_session(&account_repo, &form.name, &form.password).await;

    match session_token {
        Some(session_token) => {
            let headers = AppendHeaders([(SET_COOKIE, session_token.cookie())]);
            let response = Json(AccountResponse::new(id.unwrap(), form.name.clone()));
            Ok((headers, response))
        },
        None => Err(Json(AccountResponse::error()))
    }
}

#[derive(Deserialize)]
pub struct SignIn {
    name: String,
    password: String,
}

#[derive(Deserialize)]
pub struct SignUp {
    name: String,
    password: String,
}

