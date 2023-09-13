use postgres_types::{FromSql, ToSql};
use serde::Serialize;

/// User account data.
#[derive(Serialize, Debug, ToSql, FromSql)]
pub struct UserObject {
    /// User id.
    id: i32,
    /// User display name.
    name: String,
}

/// Response data of `/api/user/me` or `/api/user/:id`.
#[derive(Serialize, Debug, ToSql, FromSql)]
pub struct User {
    /// Getting user data successeed or not.
    /// * `ok` - successeed
    /// * `ng` - failed
    status: String,
    /// User data.
    user: UserObject,
    /// Error message.
    #[serde(rename = "errorMessage")]
    error_message: Option<String>,
}

/// Response data of `/api/user/`.
#[derive(Serialize)]
pub struct AllUsers {
    /// Getting all users data successeed or not.
    /// * `ok` - successeed
    /// * `ng` - failed
    status: String,
    /// All users data.
    #[serde(rename = "items")]
    users: Vec<UserObject>,
    /// Error message.
    #[serde(rename = "errorMessage")]
    error_message: Option<String>,
}

impl UserObject {
    /// Return new `UserObject`.
    pub fn new(id: i32, name: String) -> Self {
        UserObject { id, name }
    }

    /// Return dummy `UserObject`.
    pub fn dummy() -> Self {
        UserObject {
            id: 0,
            name: String::new(),
        }
    }
}

impl User {
    /// Return successeed response.
    pub fn new(status: String, id: i32, name: String, error_message: Option<String>) -> Self {
        User {
            status,
            user: UserObject { id, name },
            error_message,
        }
    }

    /// Return successeed response.
    pub fn error(msg: &str) -> Self {
        User {
            status: "ng".to_string(),
            user: UserObject {
                id: 0,
                name: String::new(),
            },
            error_message: Some(msg.to_string()),
        }
    }

    /// Return `UserObject`.
    pub fn get_object(self) -> UserObject {
        self.user
    }
}

impl AllUsers {
    /// Return new `AllUsers`.
    pub fn new(status: String, users: Vec<UserObject>, error_message: Option<String>) -> Self {
        AllUsers {
            status,
            users,
            error_message,
        }
    }
}
