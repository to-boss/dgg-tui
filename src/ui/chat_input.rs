use std::collections::VecDeque;

pub struct ChatInput {
    pub max_messages: usize,
    pub history: VecDeque<String>,
    pub index: i8,
    pub current_message: String,
    pub buffer_message: String,
}

impl ChatInput {
    pub fn default() -> Self {
        let max_messages = 200;
        let index = 0;
        let history = VecDeque::with_capacity(max_messages);
        let buffer_message = String::new();
        let current_message = String::new();

        ChatInput {
            max_messages,
            history,
            index,
            buffer_message,
            current_message,
        }
    }

    /// Delete the last word of the current message and every whitespace before.
    pub fn delete_current_word(&mut self) {
        let index_back = self
            .current_message
            .chars()
            .rev()
            .position(|c| c.is_whitespace());

        match index_back {
            Some(index) => {
                if index == 0 {
                    self.current_message.pop();
                    self.delete_current_word();
                }
                let index_front = self.current_message.len() - index;
                self.current_message = self.current_message[..index_front].to_string();
            }
            None => self.current_message.clear(),
        }
    }

    pub fn get_current_word(&self) -> String {
        let index_back = self
            .current_message
            .chars()
            .rev()
            .position(|c| c.is_whitespace());

        match index_back {
            Some(index) => {
                if index == 0 {
                    return self.current_message[self.current_message.len() - 1..].to_string();
                }
                let index_front = self.current_message.len() - index;
                self.current_message[index_front..].to_string()
            }
            None => self.current_message[..].to_string(),
        }
    }

    pub fn add(&mut self) {
        // don't add the message to the history if its the same
        if self.history.len() > 0 && self.history[0] == self.current_message {
            self.current_message.clear();
            return;
        }
        if self.history.len() == self.max_messages {
            self.history.pop_back();
        }
        self.history
            .push_front(self.current_message.drain(..).collect());
        self.index = -1;
    }

    pub fn next(&mut self) {
        if self.history.len() == 0 {
            return;
        }

        if self.index == -1 {
            self.buffer_message = self.current_message.drain(..).collect();
            self.index = 0;
            self.current_message = self.history[self.index()].to_string();
            return;
        }

        if self.index() < self.history.len() - 1 {
            self.index += 1;
            self.current_message = self.history[self.index()].to_string();
        }
    }

    pub fn prev(&mut self) {
        if self.history.len() == 0 {
            return;
        }

        if self.index == 0 {
            self.current_message = self.buffer_message.drain(..).collect();
            self.index = -1;
            return;
        }

        if self.index > 0 {
            self.index -= 1;
            self.current_message = self.history[self.index()].to_string();
        }
    }

    pub fn index(&self) -> usize {
        self.index as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_word_test() {
        let mut cih = ChatInput::default();
        cih.current_message = String::from("hello whats up");
        let last_word = cih.get_current_word();
        assert_eq!(last_word, "up");
    }

    #[test]
    fn get_word_test_only_one_word() {
        let mut cih = ChatInput::default();
        cih.current_message = String::from("hello");
        let last_word = cih.get_current_word();
        assert_eq!(last_word, "hello");
    }

    #[test]
    fn get_word_test_whitespace_last() {
        let mut cih = ChatInput::default();
        cih.current_message = String::from("hello whats up ");
        let last_word = cih.get_current_word();
        assert_eq!(last_word, " ");
    }

    #[test]
    fn delete_current_word() {
        let mut cih = ChatInput::default();
        cih.current_message = String::from("hello whats up");
        cih.delete_current_word();
        assert_eq!(cih.current_message, "hello whats ");
    }

    #[test]
    fn delete_current_word_only_one_word() {
        let mut cih = ChatInput::default();
        cih.current_message = String::from("hello");
        cih.delete_current_word();
        assert_eq!(cih.current_message, "");
    }

    #[test]
    fn delete_current_word_whitespace_last() {
        let mut cih = ChatInput::default();
        cih.current_message = String::from("hello whats up ");
        cih.delete_current_word();
        assert_eq!(cih.current_message, "hello whats ");
    }
}
