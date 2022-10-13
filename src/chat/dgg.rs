use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

use tungstenite::{connect, stream::MaybeTlsStream, WebSocket};
use url;

use crate::chat::user::UserList;

use super::{message::Message, state::State, user::User};

macro_rules! c_dbg {
    ($self:ident, $exp:expr) => {
        if $self.debug {
            println!("{}", $exp);
        }
    };
}
pub struct DGG {
    pub ws: WebSocket<MaybeTlsStream<TcpStream>>,
    pub state: Arc<Mutex<State>>,
    pub debug: bool,
}

impl DGG {
    pub fn new(max_massages: usize) -> Self {
        // TODO: Check for errors and no unwrap?
        let url = url::Url::parse("wss://chat.destiny.gg/ws").unwrap();

        let (ws, _res) = connect(url).expect("Failed to connect to WebSocket.\n");
        println!("Successfully connected to DGG 😍.");

        let state = Arc::new(Mutex::new(State::new(max_massages)));

        DGG {
            ws,
            state,
            debug: false,
        }
    }

    pub fn get_state_ref(&self) -> Arc<Mutex<State>> {
        Arc::clone(&self.state)
    }

    pub fn listen(&mut self) {
        loop {
            let msg = match self.ws.read_message() {
                Ok(val) => val.to_string(),
                Err(err) => panic!("Can't read message! {}", err),
            };

            let msg_splits: Vec<&str> = msg.splitn(2, " ").collect();
            let (prefix, json) = (msg_splits[0], msg_splits[1]);

            match prefix {
                "MSG" => {
                    let msg = Message::from_json(json).unwrap();
                    c_dbg!(self, msg.to_string());
                    self.state.lock().unwrap().add_message(msg);
                }
                "JOIN" => {
                    let mut state = self.state.lock().unwrap();
                    state.ul.add(User::from_json(json).unwrap());
                    c_dbg!(self, "User joined.");
                }
                "QUIT" => {
                    let mut state = self.state.lock().unwrap();
                    state.ul.remove(User::from_json(json).unwrap());
                    c_dbg!(self, "User quit.");
                }
                "NAMES" => {
                    let mut ul = UserList::from_json(json).unwrap();
                    println!(
                        "Connected. Serving {} connections and {} users.",
                        ul.conn_count,
                        ul.users.len()
                    );
                    let mut state = self.state.lock().unwrap();
                    state.ul.append(&mut ul);
                }
                "MUTE" => println!("{:#?}::{:#?}", msg, json),
                "UNMUTE" => println!("{:#?}::{:#?}", msg, json),
                "BAN" => println!("{:#?}::{:#?}", msg, json),
                "UNBAN" => println!("{:#?}::{:#?}", msg, json),
                "SUBONLY" => println!("{:#?}::{:#?}", msg, json),
                "BROADCAST" => println!("{:#?}::{:#?}", msg, json),
                "PRIVMSG" => println!("{:#?}::{:#?}", msg, json),
                "PRIVMSGSENT" => println!("{:#?}::{:#?}", msg, json),
                "PING" => println!("{:#?}::{:#?}", msg, json),
                "PONG" => println!("{:#?}::{:#?}", msg, json),
                "ERR" => println!("{:#?}::{:#?}", msg, json),
                "REFRESH" => println!("{:#?}::{:#?}", msg, json),
                "Binary" => (),
                _ => panic!("Couldn't find prefix."),
            }
        }
    }

    pub fn debug_on(&mut self) {
        self.debug = true;
    }
}
