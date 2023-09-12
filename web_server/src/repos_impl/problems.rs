use tokio_postgres::Row;

use crate::database::ConnectionPool;
use crate::entities::{Problem, ProblemObject};
use crate::repositories::Problems;

/// Implementation for `Problems`.
pub struct ProblemImpl<'a> {
    pub pool: &'a ConnectionPool,
}

#[axum::async_trait]
impl<'a> Problems for ProblemImpl<'a> {
    /// Find a problem by id.
    async fn find_problem(&self, id: i32) -> Option<Problem> {
        let conn = self.pool.get().await.unwrap();
        let row = conn
            .query_opt("SELECT * FROM problems WHERE id = $1", &[&id])
            .await
            .unwrap();

        row.map(std::convert::Into::into)
    }

    /// Get the all problems.
    async fn all_problems(&self) -> Vec<ProblemObject> {
        let conn = self.pool.get().await.unwrap();
        let row = conn
            .query("SELECT * FROM problems ORDER BY id", &[])
            .await
            .unwrap();

        row.into_iter().map(std::convert::Into::into).collect()
    }
}

impl From<Row> for Problem {
    /// Convert SQL result to `Problem`.
    fn from(r: Row) -> Self {
        Problem::new(
            r.get("id"),
            r.get("title"),
            r.get("statement"),
            r.get("code"),
            r.get("input_desc"),
            r.get("output_desc"),
            r.get("score"),
        )
    }
}

impl From<Row> for ProblemObject {
    /// Convert SQL result to `ProblemObject`.
    fn from(r: Row) -> Self {
        ProblemObject::new(
            r.get("id"),
            r.get("title"),
            r.get("statement"),
            r.get("code"),
            r.get("input_desc"),
            r.get("output_desc"),
            r.get("score"),
        )
    }
}
