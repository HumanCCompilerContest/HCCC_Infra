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
    async fn get_all_testcases(&self, num_of_testcases: u32) -> Vec<Vec<Testcase>> {
        let conn = self.pool.get().await.unwrap();
        let mut testcases = Vec::new();
        for problem_id in 0..num_of_testcases {
            testcases.push(
                conn.query(
                    "SELECT * FROM testcases WHERE problem_id = $1",
                    &[&(problem_id as i32)],
                )
                .await
                .unwrap()
                .into_iter()
                .map(std::convert::Into::into)
                .collect(),
            )
        }

        testcases
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
