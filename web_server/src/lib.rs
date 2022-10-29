mod controllers {
    mod root;
    mod submit;
    pub use root::app;
    pub use submit::submit;
}

mod database;

mod entities {
    mod submit;
    pub use submit::Submit;
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
    pub use submit::{create_submit, list_submit};
}

mod response;

mod views {
    mod home;
    mod partial {
        mod submit;
        pub use submit::Submit;
    }
    pub use home::Home;
    pub use partial::Submit;
}
pub use controllers::app;
