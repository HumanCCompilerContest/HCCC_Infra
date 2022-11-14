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
