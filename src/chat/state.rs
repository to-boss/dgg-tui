use std::{collections::VecDeque, str::FromStr};

use super::{
    event::{Action, Event},
    message::Message,
    user::UserList,
};

pub struct State {
    pub username: String,
    pub chat_input: String,
    pub ul: UserList,
    pub deque: VecDeque<Event>,
    pub messages: Vec<Message>,
    pub windows: WindowList,
    pub message_to_send: Option<String>,
    pub debugs: Vec<String>,
}

impl State {
    pub fn new(max_messages: u16, username: String) -> State {
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

    pub fn add_send_message(&mut self, send_msg: String) {
        self.message_to_send = Some(send_msg);
    }

    pub fn add_message(&mut self, msg: Message) {
        if self.messages.len() >= self.windows.get(WindowType::Chat).max_displays.into() {
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
        if self.debugs.len() >= self.windows.get(WindowType::Debug).max_displays.into() {
            self.debugs.drain(0..1);
        }
        self.debugs.push(s);
    }
}

#[derive(PartialEq, Eq)]
pub struct Window {
    pub window_type: WindowType,
    pub active: bool,
    pub auto_scroll: bool,
    pub max_displays: u16,
    pub scroll: u16,
}

impl Window {
    pub fn new(window_type: WindowType, active: bool, max_displays: u16) -> Self {
        Window {
            window_type,
            active,
            auto_scroll: false,
            max_displays,
            scroll: 0,
        }
    }

    pub fn scroll_to_bottom(&self) -> (u16, u16) {
        (self.max_displays - 2, 0)
    }

    pub fn scroll(&mut self, val: i16) {
        let mut scroll = self.scroll as i16;
        if scroll + val >= 0 {
            scroll += val;
        }
        self.scroll = scroll as u16;
    }

    pub fn flip(&mut self) {
        self.active = !self.active;
    }
}

#[derive(PartialEq, Eq)]
pub enum WindowType {
    Chat,
    ChatInput,
    Debug,
    UserList,
}

pub struct WindowList {
    pub windows: Vec<Window>,
}

impl WindowList {
    pub fn get(&self, window_type: WindowType) -> &Window {
        self.windows
            .iter()
            .find(|w| w.window_type == window_type)
            .unwrap()
    }

    pub fn get_mut(&mut self, window_type: WindowType) -> &mut Window {
        self.windows
            .iter_mut()
            .find(|w| w.window_type == window_type)
            .unwrap()
    }
}
