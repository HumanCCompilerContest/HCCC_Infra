use serde::Serialize;
use postgres_types::ToSql;

#[allow(non_snake_case)]
#[derive(Serialize)]
#[derive(Debug, ToSql)]
pub struct Rank {
    rank: i32,
    userName: String,
    score: i64,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
#[derive(Debug, ToSql)]
pub struct Ranking {
    status: String,
    #[serde(rename = "items")]
    ranks: Vec<Rank>,
    errorMessage: Option<String>,
}

impl Rank {
    pub fn new(user_name: String, score: i64) -> Self {
        Rank {
            rank: 0,
            userName: user_name,
            score,
        }
    }

    pub fn set_rank(mut self, rank: usize) -> Self {
        self.rank = rank as i32;
        self
    }
}

impl Ranking {
    pub fn new(ranks: Vec<Rank>) -> Self {
        Ranking {
            status: "ok".to_string(),
            ranks,
            errorMessage: None,
        }
    }
}
