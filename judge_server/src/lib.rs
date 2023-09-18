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

    mod problem;
    mod submit;
    pub use problem::Problem;
    pub use submit::{JudgeResult, Submit};
}

mod repos_impl {
    //! Connect to database and load/store data.

    mod problem;
    mod submit;
    pub use problem::ProblemImpl;
    pub use submit::SubmitImpl;
}

pub mod repositories {
    //! Definition interface for communicate database.

    mod problem;
    pub mod submit;
    pub use problem::Problems;
    pub use submit::Submits;
}
