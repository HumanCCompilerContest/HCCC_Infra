use sha2::{Digest, Sha256};

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
