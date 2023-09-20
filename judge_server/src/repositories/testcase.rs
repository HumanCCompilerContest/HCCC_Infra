use crate::entities::Testcase;

/// A trait for database connection.
#[axum::async_trait]
pub trait Testcases {
    async fn get_all_testcases(&self, num_of_testcases: u32) -> Vec<Vec<Testcase>>;
}
