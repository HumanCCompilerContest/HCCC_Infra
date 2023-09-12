use crate::entities::{ProblemObject, UserObject};
use chrono::{DateTime, Local};
use postgres_types::{FromSql, ToSql};
use serde::Serialize;

/// Judge result  
/// It is provied by exit status of `test_runner` and `judge_server`.
#[derive(Debug, Copy, Clone, Serialize, ToSql, FromSql)]
#[postgres(name = "judgeresult")]
pub enum JudgeResult {
    /// ACcepted
    AC = 0,
    /// Wrong Answer
    WA,
    /// Wrong Compile Error
    WC,
    /// Assembly Error
    AE,
    /// Linker Error
    LE,
    /// Runtime Error
    RE,
    /// Time Limit Exceeded
    TLE,
    /// the submit is pending.
    Pending,
    /// The judge failed due to system error.
    SystemError,
}

/// Submission data
#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct SubmissionObject {
    /// Submission id.
    id: i32,
    /// Submission time.
    time: DateTime<Local>,
    /// Submitted assembly.
    asm: String,
    /// Did submited as `compile error`.
    is_ce: bool,
    /// Judge result.
    result: JudgeResult,
    /// Submitted user.
    user: UserObject,
    /// Problem.
    problem: ProblemObject,
}

/// Getting submission result
#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct Submission {
    /// Getting submission successeed or not.
    /// * `ok` - successeed
    /// * `ng` - failed
    status: String,
    /// Submissions
    submission: SubmissionObject,
    /// Error message.
    errorMessage: Option<String>,
}

/// Getting submission result submitted by specified user.
#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct UserSubmissions {
    /// Getting submission that submitted by specified user successeed or not.
    /// * `ok` - successeed
    /// * `ng` - failed
    status: String,
    /// Submissions
    #[serde(rename = "items")]
    submissions: Vec<SubmissionObject>,
    /// Error message.
    errorMessage: Option<String>,
}

impl SubmissionObject {
    /// Return new `SubmissionObject`.
    pub fn new(
        id: i32,
        time: DateTime<Local>,
        asm: String,
        is_ce: bool,
        result: JudgeResult,
        user: UserObject,
        problem: ProblemObject,
    ) -> Self {
        SubmissionObject {
            id,
            time,
            asm,
            is_ce,
            result,
            user,
            problem,
        }
    }

    /// Return dummy `SubmissionObject`.
    pub fn dummy() -> Self {
        SubmissionObject {
            id: 0,
            time: Local::now(),
            asm: String::new(),
            is_ce: false,
            result: JudgeResult::SystemError,
            user: UserObject::dummy(),
            problem: ProblemObject::dummy(),
        }
    }
}

impl Submission {
    /// Return successeed response.
    pub fn new(
        id: i32,
        time: DateTime<Local>,
        asm: String,
        is_ce: bool,
        result: JudgeResult,
        user: UserObject,
        problem: ProblemObject,
    ) -> Self {
        Submission {
            status: "ok".to_string(),
            submission: SubmissionObject::new(id, time, asm, is_ce, result, user, problem),
            errorMessage: None,
        }
    }

    /// Return error response.
    pub fn error() -> Self {
        Submission {
            status: "ng".to_string(),
            submission: SubmissionObject::dummy(),
            errorMessage: Some("couldn't find a user".to_string()),
        }
    }
}

impl UserSubmissions {
    /// Return successeed response.
    pub fn new(
        status: String,
        submissions: Vec<SubmissionObject>,
        error_message: Option<String>,
    ) -> Self {
        UserSubmissions {
            status,
            submissions,
            errorMessage: error_message,
        }
    }

    /// Return error response.
    pub fn error(status: &str, msg: &str) -> Self {
        UserSubmissions {
            status: status.to_string(),
            submissions: Vec::new(),
            errorMessage: Some(msg.to_string()),
        }
    }
}
