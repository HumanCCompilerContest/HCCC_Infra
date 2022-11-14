use tokio_postgres::Row;

use crate::database::ConnectionPool;
use crate::entities::{Problem, ProblemObject};
use crate::repositories::Problems;

pub struct ProblemImpl<'a> {
    pub pool: &'a ConnectionPool,
}

#[axum::async_trait]
impl<'a> Problems for ProblemImpl<'a> {
    async fn find_problem(&self, id: i32) -> Option<Problem> {
        let conn = self.pool.get().await.unwrap();
        let row = conn
            .query_opt("SELECT * FROM problems WHERE id = $1", &[&id])
            .await
            .unwrap();

        row.map(|r| r.into())
    }

    async fn all_problems(&self) -> Vec<ProblemObject> {
        let conn = self.pool.get().await.unwrap();
        let row = conn.query("SELECT * FROM problems", &[]).await.unwrap();

        row.into_iter().map(|r| r.into()).collect()
    }
}

impl From<Row> for Problem {
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
