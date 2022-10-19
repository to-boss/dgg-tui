use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub features: Vec<String>,
    #[serde(rename(deserialize = "nick"))]
    pub name: String,
    #[serde(with = "time::serde::timestamp", default = "default_timestamp")]
    pub timestamp: OffsetDateTime,
}

fn default_timestamp() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}

impl User {
    pub fn from_json(json: &str) -> User {
        serde_json::from_str(json).unwrap()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserList {
    #[serde(rename(deserialize = "connectioncount"))]
    pub conn_count: usize,
    pub users: Vec<User>,
}

impl UserList {
    pub fn from_json(json: &str) -> UserList {
        serde_json::from_str(json).unwrap()
    }

    pub fn append(&mut self, other: &mut UserList) {
        self.conn_count += other.conn_count;
        self.users.append(&mut other.users);
    }

    pub fn new() -> UserList {
        UserList {
            users: Vec::new(),
            conn_count: 0,
        }
    }
    pub fn remove(&mut self, user: User) {
        self.users.retain(|u| u.name != user.name);
    }

    pub fn add(&mut self, user: User) {
        self.users.push(user);
    }
}
