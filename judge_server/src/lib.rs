mod constants {
    pub fn database_url() -> String {
        dotenv::dotenv().ok();
        std::env::var("DATABASE_URL").unwrap()
    }
}

pub mod database;

pub mod entities {
    mod submit;
    pub use submit::{JudgeResult, Submit};
}

mod repos_impl {
    mod submit;
    pub use submit::SubmitImpl;
}

pub mod repositories {
    pub mod submit;
    pub use submit::Submits;
}
