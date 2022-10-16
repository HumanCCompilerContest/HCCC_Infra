use crate::entities::Submit;

#[axum::async_trait]
pub trait Submits {
    async fn find(&self, id: i32) -> Option<Submit>;
    async fn list(&self) -> Vec<Submit>;
    async fn store(&self, entity: &Submit);
}

