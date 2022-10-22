use std::collections::VecDeque;

pub struct ChatInputHistory {
    pub max_messages: usize,
    pub history: VecDeque<String>,
    pub index: i8,
    pub current_message: String,
    pub buffer_message: String,
}

impl ChatInputHistory {
    pub fn default() -> Self {
        let max_messages = 100;
        let index = 0;
        let history = VecDeque::with_capacity(max_messages);
        let buffer_message = String::new();
        let current_message = String::new();

        ChatInputHistory {
            max_messages,
            history,
            index,
            buffer_message,
            current_message,
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
