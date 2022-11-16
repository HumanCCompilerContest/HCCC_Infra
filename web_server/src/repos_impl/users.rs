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
                "SELECT a_outer.name AS name, coalesce(SUM(sub.score),0) AS score, MAX(sub.min_time) AS max_time
                FROM
                (
                    SELECT a.name AS name, s.problem_id AS problem_id, p.score AS score, MIN(s.time) AS min_time FROM submits AS s 
                    JOIN accounts AS a ON s.user_id = a.id
                    JOIN problems AS p ON s.problem_id = p.id
                    WHERE s.result = 'AC'
                    GROUP BY a.name, s.problem_id, p.score
                ) AS sub
                RIGHT JOIN accounts AS a_outer ON sub.name = a_outer.name
                GROUP BY a_outer.name
                ORDER BY a_outer.name
                ;",
                &[]
            )
            .await
            .unwrap();
        let not_ac = conn
            .query(
                "SELECT a.name AS name, COUNT(*) AS wrong_count FROM submits AS s
                JOIN (
                SELECT user_id, problem_id FROM submits
                    WHERE result = 'AC'
                    GROUP BY user_id, problem_id
                ) AS sub ON s.user_id = sub.user_id AND s.problem_id = sub.problem_id
                JOIN accounts AS a ON s.user_id = a.id
                WHERE s.result != 'AC' AND s.result != 'WC'
                GROUP BY a.name
                ORDER BY a.name
                ;",
                &[],
            )
            .await
            .unwrap();

        // return null when wc is null
        let mut ranking = dbg!(ac)
            .iter()
            .map(|x| {
                let name = x.get("name");
                let nac = not_ac
                    .iter()
                    .find(|y| y.get::<&str, String>("name") == name)
                    .map(|y| y.get("wrong_count"))
                    .unwrap_or(0);

                Rank::new(name, std::cmp::max(0, x.get::<&str, i64>("score") - nac))
            })
            .collect::<Vec<Rank>>();

        ranking.sort_by(|x, y| (-x.score).cmp(&-y.score));
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
