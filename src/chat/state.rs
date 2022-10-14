use std::{collections::VecDeque, str::FromStr};

use super::{
    event::{Action, Event},
    message::Message,
    user::UserList,
};

pub struct State {
    pub username: String,
    pub ul: UserList,
    pub deque: VecDeque<Event>,
    pub max_messages: usize,
    pub messages: Vec<Message>,
    pub windows: Vec<Window>,
    pub message_to_send: Option<String>,
    pub debugs: Vec<String>,
}

impl State {
    pub fn new(max_messages: usize, username: String) -> State {
        let ul = UserList::new();
        let deque = VecDeque::new();
        let messages = Vec::new();
        let mut debugs = Vec::new();
        let windows = vec![Window::new("Debug", false), Window::new("UserList", true)];

        debugs.push(String::from("H1"));
        debugs.push(String::from("H2"));
        debugs.push(String::from("H3"));

        State {
            username,
            ul,
            deque,
            max_messages,
            messages,
            windows,
            message_to_send: None,
            debugs,
        }
    }

    pub fn add_send_message(&mut self, send_msg: String) {
        self.message_to_send = Some(send_msg);
    }

    pub fn add_message(&mut self, msg: Message) {
        if self.messages.len() >= self.max_messages {
            self.messages.drain(0..1);
        }
        self.messages.push(msg);
    }

    pub fn push_ui_events(&mut self, events: &mut VecDeque<Event>) {
        self.deque.append(events);
    }

    pub fn push_new_event(&mut self, action: &str, body: String) {
        let act = Action::from_str(action).unwrap();
        self.deque.push_back(Event::new(act, body));
    }

    pub fn push_event(&mut self, event: Event) {
        self.deque.push_back(event);
    }

    pub fn pop_event(&mut self) -> Option<Event> {
        self.deque.pop_front()
    }

    pub fn add_debug(&mut self, s: String) {
        self.debugs.push(s);
    }
}

#[derive(PartialEq, Eq)]
pub struct Window {
    pub name: &'static str,
    pub active: bool,
}

impl Window {
    pub fn new(name: &'static str, active: bool) -> Self {
        Window { name, active }
    }

    pub fn flip(&mut self) {
        self.active = !self.active;
    }
}
