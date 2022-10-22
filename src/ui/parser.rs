use crate::chat::{features::Feature, user::UserList};
use anyhow::Result;

use super::emotes::EmoteList;

pub struct ParsedMessage<'a> {
    parts: Vec<Part<'a>>,
    words: Vec<&'a str>,
    emote_list: &'a EmoteList,
    user_list: &'a UserList,
}

impl<'a> ParsedMessage<'a> {
    pub fn new(emote_list: &'a EmoteList, user_list: &'a UserList) -> Self {
        ParsedMessage {
            parts: Vec::new(),
            words: Vec::new(),
            emote_list,
            user_list,
        }
    }
    pub fn parse_chat_message(&mut self) {
        self.parts = self
            .words
            .iter()
            .map(|word| match *word {
                arrow_right if word.starts_with(">") => Part::ArrowRight(arrow_right),
                link if word.starts_with("https://") => Part::Link(link),
                embed if word.starts_with("#youtube") => Part::YoutubeEmbed(embed),
                embed if word.starts_with("#twitch") => Part::TwitchEmbed(embed),
                tag if word.len() == 4 && word.contains("nsfw") => Part::Nsfw(tag),
                tag if word.len() == 4 && word.contains("nsfl") => Part::Nsfl(tag),
                name if self.is_emote(word) => Part::Emote(name),
                name if self.is_user(word) => Part::User(name),
                _ => Part::Word(*word),
            })
            .collect();
    }

    pub fn is_user(&self, word: &str) -> bool {
        self.user_list.users.iter().any(|user| word == user.name)
    }

    pub fn is_emote(&self, word: &str) -> bool {
        self.emote_list
            .emotes
            .iter()
            .any(|emote| word == emote.name)
    }
}

pub enum Part<'a> {
    ArrowRight(&'a str),
    Link(&'a str),
    YoutubeEmbed(&'a str),
    TwitchEmbed(&'a str),
    Nsfw(&'a str),
    Nsfl(&'a str),
    Emote(&'a str),
    User(&'a str),
    Word(&'a str),
}

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

pub fn parse_flair(feats: &Vec<String>) -> Result<Feature> {
    match feats.len() {
        2 | 3 | 4 | 5 | 6 => Ok(Feature::from_str(&feats[1])?),
        1 => Ok(Feature::from_str(&feats[0])?),
        _ => Ok(Feature::White),
    }
}
