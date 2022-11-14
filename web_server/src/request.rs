use async_session::{Session, SessionStore};
use async_sqlx_session::PostgresSessionStore;
use axum::extract::{FromRequest, RequestParts, TypedHeader};
use axum::headers::Cookie;
use axum::response::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::constants::{database_url, AXUM_SESSION_COOKIE_NAME, AXUM_SESSION_USER_ID_KEY};

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

        let database_url = database_url();
        let store = PostgresSessionStore::new(&database_url)
            .await
            .map_err(|_| error_json())?;
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
