use async_session::{Session, SessionStore};
use async_sqlx_session::PostgresSessionStore;
use std::time::Duration;

use crate::constants::{database_url, AXUM_SESSION_COOKIE_NAME, AXUM_SESSION_USER_ID_KEY};
use crate::entities::Account;
use crate::repositories::Accounts;

pub async fn create_account(
    repo: &impl Accounts,
    user_name: &str,
    password: &str,
) -> Result<u64, tokio_postgres::Error> {
    let new_account = Account::create(user_name, password);
    repo.store(&new_account).await
}

pub async fn create_session(
    repo: &impl Accounts,
    user_name: &str,
    password: &str,
) -> (Option<i32>, Option<SessionToken>) {
    let account = repo.find_by(user_name).await;
    if let Some(account) = account {
        if !account.matches_password(password) {
            return (None, None);
        }

        let database_url = database_url();
        let store = PostgresSessionStore::new(&database_url).await.unwrap();

        let mut session = Session::new();
        session
            .insert(AXUM_SESSION_USER_ID_KEY, account.id().unwrap())
            .unwrap();
        session.expire_in(Duration::from_secs(804800));

        let cookie = store.store_session(session).await.unwrap().unwrap();

        (account.id(), Some(SessionToken(cookie)))
    } else {
        (None, None)
    }
}

pub struct SessionToken(String);

impl SessionToken {
    pub fn cookie(&self) -> String {
        format!(
            "{}={}; Max-Age=604800; Path=/; HttpOnly",
            AXUM_SESSION_COOKIE_NAME, &self.0
        )
    }
}
