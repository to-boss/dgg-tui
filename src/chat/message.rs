use serde_json::Value;
use std::{fmt::Display, io::Error};

pub struct Message {
    pub name: String,
    pub features: Vec<String>,
    pub timestamp: u64,
    pub message: String,
}

impl Message {
    pub fn from_json(json: &str) -> Result<Message, Error> {
        let v: Value = serde_json::from_str(json)?;

        let name = v["nick"].as_str().unwrap().to_string();
        let features = v["features"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| f.as_str().unwrap().to_string())
            .collect();
        let timestamp = v["timestamp"].as_u64().unwrap();
        let message = v["data"].as_str().unwrap().to_string();

        Ok(Message {
            name,
            features,
            timestamp,
            message,
        })
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.message)
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.message == other.message
    }
}
