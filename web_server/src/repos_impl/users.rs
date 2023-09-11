use tokio_postgres::Row;

use crate::constants::contest_duration;
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

        row.map(std::convert::Into::into)
    }

    async fn all_users(&self) -> Vec<UserObject> {
        let conn = self.pool.get().await.unwrap();
        let row = conn.query("SELECT * FROM accounts", &[]).await.unwrap();

        row.into_iter().map(std::convert::Into::into).collect()
    }

    async fn create_ranking(&self) -> Vec<Rank> {
        let (start, end) = contest_duration();
        let conn = self.pool.get().await.unwrap();
        let ac = conn
            .query(
                "SELECT a_outer.name AS name, coalesce(SUM(sub.score), 0) AS score, coalesce(MAX(sub.min_time), NOW()) AS max_time
                FROM
                (
                    SELECT a.name AS name, s.problem_id AS problem_id, p.score AS score, MIN(s.time) AS min_time FROM submits AS s 
                    JOIN accounts AS a ON s.user_id = a.id
                    JOIN problems AS p ON s.problem_id = p.id
                    WHERE s.result = 'AC'
                    AND $1 <= s.time 
                    AND s.time <= $2
                    GROUP BY a.name, s.problem_id, p.score
                ) AS sub
                RIGHT JOIN accounts AS a_outer ON sub.name = a_outer.name
                GROUP BY a_outer.name
                ORDER BY a_outer.name
                ;",
                &[&start, &end]
            )
            .await
            .unwrap();

        let not_ac = conn
            .query(
                "SELECT a.name AS name, COUNT((s.result != 'AC' AND s.result != 'WC') or NULL) AS wrong_count FROM submits AS s
                JOIN (
                SELECT user_id, problem_id FROM submits
                    WHERE result = 'AC'
                    AND $1 <= time 
                    AND time <= $2
                    GROUP BY user_id, problem_id
                ) AS sub ON s.user_id = sub.user_id AND s.problem_id = sub.problem_id
                RIGHT JOIN accounts AS a ON s.user_id = a.id
                GROUP BY a.name
                ORDER BY a.name
                ;",
                &[&start, &end]
            )
            .await
            .unwrap();

        // return null when wc is null
        let mut ranking = ac
            .iter()
            .zip(not_ac.iter())
            .map(|(x, y)| {
                Rank::new(
                    x.get("name"),
                    std::cmp::max(
                        0,
                        x.get::<&str, i64>("score") - y.get::<&str, i64>("wrong_count"),
                    ),
                    x.get("max_time"),
                )
            })
            .collect::<Vec<Rank>>();

        ranking.sort_by(|x, y| match (-x.score).cmp(&-y.score) {
            std::cmp::Ordering::Equal => x.time.cmp(&y.time),
            other => other,
        });
        dbg!(ranking)
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
