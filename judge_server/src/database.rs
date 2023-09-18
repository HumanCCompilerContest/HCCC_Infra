//! Set up for database connection.

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

use crate::constants::database_url;
use crate::repos_impl::{ProblemImpl, SubmitImpl};

/// Connection pool of postgres
pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

#[derive(Clone)]
pub struct RepositoryProvider(ConnectionPool);

impl RepositoryProvider {
    /// Setup connection pool.
    ///
    /// # Panics
    /// It will panic when fail to build connection pool.
    #[must_use]
    pub async fn new() -> Self {
        let manager =
            PostgresConnectionManager::new_from_stringlike(database_url(), NoTls).unwrap();
        let pool = Pool::builder().build(manager).await.unwrap();

        RepositoryProvider(pool)
    }

    #[must_use]
    pub fn submit(&self) -> SubmitImpl {
        SubmitImpl { pool: &self.0 }
    }

    #[must_use]
    pub fn problem(&self) -> ProblemImpl {
        ProblemImpl { pool: &self.0 }
    }
}
