use std::str::FromStr;

enum Feature {
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
    Birtday,
    Admin,
    Sc2,
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
            //"flair7" => Ok(Feature::Eve), is this even implemented in dgg?
            "flair8" => Ok(Feature::Tier4),
            //"flair9" => Ok(Feature::Twitch),
            "flair10" => Ok(Feature::Sc2),
            "flair11" => Ok(Feature::Bot2),
            "flair12" => Ok(Feature::Broadcaster),
            "flair13" => Ok(Feature::Tier1),
            "flair15" => Ok(Feature::Birtday),
            _ => Err(()),
        }
    }
}
