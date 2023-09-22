use tokio_postgres::Row;

use crate::database::ConnectionPool;
use crate::entities::{JudgeResult, Submit};
use crate::repositories::Submits;

/// A struct for database connection.
pub struct SubmitImpl<'a> {
    pub pool: &'a ConnectionPool,
}

#[axum::async_trait]
impl<'a> Submits for SubmitImpl<'a> {
    /// Get pending submits from database.
    async fn get_pending_submits(&self) -> Vec<Submit> {
        let conn = self.pool.get().await.unwrap();
        conn.query(
            "SELECT * FROM submits WHERE result = 'Pending' ORDER BY time DESC",
            &[],
        )
        .await
        .unwrap()
        .into_iter()
        .map(std::convert::Into::into)
        .collect()
    }

    /// Store Judged result.
    async fn store_result(&self, result: JudgeResult, error_message: String, submit_id: i32) {
        let conn = self.pool.get().await.unwrap();
        conn.query_opt(
            "UPDATE submits set result = $1, error_message = $2 WHERE id = $3",
            &[&result, &error_message, &submit_id],
        )
        .await
        .unwrap();
    }
}

impl From<Row> for Submit {
    /// Convert SQL output to `Submit`.
    fn from(r: Row) -> Self {
        Submit::new(
            r.get("id"),
            r.get("user_id"),
            r.get("problem_id"),
            r.get("time"),
            r.get("asm"),
            r.get("error_message"),
            r.get("is_ce"),
            r.get("error_line_number"),
            r.get("result"),
        )
    }
}
