use serde::{Deserialize, Serialize};
use std::fmt::Display;

use super::features::Feature;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatMessage {
    #[serde(rename(deserialize = "data"))]
    pub message: String,
    pub features: Vec<String>,
    #[serde(rename(deserialize = "nick"))]
    pub name: String,
    #[serde(skip_deserializing)]
    pub timestamp: String,
    #[serde(skip_deserializing)]
    pub flair: Feature,
    #[serde(skip_deserializing)]
    pub greentext: bool,
    #[serde(skip_deserializing)]
    pub own_message: bool,
    #[serde(skip_deserializing)]
    pub mentioned: bool,
    #[serde(skip_deserializing)]
    pub nsfw: bool,
    #[serde(skip_deserializing)]
    pub nsfl: bool,
}

impl ChatMessage {
    pub fn from_json(json: &str) -> ChatMessage {
        serde_json::from_str(json).unwrap()
    }

    pub fn from_string(name: String, message: String) -> ChatMessage {
        ChatMessage {
            name,
            features: Vec::new(),
            timestamp: String::from("default_timestamp"),
            message,
            flair: Feature::White,
            greentext: false,
            own_message: false,
            mentioned: false,
            nsfw: false,
            nsfl: false,
        }
    }

    pub fn parse(&mut self, username: &str) {
        // parse flair
        self.flair = Feature::parse_flair(&self.features);

        // parse message
        self.parse_message(username);
    }

    fn parse_message(&mut self, username: &str) {
        self.message
            .split_whitespace()
            .into_iter()
            .for_each(|word| {
                if word.starts_with(">") {
                    self.greentext = true;
                }

                if word.len() == 4 {
                    match word {
                        "nsfw" => self.nsfw = true,
                        "nsfl" => self.nsfl = true,
                        _ => (),
                    }
                }

                if word.len() == username.len() && word.eq(username) {
                    self.mentioned = true;
                }

                if self.name.len() == username.len() && self.name.eq(username) {
                    self.own_message = true;
                }
            });
    }
}

impl Display for ChatMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.message)
    }
}

impl PartialEq for ChatMessage {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.message == other.message
    }
}
