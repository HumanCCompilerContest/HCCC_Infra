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
        let ac = conn
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
        let not_ac = conn
            .query(
                "SELECT a.name AS name, COUNT(*) AS wrong_count FROM submits AS s 
                    JOIN accounts AS a ON s.user_id = a.id
                    WHERE s.result != 'WC'
                    AND s.result != 'AC'
                    GROUP BY a.name
                    ORDER BY a.name
                ;",
                &[],
            )
            .await
            .unwrap();
        let wc = conn
            .query(
                "SELECT a.name AS name, COUNT(*) AS wc_count FROM submits AS s 
                    JOIN accounts AS a ON s.user_id = a.id
                    WHERE s.result = 'WC'
                    GROUP BY a.name
                    ORDER BY a.name
                ;",
                &[],
            )
            .await
            .unwrap();

        let mut ranking = ac
            .iter()
            .zip(not_ac.iter())
            .zip(wc.iter())
            .map(|((x, y), z)| {
                Rank::new(
                    x.get("name"),
                    std::cmp::max(
                        0,
                        x.get::<&str, i64>("score")
                            - y.get::<&str, i64>("wrong_count")
                            - z.get::<&str, i64>("wc_count") * 100,
                    ),
                )
            })
            .collect::<Vec<Rank>>();

        ranking.sort();
        ranking
            .into_iter()
            .enumerate()
            .map(|(rank, r)| r.set_rank(rank + 1))
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
