use serde::Serialize;

#[derive(Serialize)]
pub struct UserObject {
    id: i32,
    name: String,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct User {
    status: String,
    user: UserObject,
    errorMessage: Option<String>,
}

impl User {
    pub fn new(status: String, id: i32, name: String, error_message: Option<String>) -> Self {
        User {
            status,
            user: UserObject {
                id,
                name,
            },
            errorMessage: error_message,
        }
    }
}

