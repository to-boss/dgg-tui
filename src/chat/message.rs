use serde::{Deserialize, Serialize};
use std::{fmt::Display, io::Error};
use time::OffsetDateTime;

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    #[serde(rename(deserialize = "data"))]
    pub message: String,
    pub features: Vec<String>,
    #[serde(rename(deserialize = "nick"))]
    pub name: String,
    #[serde(with = "time::serde::timestamp")]
    pub timestamp: OffsetDateTime,
}

impl Message {
    pub fn from_json(json: &str) -> Result<Message, Error> {
        let m: Message = serde_json::from_str(json)?;
        Ok(m)
    }

    pub fn from(name: String, message: String) -> Message {
        Message {
            name,
            features: Vec::new(),
            timestamp: OffsetDateTime::now_utc(),
            message,
        }
    }

    pub fn get_timestamp_str(&self) -> String {
        let hour = self.timestamp.hour();
        let minutes = self.timestamp.minute();
        if hour < 10 && minutes < 10 {
            format!("0{}:0{}", hour, minutes)
        } else if hour < 10 {
            format!("0{}:{}", hour, minutes)
        } else if minutes < 10 {
            format!("{}:0{}", hour, minutes)
        } else {
            format!("{}:{}", hour, minutes)
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", self.timestamp, self.name, self.message)
    }
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.message == other.message
    }
}
