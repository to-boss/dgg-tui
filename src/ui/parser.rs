use crate::chat::features::Feature;
use anyhow::Result;

use super::emotes::EmoteList;

pub fn parse_emotes(words: &mut Vec<&str>, emotes: &EmoteList) -> String {
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

// pub fn parse_links(words: &mut Vec<&str>) -> Vec<&str> {
//     words.iter_mut().for_each(|word| if word.starts_with("https://") {

//     })
// }

pub fn parse_flair(feats: &Vec<String>) -> Result<Feature> {
    match feats.len() {
        2 | 3 | 4 | 5 | 6 => Ok(Feature::from_str(&feats[1])?),
        1 => Ok(Feature::from_str(&feats[0])?),
        _ => Ok(Feature::White),
    }
}
