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
    mod testcase;
    pub use problem::Problem;
    pub use submit::{JudgeResult, Submit};
    pub use testcase::Testcase;
}

mod repos_impl {
    //! Connect to database and load/store data.

    mod problem;
    mod submit;
    mod testcase;
    pub use problem::ProblemImpl;
    pub use submit::SubmitImpl;
    pub use testcase::TestcaseImpl;
}

pub mod repositories {
    //! Definition interface for communicate database.

    mod problem;
    mod submit;
    mod testcase;
    pub use problem::Problems;
    pub use submit::Submits;
    pub use testcase::Testcases;
}
