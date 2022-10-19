use anyhow::{bail, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, time::Duration};
use time::OffsetDateTime;
use tokio::time::Instant;

pub struct ApiCaller {
    client: Client,
    timer: Instant,
}

impl ApiCaller {
    pub fn new() -> ApiCaller {
        let client = reqwest::Client::default();
        let timer = Instant::now();
        ApiCaller { client, timer }
    }

    fn check_timer(&mut self) -> Result<()> {
        if self.timer.elapsed() <= Duration::from_secs(10) {
            bail!("Wait atleast 10 seconds between API calls.")
        } else {
            self.timer = Instant::now();
            Ok(())
        }
    }

    pub async fn stalk(&mut self, username: String, size: u8) -> Result<Vec<Stalk>> {
        self.check_timer()?;

        let res = self
            .client
            .get(format!(
                "https://polecat.me/api/stalk/{}?size={}",
                username, size
            ))
            .send()
            .await?
            .text()
            .await?;

        let messages: Vec<Stalk> = serde_json::from_str(&res)?;
        Ok(messages)
    }

    pub async fn get_last_embeds(&mut self) -> Result<Vec<Embed>> {
        self.timer = Instant::now();
        let res = self
            .client
            .get("https://vyneer.me/tools/embeds?t=30")
            .send()
            .await?
            .text()
            .await?;

        let embeds: Vec<Embed> = serde_json::from_str(&res)?;
        Ok(embeds)
    }

    pub async fn get_chat_history(&self) -> Result<Vec<String>> {
        let res = self
            .client
            .get("https://www.destiny.gg/api/chat/history")
            .send()
            .await?
            .text()
            .await?;

        let messages: Vec<String> = serde_json::from_str(&res)?;
        Ok(messages)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatHistory {
    nick: String,
    features: Vec<String>,
    #[serde(with = "time::serde::timestamp")]
    pub timestamp: OffsetDateTime,
    data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stalk {
    #[serde(with = "time::serde::timestamp", rename(deserialize = "date"))]
    pub timestamp: OffsetDateTime,
    pub flairs: String,
    pub nick: String,
    pub text: String,
}

impl Stalk {
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

impl Display for Stalk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.get_timestamp_str(),
            self.nick,
            self.text
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Embed {
    pub channel: String,
    pub count: u8,
    pub link: String,
    pub platform: String,
    pub title: String,
}

impl Embed {
    pub fn real_link(&self) -> String {
        let index = self.link.find("/").unwrap();
        let prefix = "https://www.";
        let fix = match self.platform.as_str() {
            "twitch" => "twitch.tv",
            "youtube" => "youtube.com",
            _ => "ERROR: matching real_link()",
        };
        let suffix = &self.link[index..];
        format!("{}{}{}", prefix, fix, suffix)
    }
}

impl Display for Embed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{} times] {} | {}",
            self.count,
            self.real_link(),
            self.title,
        )
    }
}
