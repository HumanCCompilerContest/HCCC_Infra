use crate::entities::{Ranking};
use crate::repositories::Users;

pub async fn get_ranking(repo: &impl Users) -> Ranking {
    repo.create_ranking()
       .await
       .unwrap_or(Ranking::error())
}
