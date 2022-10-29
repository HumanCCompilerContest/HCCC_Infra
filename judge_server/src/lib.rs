mod controllers {
    mod root;
    mod submit;
    pub use root::app;
    pub use submit::submit;
}

mod database;

mod entities {
    mod submit;
    pub use submit::{Submit, JudgeResult};
}

mod repos_impl {
    mod submit;
    pub use submit::SubmitImpl;
}

mod repositories {
    mod submit;
    pub use submit::Submits;
}

mod services {
    mod submit;
    pub use submit::accept_submit;
}

pub use controllers::app;
