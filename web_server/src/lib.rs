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
    mod users;
    mod problems;

    pub use accounts::accounts;
    pub use root::app;
    pub use submits::submit;
    pub use users::user;
    pub use problems::problem;
}

mod database;

mod entities {
    mod account;
    mod submit;
    mod users;
    mod problem;

    pub use account::Account;
    pub use submit::Submit;
    pub use users::{User, UserObject, AllUsers};
    pub use problem::{Problem, ProblemObject, AllProblems};
}

mod repos_impl {
    mod accounts;
    mod submits;
    mod users;
    mod problems;

    pub use accounts::AccountsImpl;
    pub use submits::SubmitImpl;
    pub use users::UserImpl;
    pub use problems::ProblemImpl;
}

mod repositories {
    mod accounts;
    mod submits;
    mod users;
    mod problem;

    pub use accounts::Accounts;
    pub use submits::Submits;
    pub use users::Users;
    pub use problem::Problems;
}

mod services {
    mod accounts;
    mod submits;
    mod users;
    mod problems;

    pub use accounts::{create_account, create_session, SessionToken};
    pub use submits::{create_submit, list_submit};
    pub use users::{get_user, get_all_users};
    pub use problems::{get_problem, get_all_problems};
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
