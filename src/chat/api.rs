use std::{error::Error, fmt::Display};

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::message::Message;

pub struct ApiCaller {
    client: Client,
}

impl ApiCaller {
    pub fn new() -> ApiCaller {
        let client = reqwest::blocking::Client::default();
        ApiCaller { client }
    }

    pub fn get_last_embeds(&self) -> Result<Vec<Embed>, Box<dyn Error>> {
        let res = self
            .client
            .get("https://vyneer.me/tools/embeds/last")
            .send()?
            .text()
            .unwrap();
        let embeds: Vec<Embed> = serde_json::from_str(&res).unwrap();
        Ok(embeds)
    }

    pub fn get_chat_history(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let res = self
            .client
            .get("https://www.destiny.gg/api/chat/history")
            .send()?
            .text()
            .unwrap();
        let messages: Vec<String> = serde_json::from_str(&res)?;
        Ok(messages)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Embed {
    pub channel: String,
    pub link: String,
    pub platform: String,
    pub timestamp: u64,
    pub title: String,
}

impl Display for Embed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.channel, self.title)
    }
}