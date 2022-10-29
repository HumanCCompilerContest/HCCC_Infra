use crate::entities::Submit;

#[axum::async_trait]
pub trait Submits {
    async fn get_submit(&self, user_id: u32, submit_id: u32) -> Option<Submit>; 
    async fn list(&self) -> Vec<Submit>;
    async fn store(&self, entity: &Submit);
}

