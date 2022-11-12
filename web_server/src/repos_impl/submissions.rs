use tokio_postgres::Row;
use chrono::{DateTime, Local};

use crate::database::ConnectionPool;
use crate::entities::{Submission, SubmissionObject, JudgeResult};
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

    async fn store_submission<'b>(
        &self,
        user_id: i32,
        problem_id: i32,
        submit_time: DateTime<Local>,
        asem: &'b str,
        judge_result: JudgeResult,
    ) -> Option<i32> {
        let conn = self.pool.get().await.unwrap();
        let row = conn
            .query_opt(
                "INSERT INTO submits (user_id, problem_id, time, asem, result) VALUES ($1, $2, $3, $4, $5) RETURNING id",
                &[&user_id, &problem_id, &submit_time, &asem, &judge_result]
            )
            .await
            .unwrap();

        row.map(|r| r.get("id"))
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




