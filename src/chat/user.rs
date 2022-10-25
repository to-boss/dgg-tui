use super::features::Feature;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub features: Vec<String>,
    #[serde(rename(deserialize = "nick"))]
    pub name: String,
    #[serde(skip_deserializing)]
    pub timestamp: String,
    #[serde(skip_deserializing)]
    pub flair: Feature,
}

impl User {
    pub fn from_json(json: &str) -> User {
        let mut user: User = serde_json::from_str(json).unwrap();
        user.parse_flair();
        user
    }

    pub fn parse_flair(&mut self) {
        self.flair = Feature::parse_flair(&self.features);
    }
}

impl Default for User {
    fn default() -> Self {
        User {
            features: Vec::new(),
            name: String::from("default_name"),
            timestamp: String::from("default_timestamp"),
            flair: Feature::White,
        }
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
