use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

use crate::constants::database_url;
use crate::repos_impl::SubmitImpl;

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub async fn new_repo() -> RepositoryProvider {
    let manager = PostgresConnectionManager::new_from_stringlike(database_url(), NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

    RepositoryProvider(pool)
}

#[derive(Clone)]
pub struct RepositoryProvider(ConnectionPool);

impl RepositoryProvider {
    pub fn submit(&self) -> SubmitImpl {
        SubmitImpl { pool: &self.0 }
    }
}
