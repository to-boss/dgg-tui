use std::{collections::VecDeque, str::FromStr};

use super::{
    event::{Action, Event},
    message::Message,
    user::UserList,
};

pub struct State {
    pub ul: UserList,
    pub deque: VecDeque<Event>,
    pub max_messages: usize,
    pub inc_messages: Vec<Message>,
}

impl State {
    pub fn new(max_messages: usize) -> State {
        let ul = UserList::new();
        let deque = VecDeque::new();
        let inc_messages = Vec::new();

        State {
            ul,
            deque,
            max_messages,
            inc_messages,
        }
    }

    pub fn add_message(&mut self, msg: Message) {
        if self.inc_messages.len() >= self.max_messages {
            self.inc_messages.drain(0..1);
        }
        self.inc_messages.push(msg);
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
}
