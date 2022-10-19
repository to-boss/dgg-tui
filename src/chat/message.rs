use serde::{Deserialize, Serialize};
use std::fmt::Display;
use time::OffsetDateTime;
use tungstenite::Message;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatMessage {
    #[serde(rename(deserialize = "data"))]
    pub message: String,
    pub features: Vec<String>,
    #[serde(rename(deserialize = "nick"))]
    pub name: String,
    #[serde(with = "time::serde::timestamp")]
    pub timestamp: OffsetDateTime,
}

impl ChatMessage {
    pub fn from_json(json: &str) -> ChatMessage {
        serde_json::from_str(json).unwrap()
    }

    pub fn from_string(name: String, message: String) -> ChatMessage {
        ChatMessage {
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

impl From<Message> for ChatMessage {
    fn from(msg: Message) -> ChatMessage {
        ChatMessage::from_string("hi".to_string(), "hi".to_string())
    }
}

impl Display for ChatMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.get_timestamp_str(),
            self.name,
            self.message
        )
    }
}

impl PartialEq for ChatMessage {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.message == other.message
    }
}
