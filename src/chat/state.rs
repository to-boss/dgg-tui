use std::{collections::VecDeque, str::FromStr, sync::mpsc::Sender};

use tungstenite::Message;

use crate::ui::window::{Window, WindowList, WindowType};

use super::{
    event::{Action, Event},
    message::ChatMessage,
    user::UserList,
};

pub struct State {
    pub io_sender: Sender<Action>,
    pub username: String,
    pub chat_input: String,
    pub ul: UserList,
    pub deque: VecDeque<Event>,
    pub messages: Vec<ChatMessage>,
    pub windows: WindowList,
    pub message_to_send: Option<String>,
    pub debugs: Vec<String>,
}

impl State {
    pub fn new(max_messages: u16, username: String, io_sender: Sender<Action>) -> State {
        let ul = UserList::new();
        let deque = VecDeque::new();
        let messages = Vec::new();
        let debugs = Vec::new();
        let chat_input = String::new();
        let windows = WindowList {
            windows: vec![
                Window::new(WindowType::Chat, true, max_messages),
                Window::new(WindowType::ChatInput, true, 2),
                Window::new(WindowType::Debug, false, 30),
                Window::new(WindowType::UserList, false, 50),
            ],
        };

        State {
            io_sender,
            username,
            chat_input,
            ul,
            deque,
            messages,
            windows,
            message_to_send: None,
            debugs,
        }
    }

    pub fn dispatch(&self, action: Action) {
        self.io_sender.send(action).unwrap();
    }

    pub fn add_message(&mut self, msg: ChatMessage) {
        if self.messages.len() >= self.windows.get(WindowType::Chat).max_displays.into() {
            self.messages.drain(0..1);
        }
        self.messages.push(msg);
    }

    pub fn push_ui_events(&mut self, events: &mut VecDeque<Event>) {
        self.deque.append(events);
    }

    pub fn push_new_event(&mut self, action: &str, body: String) {
        todo!();
        // let act = Action::from_str(action).unwrap();
        // self.deque.push_back(Event::new(act, body));
    }

    pub fn push_event(&mut self, event: Event) {
        self.deque.push_back(event);
    }

    pub fn pop_event(&mut self) -> Option<Event> {
        self.deque.pop_front()
    }

    pub fn add_debug(&mut self, s: String) {
        if self.debugs.len() >= self.windows.get(WindowType::Debug).max_displays.into() {
            self.debugs.drain(0..1);
        }
        self.debugs.push(format!("DEBUG: {}", s));
    }
}
