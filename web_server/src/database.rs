use axum::Extension;
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

use crate::constants::database_url;
use crate::repos_impl::{AccountsImpl, UserImpl, ProblemImpl, SubmissionImpl};

pub type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

pub async fn layer() -> Extension<RepositoryProvider> {
    let manager = PostgresConnectionManager::new_from_stringlike(database_url(), NoTls).unwrap();
    let pool = Pool::builder().build(manager).await.unwrap();

    Extension(RepositoryProvider(pool))
}

#[derive(Clone)]
pub struct RepositoryProvider(ConnectionPool);

impl RepositoryProvider {
    pub fn accounts(&self) -> AccountsImpl {
        AccountsImpl { pool: &self.0 }
    }

    pub fn user(&self) -> UserImpl {
        UserImpl { pool: &self.0 }
    }

    pub fn problem(&self) -> ProblemImpl {
        ProblemImpl { pool: &self.0 }
    }

    pub fn submission(&self) -> SubmissionImpl {
        SubmissionImpl { pool: &self.0 }
    }
}
