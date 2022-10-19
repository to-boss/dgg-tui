use std::sync::mpsc::Sender;

use crate::ui::window::{Window, WindowList, WindowType};

use super::{event::Action, message::ChatMessage, user::UserList};

pub struct State {
    pub io_sender: Sender<Action>,
    pub username: String,
    pub chat_input: String,
    pub ul: UserList,
    pub messages: Vec<ChatMessage>,
    pub message_to_send: Option<String>,
    pub windows: WindowList,
    pub debugs: Vec<String>,
}

impl State {
    pub fn new(max_messages: u16, username: String, io_sender: Sender<Action>) -> State {
        let ul = UserList::new();
        let messages = Vec::new();
        let debugs = Vec::new();
        let chat_input = String::new();
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
}
