use chrono::{DateTime, Utc};
use postgres_types::{FromSql, ToSql};

#[derive(Debug, Copy, Clone, ToSql, FromSql)]
#[postgres(name = "judgeresult")]
pub enum JudgeResult {
    AC = 0,
    WA,
    WC,
    AE,
    LE,
    RE,
    TLE,
    Pending,
    SystemError,
}

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

    pub fn id(&self) -> i32 {
        self.id
    }
}
