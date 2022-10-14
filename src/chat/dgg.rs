use std::{
    net::TcpStream,
    sync::{
        mpsc::{self, Receiver, Sender, TryRecvError},
        Arc, Mutex,
    },
};

use tungstenite::{connect, http::Request, stream::MaybeTlsStream, Message, WebSocket};
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

        let request = Request::builder()
            .header("Host", "chat.destiny.gg")
            .header("Origin", "https://www.destiny.gg")
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header(
                "Sec-WebSocket-Key",
                tungstenite::handshake::client::generate_key(),
            )
            .header("cookie", format!("authtoken={}", token))
            .uri("wss://destiny.gg/ws")
            .body(())
            .unwrap();

        let (ws, _res) = connect(request).expect("Failed to connect to WebSocket.\n");

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
            let msg = match self.ws.read_message() {
                Ok(msg) => msg.to_string(),
                Err(_) => String::new(),
            };

            // Adds Event from WebSocket to Queue
            let mut state = self.state.lock().unwrap();
            if !msg.is_empty() {
                let msg_splits: Vec<String> = msg.splitn(2, " ").map(|s| s.to_owned()).collect();

                let (prefix, json) = (msg_splits[0].clone(), msg_splits[1].clone());
                state.push_new_event(&prefix, json.clone());
            }

            // Check if we want to send a message
            if let Some(msg) = &state.message_to_send {
                self.ws.write_message(msg.clone().into()).unwrap();
                state.message_to_send = None;
            }
        }
    }

    pub fn debug_on(&mut self) {
        self.debug = true;
    }
}
