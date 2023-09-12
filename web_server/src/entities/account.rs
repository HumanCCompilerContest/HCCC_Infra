use crate::entities::UserObject;
use serde::Serialize;
use sha2::{Digest, Sha256};

/// The response for create account.
#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct AccountResponse {
    /// Creating account successeed or not.
    /// * `ok` - successeed
    /// * `ng` - failed
    status: String,
    /// Created user account.
    user: UserObject,
    /// Error message.
    errorMessage: Option<String>,
}

impl AccountResponse {
    /// Return ok response.
    pub fn new(id: i32, name: String) -> Self {
        AccountResponse {
            status: "ok".to_string(),
            user: UserObject::new(id, name),
            errorMessage: None,
        }
    }

    /// Called when failed to create account
    pub fn error(errmsg: &str) -> Self {
        AccountResponse {
            status: "ng".to_string(),
            user: UserObject::dummy(),
            errorMessage: Some(errmsg.to_string()),
        }
    }
}

/// User account.
pub struct Account {
    /// Account id.
    id: Option<i32>,
    /// User name.
    pub user_name: String,
    /// password hashed by sha256.
    pub hashed_password: String,
}

impl Account {
    /// Return new `Account`.
    pub fn new(id: i32, user_name: String, hashed_password: String) -> Account {
        Account {
            id: Some(id),
            user_name,
            hashed_password,
        }
    }

    /// Create new account.
    pub fn create(user_name: &str, password: &str) -> Account {
        Account {
            id: None,
            user_name: user_name.to_string(),
            hashed_password: to_sha256(password),
        }
    }

    /// Return its id.
    pub fn id(&self) -> Option<i32> {
        self.id
    }

    /// Whether given password matches stored password or not.
    pub fn matches_password(&self, password: &str) -> bool {
        self.hashed_password == to_sha256(password)
    }
}

fn to_sha256(str: &str) -> String {
    let str = str.as_bytes();
    let hashed_str = Sha256::digest(str);

    format!("{hashed_str:x}")
}
