use async_session::{Session, SessionStore};
use axum::{
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::Cookie,
    response::Json,
    Extension,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::PgPool;

use crate::constants::{database_url, AXUM_SESSION_COOKIE_NAME, AXUM_SESSION_USER_ID_KEY};

pub type ConnectionPool = PgPool;
pub async fn layer() -> Extension<ConnectionPool> {
    let pool = PgPool::connect(&database_url()).await.unwrap();

    Extension(pool)
}

#[derive(Deserialize, Serialize)]
pub struct UserContext {
    pub session: Session,
}

impl UserContext {
    pub fn user_id(&self) -> i32 {
        self.session.get::<i32>(AXUM_SESSION_USER_ID_KEY).unwrap()
    }
}

#[axum::async_trait]
impl<B> FromRequest<B> for UserContext
where
    B: Send,
{
    type Rejection = Json<serde_json::Value>;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let error_json =
            || Json(json!({"status": "login-required", "errorMessage": "session expired"}));

        let Extension(pool): Extension<ConnectionPool> =
            Extension::<ConnectionPool>::from_request(req)
                .await
                .expect("`SessionStore` extension missing");
        let store = async_sqlx_session::PostgresSessionStore::from_client(pool);
        let cookies = Option::<TypedHeader<Cookie>>::from_request(req)
            .await
            .unwrap()
            .ok_or(error_json())?;
        let session_str = cookies.get(AXUM_SESSION_COOKIE_NAME).ok_or(error_json())?;
        let session = store
            .load_session(session_str.to_string())
            .await
            .map_err(|_| error_json())?;
        let session = session.ok_or(error_json())?;
        let context = UserContext { session };

        Ok(context)
    }
}
