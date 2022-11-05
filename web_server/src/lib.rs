mod constants {
    use std::env;

    pub const AXUM_SESSION_COOKIE_NAME: &str = "hccc_session";
    pub const AXUM_SESSION_USER_ID_KEY: &str = "uid";

    pub fn database_url() -> String {
        dotenv::dotenv().ok();
        env::var("DATABASE_URL").unwrap()
    }
}
mod controllers {
    mod accounts;
    mod root;
    mod submits;
    mod user;

    pub use accounts::accounts;
    pub use root::app;
    pub use submits::submit;
    pub use user::user;
}

mod database;

mod entities {
    mod account;
    mod submit;
    mod user;

    pub use account::Account;
    pub use submit::Submit;
    pub use user::User;
}

mod repos_impl {
    mod accounts;
    mod submits;
    mod user;

    pub use accounts::AccountsImpl;
    pub use submits::SubmitImpl;
    pub use user::UserImpl;
}

mod repositories {
    mod accounts;
    mod submits;
    mod user;

    pub use accounts::Accounts;
    pub use submits::Submits;
    pub use user::Users;
}

mod services {
    mod accounts;
    mod submits;
    mod user;

    pub use accounts::{create_account, create_session, SessionToken};
    pub use submits::{create_submit, list_submit};
    pub use user::get_me;
}

mod request;

mod response;

mod views {
    mod home;
    mod sign_in;
    mod sign_up;
    mod partial {
        mod submit;
        pub use submit::Submit;
    }
    pub use home::Home;
    pub use partial::Submit;
    pub use sign_in::SignIn;
    pub use sign_up::SignUp;
}

pub use controllers::app;

pub async fn setup_session_store() {
    let database_url = constants::database_url();
    let store = async_sqlx_session::PostgresSessionStore::new(&database_url)
        .await
        .unwrap();
    store.migrate().await.unwrap();
    store.spawn_cleanup_task(std::time::Duration::from_secs(3600));
}
