use tokio_postgres::Row;

use crate::database::ConnectionPool;
use crate::entities::{Rank, User, UserObject};
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

    async fn all_users(&self) -> Vec<UserObject> {
        let conn = self.pool.get().await.unwrap();
        let row = conn.query("SELECT * FROM accounts", &[]).await.unwrap();

        row.into_iter().map(|r| r.into()).collect()
    }

    async fn create_ranking(&self) -> Vec<Rank> {
        let conn = self.pool.get().await.unwrap();
        let row = conn
            .query(
                "SELECT sub.name AS name, SUM(sub.score) AS score, MAX(sub.min_time) AS max_time
                FROM
                (
                    SELECT a.name AS name, s.problem_id AS problem_id, p.score AS score, MIN(s.time) AS min_time FROM submits AS s 
                    JOIN accounts AS a ON s.user_id = a.id
                    JOIN problems AS p ON s.problem_id = p.id
                    WHERE s.result = 'AC'
                    GROUP BY a.name, s.problem_id, p.score
                ) AS sub
                GROUP BY sub.name
                ;",
                &[]
            )
            .await
            .unwrap();

        row.into_iter()
            .enumerate()
            .map(|(rank, r)| <tokio_postgres::Row as Into<Rank>>::into(r).set_rank(rank + 1))
            .collect()
    }
}

impl From<Row> for User {
    fn from(r: Row) -> Self {
        User::new("ok".to_string(), r.get("id"), r.get("name"), None)
    }
}

impl From<Row> for UserObject {
    fn from(r: Row) -> Self {
        UserObject::new(r.get("id"), r.get("name"))
    }
}

impl From<Row> for Rank {
    fn from(r: Row) -> Self {
        Rank::new(
            r.get("name"),
            r.get("sum"), // sum of score
        )
    }
}
