use std::str::FromStr;

use crate::chat::features::Feature;

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

pub fn parse_flair(feats: &Vec<String>) -> Feature {
    let len = feats.len();
    if len == 2 {
        return Feature::from_str(&feats[1]).unwrap();
    } else if len == 1 && feats.len() > 0 {
        return Feature::from_str(&feats[0]).unwrap();
    } else {
        Feature::White
    }
}
