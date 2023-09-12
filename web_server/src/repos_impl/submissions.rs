use chrono::{DateTime, Local};
use tokio_postgres::Row;

use crate::database::ConnectionPool;
use crate::entities::{JudgeResult, ProblemObject, Submission, SubmissionObject, UserObject};
use crate::repositories::Submissions;

/// Implementation for `Submissions`.
pub struct SubmissionImpl<'a> {
    pub pool: &'a ConnectionPool,
}

#[axum::async_trait]
impl<'a> Submissions for SubmissionImpl<'a> {
    /// Find a submittion by id.
    async fn find_submission(&self, id: i32) -> Option<Submission> {
        let conn = self.pool.get().await.unwrap();
        let row = conn
            .query_opt("SELECT * FROM submits JOIN accounts ON submits.user_id = accounts.id JOIN problems ON submits.problem_id = problems.id WHERE submits.id = $1", &[&id])
            .await
            .unwrap();

        row.map(std::convert::Into::into)
    }

    /// Store a submission.
    async fn store_submission<'b>(
        &self,
        user_id: i32,
        problem_id: i32,
        submit_time: DateTime<Local>,
        asm: &'b str,
        is_ce: bool,
        judge_result: JudgeResult,
    ) -> Option<i32> {
        let conn = self.pool.get().await.unwrap();
        let row = conn
            .query_opt(
                "INSERT INTO submits (user_id, problem_id, time, asm, result, is_ce) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id",
                &[&user_id, &problem_id, &submit_time, &asm, &judge_result, &is_ce]
            )
            .await
            .unwrap();

        row.map(|r| r.get("id"))
    }

    /// Get all submissions.
    async fn get_all_submissions(&self) -> Vec<SubmissionObject> {
        const TARGET_COLUMN: &str = "submits.id, time, asm, is_ce, result, user_id, name, problem_id, title, statement, code, input_desc, output_desc, problems.score";
        const TARGET_TABLES: &str = "submits JOIN accounts ON submits.user_id = accounts.id JOIN problems ON submits.problem_id = problems.id";
        let conn = self.pool.get().await.unwrap();
        let row = conn
            .query(
                &format!("SELECT {TARGET_COLUMN} FROM {TARGET_TABLES} ORDER BY time DESC"),
                &[],
            )
            .await
            .unwrap();

        row.into_iter().map(std::convert::Into::into).collect()
    }

    /// Get all submissions that specified user submitted.
    async fn user_submitted(&self, user_id: i32) -> Vec<SubmissionObject> {
        const TARGET_COLUMN: &str = "submits.id, time, asm, is_ce, result, user_id, name, problem_id, title, statement, code, input_desc, output_desc, problems.score";
        const TARGET_TABLES: &str = "submits JOIN accounts ON submits.user_id = accounts.id JOIN problems ON submits.problem_id = problems.id";
        let conn = self.pool.get().await.unwrap();
        let row = conn
            .query(
                &format!(
                    "SELECT {TARGET_COLUMN} FROM {TARGET_TABLES} WHERE user_id = $1 ORDER BY time DESC"
                ),
                &[&user_id],
            )
            .await
            .unwrap();

        row.into_iter().map(std::convert::Into::into).collect()
    }
}

impl From<Row> for Submission {
    /// Convert SQL result to `Submission`.
    fn from(r: Row) -> Self {
        Submission::new(
            r.get("id"),
            r.get("time"),
            r.get("asm"),
            r.get("is_ce"),
            r.get("result"),
            UserObject::new(r.get("user_id"), r.get("name")),
            ProblemObject::new(
                r.get("problem_id"),
                r.get("title"),
                r.get("statement"),
                r.get("code"),
                r.get("input_desc"),
                r.get("output_desc"),
                r.get("score"),
            ),
        )
    }
}

impl From<Row> for SubmissionObject {
    /// Convert SQL result to `SubmissionObject`.
    fn from(r: Row) -> Self {
        SubmissionObject::new(
            r.get("id"),
            r.get("time"),
            r.get("asm"),
            r.get("is_ce"),
            r.get("result"),
            UserObject::new(r.get("user_id"), r.get("name")),
            ProblemObject::new(
                r.get("problem_id"),
                r.get("title"),
                r.get("statement"),
                r.get("code"),
                r.get("input_desc"),
                r.get("output_desc"),
                r.get("score"),
            ),
        )
    }
}
