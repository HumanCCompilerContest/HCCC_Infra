use serde::Serialize;
use chrono::{DateTime, Local};
use postgres_types::{ToSql, FromSql};
use crate::entities::{UserObject, ProblemObject};

#[derive(Debug, Copy, Clone)]
#[derive(Serialize, ToSql, FromSql)]
#[postgres(name = "judgeresult")]
pub enum JudgeResult {
    AC = 0,
    WA,
    AE,
    LE,
    RE,
    TLE,
    Pending,
    SystemError,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct SubmissionObject {
	id: i32,
	time: DateTime<Local>,
	asm: String,
	result: JudgeResult,
	user: UserObject,
	problem: ProblemObject,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct Submission {
    status: String,
    submission: SubmissionObject,
    errorMessage: Option<String>,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct UserSubmissions {
    status: String,
    #[serde(rename = "items")]
    submissions: Vec<SubmissionObject>,
    errorMessage: Option<String>,
}

impl SubmissionObject {
    pub fn new(
        id: i32,
        time: DateTime<Local>,
        asm: String,
        result: JudgeResult,
        user: UserObject,
        problem: ProblemObject,
    ) -> Self {
        SubmissionObject {
            id,
            time,
            asm,
            result,
            user,
            problem,
        }
    }

    pub fn dummy() -> Self {
        SubmissionObject {
            id: 0,
            time: Local::now(),
            asm: String::new(),
            result: JudgeResult::SystemError,
            user: UserObject::dummy(),
            problem: ProblemObject::dummy(),
        }
    }
}

impl Submission {
    pub fn new(
        id: i32,
        time: DateTime<Local>,
        asm: String,
        result: JudgeResult,
        user: UserObject,
        problem: ProblemObject,
    ) -> Self {
        Submission {
            status: "ok".to_string(),
            submission: SubmissionObject::new(
                id,
                time,
                asm,
                result,
                user,
                problem,
            ),
            errorMessage: None,
        }
    }

    pub fn error() -> Self {
        Submission {
            status: "ng".to_string(),
            submission: SubmissionObject::dummy(),
            errorMessage: Some("couldn't find a user".to_string()),
        }
    }
}

impl UserSubmissions {
    pub fn new(status: String, submissions: Vec<SubmissionObject>, error_message: Option<String>) -> Self {
        UserSubmissions {
            status,
            submissions,
            errorMessage: error_message,
        }
    }

    pub fn error(status: &str, msg: &str) -> Self {
        UserSubmissions {
            status: status.to_string(),
            submissions: Vec::new(),
            errorMessage: Some(msg.to_string()),
        }
    }
}
