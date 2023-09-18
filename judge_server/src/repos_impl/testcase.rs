use tokio_postgres::Row;

use crate::database::ConnectionPool;
use crate::entities::Testcase;
use crate::repositories::Testcases;

/// A struct for database connection.
pub struct TestcaseImpl<'a> {
    pub pool: &'a ConnectionPool,
}

#[axum::async_trait]
impl<'a> Testcases for TestcaseImpl<'a> {
    /// Get pending submits from database.
    async fn get_all_testcases(&self) -> Vec<Testcase> {
        let conn = self.pool.get().await.unwrap();
        conn.query("SELECT * FROM testcases", &[])
            .await
            .unwrap()
            .into_iter()
            .map(std::convert::Into::into)
            .collect()
    }
}

impl From<Row> for Testcase {
    /// Convert SQL output to `Testcase`.
    fn from(r: Row) -> Self {
        Testcase::new(
            r.get("id"),
            r.get("problem_id"),
            r.get("input"),
            r.get("expect"),
        )
    }
}
