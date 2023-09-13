mod constants {
    //! Constants of connection data and the contest data.
    use chrono::{DateTime, Local};
    use std::env;

    pub const AXUM_SESSION_COOKIE_NAME: &str = "hccc_session";
    pub const AXUM_SESSION_USER_ID_KEY: &str = "uid";

    /// Return database url.
    ///
    /// # Panics
    /// It will panic if enviroment variable `DATABASE_URL` is not found.
    pub fn database_url() -> String {
        dotenv::dotenv().ok();
        env::var("DATABASE_URL").unwrap()
    }

    /// Return the contest duration obtained from the env variable.
    ///
    /// # Panics
    /// It will panic if variable contents format is invalid.
    pub fn contest_duration() -> (DateTime<Local>, DateTime<Local>) {
        dotenv::dotenv().ok();
        (
            DateTime::parse_from_rfc3339(&env::var("CONTEST_BEGIN").unwrap())
                .unwrap()
                .with_timezone(&Local),
            DateTime::parse_from_rfc3339(&env::var("CONTEST_END").unwrap())
                .unwrap()
                .with_timezone(&Local),
        )
    }
}

mod controllers {
    //! Routing the api.

    mod accounts;
    mod problems;
    mod root;
    mod submissions;
    mod users;

    pub use problems::problem;
    pub use root::app;
    pub use submissions::submissions;
    pub use users::user;
}

mod database;

mod entities {
    //! Definition deta structure.

    mod account;
    mod problem;
    mod ranking;
    mod submission;
    mod users;

    pub use account::{Account, AccountResponse};
    pub use problem::{AllProblems, Problem, ProblemObject};
    pub use ranking::{Rank, Ranking};
    pub use submission::{JudgeResult, Submission, SubmissionObject, UserSubmissions};
    pub use users::{AllUsers, User, UserObject};
}

mod repos_impl {
    //! Connect to database and load/store data.

    mod accounts;
    mod problems;
    mod submissions;
    mod users;

    pub use accounts::AccountsImpl;
    pub use problems::ProblemImpl;
    pub use submissions::SubmissionImpl;
    pub use users::UserImpl;
}

mod repositories {
    //! Definition interface for communicate database.

    mod accounts;
    mod problem;
    mod submissions;
    mod users;

    pub use accounts::Accounts;
    pub use problem::Problems;
    pub use submissions::Submissions;
    pub use users::Users;
}

mod services {
    //! Definition specific services.

    mod accounts;
    mod problems;
    mod ranking;
    mod submissions;
    mod users;

    pub use accounts::{create_account, create_session, SessionToken};
    pub use problems::{get_all_problems, get_problem};
    pub use ranking::get_ranking;
    pub use submissions::{
        get_all_users_submissions, get_submission, get_user_submissions, submit_asm,
    };
    pub use users::{get_all_users, get_user};
}

mod request;

pub use controllers::app;

/// Setup the session store for postgress.  
/// The session store will clean up sessions every 3600 sec.
///
/// # Panics
/// If failed to connect database or failed to create session.
pub async fn setup_session_store() {
    use crate::constants::database_url;
    let store = async_sqlx_session::PostgresSessionStore::new(&database_url())
        .await
        .unwrap();
    store.migrate().await.unwrap();
    store.spawn_cleanup_task(std::time::Duration::from_secs(3600));
}

/// Return whether the contest has not yet begun or not.
#[must_use]
pub fn is_contest_has_not_yet_begun() -> bool {
    let (begin, _end) = constants::contest_duration();
    let now = chrono::Local::now();

    now < begin
}

/// Return whether or not the contest is underway.
#[must_use]
pub fn is_contest_underway() -> bool {
    let (begin, end) = constants::contest_duration();
    let now = chrono::Local::now();

    begin <= now && now <= end
}
