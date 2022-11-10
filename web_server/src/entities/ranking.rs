pub struct Rank {
    rank: i32,
    userName: String,
    score: i32,
}

pub struct Ranking {
    status: String,
    ranks: Vec<Rank>,
    errorMessage: Option<String>,
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
