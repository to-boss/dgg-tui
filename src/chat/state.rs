use super::{message::Message, user::UserList};

pub struct State {
    pub ul: UserList,
    pub max_messages: usize,
    pub inc_messages: Vec<Message>,
    pub out_message: Option<String>,
}

impl State {
    pub fn new(max_messages: usize) -> State {
        let ul = UserList::new();
        let inc_messages = Vec::new();
        let out_message = None;

        State {
            ul,
            max_messages,
            inc_messages,
            out_message,
        }
    }

    pub fn add_message(&mut self, msg: Message) {
        if self.inc_messages.len() >= self.max_messages {
            self.inc_messages.drain(0..1);
        }
        self.inc_messages.push(msg);
    }
}
