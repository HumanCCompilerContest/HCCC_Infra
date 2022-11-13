use serde::Serialize;
use postgres_types::{ToSql, FromSql};

#[derive(Serialize)]
#[derive(Debug, ToSql, FromSql)]
pub struct UserObject {
    id: i32,
    name: String,
}

#[derive(Serialize)]
#[derive(Debug, ToSql, FromSql)]
pub struct User {
    status: String,
    user: UserObject,
    #[serde(rename = "errorMessage")]
    error_message: Option<String>,
}

#[derive(Serialize)]
pub struct AllUsers {
    status: String,
    #[serde(rename = "items")]
    users: Vec<UserObject>,
    #[serde(rename = "errorMessage")]
    error_message: Option<String>,
}

impl UserObject {
    pub fn new(id: i32, name:String) -> Self {
        UserObject {
            id,
            name,
        }
    }

    pub fn dummy() -> Self {
        UserObject {
            id: 0,
            name: String::new(),
        }
    }
}

impl User {
    pub fn new(status: String, id: i32, name: String, error_message: Option<String>) -> Self {
        User {
            status,
            user: UserObject {
                id,
                name,
            },
            error_message,
        }
    }

    pub fn error(msg: &str) -> Self {
        User {
            status: "ng".to_string(),
            user: UserObject {
                id: 0,
                name: String::new(),
            },
            error_message: Some(msg.to_string())
        }
    }

    pub fn get_object(self) -> UserObject {
        self.user
    }
}

impl AllUsers {
    pub fn new(status: String, users: Vec<UserObject>, error_message: Option<String>) -> Self {
        AllUsers {
            status,
            users,
            error_message,
        }
    }
}

