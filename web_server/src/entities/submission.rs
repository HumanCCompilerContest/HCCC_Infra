use serde::Serialize;
use chrono::{DateTime, Local};
use postgres_types::{ToSql, FromSql};
use crate::entities::Problem;
use crate::entities::User;

#[derive(Debug, Copy, Clone, ToSql, FromSql)]
#[postgres(name = "judgeresult")]
pub enum JudgeResult {
    Pending,
    AC,
    WA,
    AE,
    LE,
    TLE,
    SystemError,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct SubmissionObject {
	id: i32,
	time: DateTime<Local>,
	asem: String,
	result: String,
	user: User,
	problem: Problem,
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
        asem: String,
        result: String,
        user: User,
        problem: Problem,
    ) -> Self {
        SubmissionObject {
            id,
            time,
            asem,
            result,
            user,
            problem,
        }
    }

    pub fn dummy() -> Self {
        SubmissionObject {
            id: 0,
            time: Local::now(),
            asem: String::new(),
            result: "SystemError".to_string(),
            user: User::error("couldn't find a user"),
            problem: Problem::error(),
        }
    }
}

impl Submission {
    pub fn new(
        id: i32,
        time: DateTime<Local>,
        asem: String,
        result: String,
        user: User,
        problem: Problem,
    ) -> Self {
        Submission {
            status: "ok".to_string(),
            submission: SubmissionObject::new(
                id,
                time,
                asem,
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
}
