use crate::chat::features::Feature;
use anyhow::{bail, Result};

use super::emotes::EmoteList;

pub fn parse_emotes(s: String, emotes: &EmoteList) -> String {
    let mut words: Vec<&str> = s.split_whitespace().collect();

    // replace words into emotes
    emotes.emotes.iter().for_each(|emote| {
        for word in words.iter_mut() {
            if *word == emote.name {
                *word = emote.emote;
            }
        }
    });

    // reconstruct into message and return
    words.join(" ")
}

pub fn parse_flair(feats: &Vec<String>) -> Result<Feature> {
    let len = feats.len();
    if len == 2 {
        return Ok(Feature::from_str(&feats[1])?);
    } else if len == 1 && feats.len() > 0 {
        return Ok(Feature::from_str(&feats[0])?);
    } else {
        return Ok(Feature::White);
    }
    bail!("Could not parse_parse flair");
}
