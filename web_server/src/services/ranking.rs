use crate::entities::Ranking;
use crate::repositories::Users;

/// Get a ranking.
pub async fn get_ranking(repo: &impl Users) -> Ranking {
    let ranks = repo.create_ranking().await;
    Ranking::new(ranks)
}
