use std::{collections::VecDeque, sync::mpsc::Sender};

use crate::ui::window::{Window, WindowList, WindowType};

use super::{action::Action, message::ChatMessage, user::UserList};

pub struct State {
    pub io_sender: Sender<Action>,
    pub username: String,
    pub chat_input: String,
    pub ul: UserList,
    pub messages: Vec<ChatMessage>,
    pub message_to_send: Option<String>,
    pub windows: WindowList,
    pub debugs: Vec<String>,
    pub chat_history: VecDeque<String>,
    pub history_index: usize,
}

impl State {
    pub fn new(max_messages: u16, username: String, io_sender: Sender<Action>) -> State {
        let ul = UserList::new();
        let messages = Vec::new();
        let debugs = Vec::new();
        let chat_input = String::new();
        let chat_history = VecDeque::with_capacity(50);
        let history_index = 0;
        let windows = WindowList {
            windows: vec![
                Window::new(WindowType::Chat, true, max_messages),
                Window::new(WindowType::ChatInput, true, 2),
                Window::new(WindowType::Debug, true, 30),
                Window::new(WindowType::UserList, false, 50),
            ],
        };

        State {
            io_sender,
            username,
            chat_input,
            ul,
            messages,
            windows,
            message_to_send: None,
            debugs,
            chat_history,
            history_index,
        }
    }

    pub fn dispatch(&self, action: Action) {
        self.io_sender.send(action).unwrap();
    }

    pub fn add_error(&mut self, msg: String) {
        self.add_message(ChatMessage::from_string("ERROR".to_string(), msg));
    }

    pub fn add_message(&mut self, msg: ChatMessage) {
        if self.messages.len() >= self.windows.get(WindowType::Chat).max_displays.into() {
            self.messages.drain(0..1);
        }
        self.messages.push(msg);
    }

    pub fn add_debug(&mut self, s: String) {
        if self.debugs.len() >= self.windows.get(WindowType::Debug).max_displays.into() {
            self.debugs.drain(0..1);
        }
        self.debugs.push(format!("{}", s));
    }

    pub fn add_to_chat_history(&mut self) {
        self.chat_history.push_front(self.chat_input.to_owned());
        self.chat_input.clear();
    }

    pub fn chat_history_next(&mut self) {
        match self.chat_history.get(self.history_index) {
            Some(hist) => {
                if self.history_index >= self.chat_history.len() - 1 {
                    self.history_index = 0;
                    self.chat_input = hist.to_string();
                } else {
                    self.history_index += 1;
                    self.chat_input = hist.to_string();
                }
            }
            None => {
                self.history_index = 0;
            }
        }
    }

    pub fn chat_history_prev(&mut self) {
        match self.chat_history.get(self.history_index) {
            Some(hist) => {
                self.history_index -= 1;
                self.chat_input = hist.to_string();
            }
            None => {
                self.history_index = 0;
            }
        }
    }
}
