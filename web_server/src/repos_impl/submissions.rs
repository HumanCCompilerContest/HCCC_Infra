use tokio_postgres::Row;

use crate::database::ConnectionPool;
use crate::entities::{Submission, SubmissionObject};
use crate::repositories::Submissions;

pub struct SubmissionImpl<'a> {
    pub pool: &'a ConnectionPool,
}

#[axum::async_trait]
impl<'a> Submissions for SubmissionImpl<'a> {
    async fn find_submission(&self, id: i32) -> Option<Submission> {
        let conn = self.pool.get().await.unwrap();
        let row = conn
            .query_opt("SELECT * FROM submits WHERE id = $1", &[&id])
            .await
            .unwrap();

        row.map(|r| r.into())
    }

    async fn user_submitted(&self, user_id: i32) -> Vec<SubmissionObject> {
        let conn = self.pool.get().await.unwrap();
        let row = conn
            .query_opt("SELECT * FROM submits WHERE user_id", &[&user_id])
            .await
            .unwrap();

        row.into_iter()
            .map(|r| r.into())
            .collect()
    }
}

impl From<Row> for Submission {
    fn from(r: Row) -> Self {
        Submission::new(
            r.get("id"),
            r.get("title"),
            r.get("asem"),
            r.get("result"),
            r.get("user"),
            r.get("problem"),
        )
    }
}

impl From<Row> for SubmissionObject {
    fn from(r: Row) -> Self {
        SubmissionObject::new(
            r.get("id"),
            r.get("title"),
            r.get("asem"),
            r.get("result"),
            r.get("user"),
            r.get("problem"),
        )
    }
}




