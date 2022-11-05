use tokio_postgres::Row;

use crate::database::ConnectionPool;
use crate::entities::User;
use crate::repositories::Users;

pub struct UserImpl<'a> {
    pub pool: &'a ConnectionPool,
}

#[axum::async_trait]
impl<'a> Users for UserImpl<'a> {
    async fn find_user(&self, id: i32) -> Option<User> {
        let conn = self.pool.get().await.unwrap();
        let row = conn
            .query_opt("SELECT * FROM accounts WHERE id = $1", &[&id])
            .await
            .unwrap();

        row.map(|r| r.into())
    }

    async fn all_users(&self) -> Vec<User> {
        let conn = self.pool.get().await.unwrap();
        let row = conn
            .query_opt("SELECT * FROM accounts", &[])
            .await
            .unwrap();

        row.into_iter()
            .map(|r| r.into())
            .collect()
    }
}

impl From<Row> for User {
    fn from(r: Row) -> Self {
        User::new(
            "ok".to_string(),
            r.get("id"),
            r.get("name"),
            None,
        )
    }
}



