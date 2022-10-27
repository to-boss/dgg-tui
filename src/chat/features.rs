use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use tui::style::Color;

#[derive(Debug, Deserialize, Serialize)]
pub enum Feature {
    White,
    Sub,
    Tier1,
    Tier2,
    Tier3,
    Tier4,
    Bot,
    Bot2,
    Protected,
    Vip,
    Mod,
    Notable,
    Trusted,
    Contributor,
    Music,
    Broadcaster,
    Birthday,
    Admin,
    Sc2,
    Dnd,
    Lawyer,
    EmoteContributor,
    YoutubeContributor,
    Twitch,
    Eve,
    Gym,
    League,
    Nfl,
    MinecraftVIP,
    Micro,
    EmoteMaster,
    DggShirtDesigner,
    Verified,
    YoutubeEditor,
    DndGold,
    DndScoria,
    DndKnight,
    TikTokEditor,
}

impl Feature {
    pub fn to_color(&self) -> Color {
        match self {
            Feature::Tier1 => Color::Cyan,
            Feature::Tier2 => Color::LightCyan,
            Feature::Tier3 => Color::LightGreen,
            Feature::Tier4 => Color::Magenta,
            Feature::Vip => Color::Rgb(219, 76, 28),
            Feature::Micro => Color::Yellow,
            Feature::Mod => Color::Yellow,
            Feature::Broadcaster => Color::Rgb(230, 144, 20),
            Feature::Notable => Color::Rgb(230, 144, 20),
            Feature::Admin => Color::Red,
            _ => Color::White,
        }
    }

    pub fn parse_flair(flairs: &Vec<String>) -> Feature {
        match flairs.len() {
            4 => Feature::from_str(&flairs[2]).unwrap(),
            2 | 3 => Feature::from_str(&flairs[1]).unwrap(),
            1 => Feature::from_str(&flairs[0]).unwrap(),
            _ => Feature::White,
        }
    }

    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "subscriber" => Ok(Feature::Sub),
            "bot" => Ok(Feature::Bot),
            "protected" => Ok(Feature::Protected),
            "vip" => Ok(Feature::Vip),
            "moderator" => Ok(Feature::Mod),
            "admin" => Ok(Feature::Admin),
            "flair1" => Ok(Feature::Tier2),
            "flair2" => Ok(Feature::Notable),
            "flair3" => Ok(Feature::Tier3),
            "flair4" => Ok(Feature::Trusted),
            "flair5" => Ok(Feature::Contributor),
            "flair6" => Ok(Feature::Music),
            "flair7" => Ok(Feature::Nfl),
            "flair8" => Ok(Feature::Tier4),
            "flair9" => Ok(Feature::Twitch),
            "flair10" => Ok(Feature::Sc2),
            "flair11" => Ok(Feature::Bot2),
            "flair12" => Ok(Feature::Broadcaster),
            "flair13" => Ok(Feature::Tier1),
            "flair14" => Ok(Feature::MinecraftVIP),
            "flair15" => Ok(Feature::Birthday),
            "flair16" => Ok(Feature::EmoteContributor),
            "flair17" => Ok(Feature::Micro),
            "flair18" => Ok(Feature::EmoteMaster),
            "flair19" => Ok(Feature::DggShirtDesigner),
            "flair20" => Ok(Feature::Verified),
            "flair21" => Ok(Feature::YoutubeEditor),
            "flair22" => Ok(Feature::DndGold),
            "flair23" => Ok(Feature::White),
            "flair24" => Ok(Feature::DndScoria),
            "flair25" => Ok(Feature::YoutubeContributor),
            "flair26" => Ok(Feature::DndKnight),
            "flair27" => Ok(Feature::TikTokEditor),
            "flair28" => Ok(Feature::Lawyer),
            "flair29" => Ok(Feature::Gym),
            "flair30" => Ok(Feature::League),
            _ => bail!("Could not find flair {}", s),
        }
    }
}

impl Default for Feature {
    fn default() -> Self {
        Feature::White
    }
}
