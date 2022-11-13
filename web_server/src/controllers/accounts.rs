use axum::{
    extract::{self, Extension},
    response::{AppendHeaders, IntoResponse},
    http::header::SET_COOKIE,
    Json,
};
use async_session::SessionStore;
use async_sqlx_session::PostgresSessionStore;
use serde_json::json;
use serde::Deserialize;

use crate::request::UserContext;
use crate::constants::database_url;
use crate::entities::AccountResponse;
use crate::database::RepositoryProvider;
use crate::services;

pub async fn register(
    extract::Json(account_data): extract::Json<SignUp>,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> impl IntoResponse {
    tracing::debug!("/api/register");
    let account_repo = repository_provider.accounts();
    let create_account_result = services::create_account(
        &account_repo,
        &account_data.name,
        &account_data.password,
    )
    .await;

    if create_account_result.is_err() {
        return Err(Json(AccountResponse::error("this username is already in use")));
    }

    let (id, session_token) = services::create_session(&account_repo, &account_data.name, &account_data.password).await;
    match session_token {
        Some(session_token) => {
            let headers = AppendHeaders([(SET_COOKIE, session_token.cookie())]);
            let response = Json(AccountResponse::new(id.unwrap(), account_data.name.clone()));
            Ok((headers, response))
        },
        None => Err(Json(AccountResponse::error("create accout failed")))
    }
}

pub async fn login(
    extract::Json(account_data): extract::Json<SignIn>,
    Extension(repository_provider): Extension<RepositoryProvider>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    tracing::debug!("/api/login");
    let account_repo = repository_provider.accounts();
    let (id, session_token) = services::create_session(&account_repo, &account_data.name, &account_data.password).await;

    match session_token {
        Some(session_token) => {
            let headers = AppendHeaders([(SET_COOKIE, session_token.cookie())]);
            let response = Json(AccountResponse::new(id.unwrap(), account_data.name.clone()));
            Ok((headers, response))
        },
        None => Err(Json(AccountResponse::error("ログイン名またはパスワードが違います．")))
    }
}

pub async fn logout(user_context: UserContext) -> Result<Json<serde_json::Value>, Json<serde_json::Value>> {
    tracing::debug!("/api/logout");
    let ok_json = || Json(json!({"status": "ok", "errorMessage": ""}));
    let err_json = || Json(json!({"status": "ng", "errorMessage": "failed to logout"}));

    let database_url = database_url();
    let session_table = PostgresSessionStore::new(&database_url)
        .await
        .map_err(|_| err_json())?;
    match session_table.destroy_session(user_context.session).await {
        Ok(_) => Ok(ok_json()),
        Err(_) => Err(err_json()),
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

