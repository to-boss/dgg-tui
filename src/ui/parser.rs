use crate::chat::{features::Feature, message::ChatMessage, user::UserList};
use anyhow::Result;
use tui::{style::Style, text::Span, widgets::ListItem};

use super::emotes::EmoteList;

pub struct ParsedMessage<'a> {
    parts: Vec<ListItem<'a>>,
    words: Vec<String>,
    name: String,
    emote_list: &'a EmoteList,
    user_list: &'a UserList,
}

impl<'a> ParsedMessage<'a> {
    pub fn new(emote_list: &'a EmoteList, user_list: &'a UserList) -> Self {
        ParsedMessage {
            parts: Vec::new(),
            words: Vec::new(),
            name: String::from(""),
            emote_list,
            user_list,
        }
    }

    pub fn from_chat_message(
        chat_message: ChatMessage,
        emote_list: &'a EmoteList,
        user_list: &'a UserList,
    ) -> Self {
        let words = chat_message
            .message
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect();

        ParsedMessage {
            parts: Vec::new(),
            name: chat_message.name,
            words,
            emote_list,
            user_list,
        }
    }

    pub fn parse_chat_message(&mut self) {
        // Handle Linewrapping
        let full_line = format!("{}: {}", self.name, self.words.join(" "));
        // let lines = textwrap::wrap(&full_line, width);

        let items: Vec<ListItem> = self
            .words
            .iter()
            .map(|word| match word {
                arrow_right if word.starts_with(">") => {
                    ListItem::new(Span::styled(arrow_right, Style::default()))
                }
                link if word.starts_with("https://") => {
                    ListItem::new(Span::styled(link, Style::default()))
                }
                embed if word.starts_with("#youtube") => {
                    ListItem::new(Span::styled(embed, Style::default()))
                }
                embed if word.starts_with("#twitch") => {
                    ListItem::new(Span::styled(embed, Style::default()))
                }
                tag if word.len() == 4 && word.contains("nsfw") => {
                    ListItem::new(Span::styled(tag, Style::default()))
                }
                tag if word.len() == 4 && word.contains("nsfl") => {
                    ListItem::new(Span::styled(tag, Style::default()))
                }
                name if self.is_emote(&word) => ListItem::new(Span::styled(name, Style::default())),
                name if self.is_user(&word) => ListItem::new(Span::styled(name, Style::default())),
                _ => ListItem::new(Span::styled(word, Style::default())),
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
