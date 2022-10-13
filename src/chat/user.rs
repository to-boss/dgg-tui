use std::io::Error;

use serde_json::Value;

pub struct User {
    pub name: String,
    pub features: Vec<String>,
}

impl User {
    pub fn from_json(json: &str) -> Result<User, Error> {
        let v: Value = serde_json::from_str(json)?;
        let name = v["nick"].as_str().unwrap().to_string();
        let features = v["features"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| f.as_str().unwrap().to_string())
            .collect();
        Ok(User { name, features })
    }
}

pub struct UserList {
    pub users: Vec<User>,
    pub conn_count: usize,
}

impl UserList {
    pub fn from_json(json: &str) -> Result<UserList, Error> {
        let mut v: Value = serde_json::from_str(json)?;
        let connection_count = v["connectioncount"].as_u64().unwrap() as usize;
        let users_json = v["users"].as_array_mut().unwrap();
        let users = users_json
            .iter_mut()
            .map(|user| User::from_json(user.to_string().as_str()).unwrap())
            .collect();

        Ok(UserList {
            users,
            conn_count: connection_count,
        })
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
