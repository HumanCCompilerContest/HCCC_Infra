use crate::entities::Problem;
/// A trait for database connection.
#[axum::async_trait]
pub trait Problems {
    async fn get_all_problems(&self) -> Vec<Problem>;
}
