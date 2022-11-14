pub mod database;

pub mod entities {
    mod submit;
    pub use submit::{Submit, JudgeResult};
}

mod repos_impl {
    mod submit;
    pub use submit::SubmitImpl;
}

pub mod repositories {
    pub mod submit;
    pub use submit::Submits;
}
