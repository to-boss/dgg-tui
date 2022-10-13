use core::panic;
use std::str::FromStr;

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
}

impl FromStr for Feature {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
            "flair7" => Ok(Feature::Eve),
            "flair8" => Ok(Feature::Tier4),
            "flair9" => Ok(Feature::Twitch),
            "flair10" => Ok(Feature::Sc2),
            "flair11" => Ok(Feature::Bot2),
            "flair12" => Ok(Feature::Broadcaster),
            "flair13" => Ok(Feature::Tier1),
            "flair14" => Ok(Feature::White), // not sure
            "flair15" => Ok(Feature::Birthday),
            "flair16" => Ok(Feature::EmoteContributor),
            "flair17" => Ok(Feature::White), // not sure
            "flair18" => Ok(Feature::White), // not sure
            "flair19" => Ok(Feature::White), // not sure
            "flair20" => Ok(Feature::White), // not sure
            "flair21" => Ok(Feature::White), // not sure
            "flair22" => Ok(Feature::White), // not sure
            "flair23" => Ok(Feature::White), // not sure
            "flair24" => Ok(Feature::Dnd),
            "flair25" => Ok(Feature::YoutubeContributor),
            "flair26" => Ok(Feature::Dnd),
            "flair27" => Ok(Feature::White), // not sure
            "flair28" => Ok(Feature::Lawyer),
            "flair29" => Ok(Feature::Gym),
            _ => panic!("ParserError: {}", s),
        }
    }
}
