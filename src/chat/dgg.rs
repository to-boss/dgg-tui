use std::{
    net::TcpStream,
    sync::{
        mpsc::{self, Receiver, Sender, TryRecvError},
        Arc, Mutex,
    },
};

use tungstenite::{connect, stream::MaybeTlsStream, WebSocket};
use url;

use super::state::State;

pub struct DGG {
    pub ws: WebSocket<MaybeTlsStream<TcpStream>>,
    pub state: Arc<Mutex<State>>,
    pub receiver: Receiver<String>,
    pub debug: bool,
}

impl DGG {
    pub fn new(max_massages: usize) -> (Self, Sender<String>) {
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
                Ok(msg_to_send) => {
                    self.state
                        .lock()
                        .unwrap()
                        .push_new_event("SendMsg", msg_to_send);
                }
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => break,
            }
        }
    }

    pub fn debug_on(&mut self) {
        self.debug = true;
    }
}
