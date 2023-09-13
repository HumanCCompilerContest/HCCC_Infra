use chrono::{DateTime, Local};
use postgres_types::ToSql;
use serde::Serialize;

/// Data of each user rank.
#[allow(non_snake_case)]
#[derive(Eq, Ord, PartialEq, PartialOrd, Debug, Serialize, ToSql)]
pub struct Rank {
    /// Rank
    rank: i32,
    /// User name.
    userName: String,
    /// Score.
    pub score: i64,
    /// Final accepted submission time.
    pub time: DateTime<Local>,
}

/// Ranking data.
#[allow(non_snake_case)]
#[derive(Serialize, Debug, ToSql)]
pub struct Ranking {
    /// Getting ranking successeed or not.
    /// * `ok` - successeed
    /// * `ng` - failed
    status: String,
    /// Ranks.
    #[serde(rename = "items")]
    ranks: Vec<Rank>,
    /// Error message.
    errorMessage: Option<String>,
}

impl Rank {
    /// Return new `Rank`.
    pub fn new(user_name: String, score: i64, time: DateTime<Local>) -> Self {
        Rank {
            rank: 0,
            userName: user_name,
            score,
            time,
        }
    }

    /// Set given rank to its rank field.
    pub fn set_rank(mut self, rank: usize) -> Self {
        self.rank = i32::try_from(rank).expect("cast error: usize -> i32");
        self
    }
}

impl Ranking {
    /// Return new `Ranking`.
    pub fn new(ranks: Vec<Rank>) -> Self {
        Ranking {
            status: "ok".to_string(),
            ranks,
            errorMessage: None,
        }
    }
}
