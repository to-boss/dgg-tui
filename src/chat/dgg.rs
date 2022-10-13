use std::{
    net::TcpStream,
    sync::{
        mpsc::{self, Receiver, Sender, TryRecvError},
        Arc, Mutex,
    },
};

use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use url;

use super::state::State;

pub struct DGG {
    pub ws: WebSocket<MaybeTlsStream<TcpStream>>,
    pub state: Arc<Mutex<State>>,
    pub receiver: Receiver<usize>,
    pub debug: bool,
    pub token: String,
}

impl DGG {
    pub fn new(max_massages: usize) -> (Self, Sender<usize>) {
        let token =
            String::from("251rLOxzq4M9GSsW52DVIZVFvGqDhOSP4wG7pMkTYJO0VH5l32FKQoQOuzuduhGt");

        // TODO: Check for errors and no unwrap?
        let url = url::Url::parse("wss://chat.destiny.gg/ws").unwrap();

        let (ws, _res) = connect(url).expect("Failed to connect to WebSocket.\n");
        // println!("Successfully connected to DGG 😍.");

        let state = Arc::new(Mutex::new(State::new(max_massages)));
        let (sender, receiver) = mpsc::channel();

        (
            DGG {
                ws,
                state,
                receiver,
                debug: false,
                token,
            },
            sender,
        )
    }

    pub fn get_state_ref(&self) -> Arc<Mutex<State>> {
        Arc::clone(&self.state)
    }

    pub fn work(&mut self) {
        loop {
            // Receiving from WebSocket
            // blocking
            let msg = match self.ws.read_message() {
                Ok(val) => val.to_string(),
                Err(err) => panic!("Can't read message! {}", err),
            };

            let msg_splits: Vec<String> = msg.splitn(2, " ").map(|s| s.to_owned()).collect();
            let (prefix, json) = (msg_splits[0].clone(), msg_splits[1].clone());
            self.state.lock().unwrap().push_new_event(&prefix, json);

            // Sending to WebSocket
            // non blocking
            match self.receiver.try_recv() {
                Ok(val) => match val {
                    0 => {
                        let mut state = self.state.lock().unwrap();
                        state.users_window = !state.users_window;
                    }
                    1 => {
                        let mut state = self.state.lock().unwrap();
                        self.ws
                            .write_message(Message::Text(state.message_to_send.clone()))
                            .unwrap();
                        state.send_message = false;
                        //state.message_to_send;
                    }
                    _ => {}
                },
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => break,
            }
        }
    }

    pub fn debug_on(&mut self) {
        self.debug = true;
    }
}
