use chrono::{DateTime, Utc};
use postgres_types::{ToSql, FromSql};

#[derive(Debug, ToSql, FromSql)]
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

pub struct Submit {
    id: i32,
    pub user_id: i32,
    pub time: DateTime<Utc>,
    pub asem: String,
    pub result: JudgeResult,
}

impl Submit {
    pub fn new(id: i32, user_id: i32, time: DateTime<Utc>, asem: String, result: JudgeResult) -> Submit {
        Submit {
            id,
            user_id,
            time,
            asem,
            result,
        }
    }

    pub fn create(user_id: i32, asem: &str) -> Submit {
        Submit { 
            id: 0,
            user_id,
            time: Utc::now(),
            asem: asem.to_string(),
            result: JudgeResult::Pending,
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}


