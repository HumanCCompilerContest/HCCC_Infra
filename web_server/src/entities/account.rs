use serde::Serialize;
use sha2::{Digest, Sha256};
use crate::entities::UserObject;

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct AccountResponse {
    status: String,
    user: UserObject,
    errorMessage: Option<String>,
}

impl AccountResponse {
    pub fn new(id: i32, name: String) -> Self {
        AccountResponse {
            status: "ok".to_string(),
            user: UserObject::new(id, name),
            errorMessage: None,
        }
    }

    pub fn error() -> Self {
        AccountResponse {
            status: "ng".to_string(),
            user: UserObject::dummy(),
            errorMessage: Some("session error".to_string()),
        }
    }
}

pub struct Account {
    id: Option<i32>,
    pub user_name: String,
    pub hashed_password: String,
}

impl Account {
    pub fn new(id: i32, user_name: String, hashed_password: String) -> Account {
        Account {
            id: Some(id),
            user_name,
            hashed_password,
        }
    }

    pub fn create(user_name: &str, password: &str) -> Account {
        Account {
            id: None,
            user_name: user_name.to_string(),
            hashed_password: to_sha256(password),
        }
    }

    pub fn id(&self) -> Option<i32> {
        self.id
    }

    pub fn matches_password(&self, password: &str) -> bool {
        self.hashed_password == to_sha256(password)
    }
}

fn to_sha256(str: &str) -> String {
    let str = str.as_bytes();
    let hashed_str = Sha256::digest(str);

    format!("{:x}", hashed_str)
}
