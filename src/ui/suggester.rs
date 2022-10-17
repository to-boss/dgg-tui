use super::emotes::EmoteList;

pub struct Suggestor<'a> {
    pub suggestions: Vec<&'a str>,
    pub current_word: String,
    pub emote_list: &'a EmoteList,
}

impl<'a> Suggestor<'a> {
    pub fn new(emote_list: &'a EmoteList) -> Self {
        Suggestor {
            suggestions: Vec::new(),
            current_word: String::new(),
            emote_list,
        }
    }

    pub fn manage_suggestions(&mut self) {
        if self.current_word.len() > 0 {
            self.suggestions = self
                .emote_list
                .emotes
                .iter()
                .filter(|emote| emote.name.starts_with(&self.current_word))
                .map(|emote| emote.name)
                .collect()
        } else {
            self.suggestions.clear();
        }
    }

    pub fn consume(&mut self) -> &'a str {
        let sug = &self.suggestions[0][self.current_word.len()..];
        self.suggestions.clear();
        sug
    }

    pub fn clear_word(&mut self) {
        self.current_word.clear();
        self.manage_suggestions();
    }

    pub fn pop(&mut self) {
        if self.current_word.len() > 0 {
            self.current_word.pop().unwrap();
            self.manage_suggestions();
        }
    }

    pub fn push(&mut self, c: char) {
        self.current_word.push(c);
        self.manage_suggestions();
    }
}
