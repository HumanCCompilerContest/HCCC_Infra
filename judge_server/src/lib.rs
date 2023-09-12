mod constants {
    /// Get database url from dotenv.
    pub fn database_url() -> String {
        dotenv::dotenv().ok();
        std::env::var("DATABASE_URL").unwrap()
    }
}

pub mod database;

pub mod entities {
    //! Definition deta structure.

    mod submit;
    pub use submit::{JudgeResult, Submit};
}

mod repos_impl {
    //! Connect to database and load/store data.

    mod submit;
    pub use submit::SubmitImpl;
}

pub mod repositories {
    //! Definition interface for communicate database.

    pub mod submit;
    pub use submit::Submits;
}
