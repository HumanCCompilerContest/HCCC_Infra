use tokio_postgres::Row;

use crate::database::ConnectionPool;
use crate::entities::{JudgeResult, Submit};
use crate::repositories::Submits;

pub struct SubmitImpl<'a> {
    pub pool: &'a ConnectionPool,
}

#[axum::async_trait]
impl<'a> Submits for SubmitImpl<'a> {
    async fn get_pending_submits(&self) -> Vec<Submit> {
        let conn = self.pool.get().await.unwrap();
        conn.query(
            "SELECT * FROM submits WHERE result = 'Pending' ORDER BY time DESC",
            &[],
        )
        .await
        .unwrap()
        .into_iter()
        .map(|r| r.into())
        .collect()
    }

    async fn store_result(&self, result: JudgeResult, submit_id: i32) {
        let conn = self.pool.get().await.unwrap();
        conn.query_opt(
            "UPDATE submits set result = $1 WHERE id = $2",
            &[&result, &submit_id],
        )
        .await
        .unwrap();
    }
}

impl From<Row> for Submit {
    fn from(r: Row) -> Self {
        Submit::new(
            r.get("id"),
            r.get("user_id"),
            r.get("problem_id"),
            r.get("time"),
            r.get("asm"),
            r.get("is_ce"),
            r.get("result"),
        )
    }
}
