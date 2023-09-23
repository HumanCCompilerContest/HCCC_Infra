use chrono::{DateTime, Utc};
use postgres_types::{FromSql, ToSql};

/// Judge result  
/// It is provied by exit status of `test_runner`.
#[derive(Debug, Copy, Clone, ToSql, FromSql)]
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

/// Submission
#[derive(Debug, ToSql, FromSql)]
pub struct Submit {
    /// Submit id.
    id: i32,
    /// User id.
    user_id: i32,
    /// Problem id.
    problem_id: i32,
    /// Submit time.
    time: DateTime<Utc>,
    /// Assembly that user submitted.
    pub asm: String,
    /// Error message generated by assembler or linker or runtime.
    error_message: String,
    /// Is compile error submit.
    pub is_ce: bool,
    /// Line number where the first syntax discrepancy occurs.
    pub error_line_number: Option<i32>,
    /// Result of submit.
    result: JudgeResult,
}

impl Submit {
    #[must_use]
    pub fn new(
        id: i32,
        user_id: i32,
        problem_id: i32,
        time: DateTime<Utc>,
        asm: String,
        error_message: String,
        is_ce: bool,
        error_line_number: Option<i32>,
        result: JudgeResult,
    ) -> Self {
        Submit {
            id,
            user_id,
            problem_id,
            time,
            asm,
            error_message,
            is_ce,
            error_line_number,
            result,
        }
    }

    #[must_use]
    pub fn id(&self) -> i32 {
        self.id
    }

    #[must_use]
    pub fn problem_id(&self) -> i32 {
        self.problem_id
    }
}
