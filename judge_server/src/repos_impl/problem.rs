use tokio_postgres::Row;

use crate::database::ConnectionPool;
use crate::entities::Problem;
use crate::repositories::Problems;

/// A struct for database connection.
pub struct ProblemImpl<'a> {
    pub pool: &'a ConnectionPool,
}

#[axum::async_trait]
impl<'a> Problems for ProblemImpl<'a> {
    /// Get pending submits from database.
    async fn get_all_problems(&self) -> Vec<Problem> {
        let conn = self.pool.get().await.unwrap();
        conn.query(
            "SELECT id, test_target, is_wrong_code, error_line_number FROM problems",
            &[],
        )
        .await
        .unwrap()
        .into_iter()
        .map(std::convert::Into::into)
        .collect()
    }
}

impl From<Row> for Problem {
    /// Convert SQL output to `Problem`.
    fn from(r: Row) -> Self {
        Problem::new(
            r.get("id"),
            r.get("arch"),
            r.get("test_target"),
            r.get("is_wrong_code"),
            r.get("error_line_number"),
        )
    }
}
