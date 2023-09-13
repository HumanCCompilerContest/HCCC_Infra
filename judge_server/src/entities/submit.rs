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
    id: i32,
    pub user_id: i32,
    pub problem_id: i32,
    pub time: DateTime<Utc>,
    pub asm: String,
    pub is_ce: bool,
    pub result: JudgeResult,
}

impl Submit {
    #[must_use]
    pub fn new(
        id: i32,
        user_id: i32,
        problem_id: i32,
        time: DateTime<Utc>,
        asm: String,
        is_ce: bool,
        result: JudgeResult,
    ) -> Self {
        Submit {
            id,
            user_id,
            problem_id,
            time,
            asm,
            is_ce,
            result,
        }
    }

    #[must_use]
    pub fn id(&self) -> i32 {
        self.id
    }
}
