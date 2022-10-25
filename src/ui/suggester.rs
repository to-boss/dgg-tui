use std::fmt::Display;

use crate::chat::{command::Command, user::UserList};

use super::emotes::EmoteList;

// TODO: Should prob have a &ChatInput
// then we can order suggestions after usage
pub struct Suggestor<'a> {
    pub suggestions: Vec<String>,
    pub emote_list: &'a EmoteList,
    pub current_word: String,
    pub index: usize,
}

impl<'a> Suggestor<'a> {
    pub fn new(emote_list: &'a EmoteList) -> Self {
        let suggestions = Vec::new();

        Suggestor {
            suggestions,
            emote_list,
            current_word: "".to_string(),
            index: 0,
        }
    }

    pub fn get(&mut self) -> String {
        if let Some(suggestion) = self.suggestions.get(self.index) {
            self.index += 1;
            format!("{} ", suggestion)
        } else {
            self.index = 0;
            let suggestion = self.suggestions.get(self.index).unwrap();
            self.index += 1;
            format!("{} ", suggestion)
        }
    }

    pub fn update(&mut self, user_list: &UserList, current_word: String) {
        if current_word.len() == 0 {
            self.suggestions.clear();
        } else {
            self.current_word = current_word.to_lowercase();

            let mut emote_suggestions: Vec<String> = self
                .emote_list
                .emotes
                .iter()
                .filter(|emote| emote.name.to_lowercase().starts_with(&self.current_word))
                .take(10)
                .map(|emote| emote.name.to_string())
                .collect();

            let mut username_suggestions: Vec<String> = user_list
                .users
                .iter()
                .filter(|user| user.name.to_lowercase().starts_with(&self.current_word))
                .take(10)
                .map(|user| user.name.to_string())
                .collect();

            // suggest commands
            if &self.current_word[..1] == "/" {
                emote_suggestions.append(&mut Command::vec())
            }

            // emotes get recommended before names
            emote_suggestions.append(&mut username_suggestions);

            self.suggestions = emote_suggestions;
            self.index = 0;
        }
    }
}

impl Display for Suggestor<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.suggestions.len() > 0 {
            write!(f, " {} ", self.suggestions.join(" | "))
        } else {
            write!(f, "─")
        }
    }
}
