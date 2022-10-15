use std::{
    net::TcpStream,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex, MutexGuard,
    },
};

use tungstenite::{connect, http::Request, stream::MaybeTlsStream, WebSocket};

use super::state::State;

pub struct DGG {
    pub ws: WebSocket<MaybeTlsStream<TcpStream>>,
    pub rx: Receiver<String>,
    pub state: Arc<Mutex<State>>,
    pub token: String,
}

impl DGG {
    pub fn new(max_massages: usize) -> (Self, Sender<String>) {
        let name = String::from("onlyclose");
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

        let state = Arc::new(Mutex::new(State::new(max_massages, name)));
        let (tx, rx) = mpsc::channel();

        (
            DGG {
                ws,
                state,
                token,
                rx,
            },
            tx,
        )
    }

    pub fn get_state_ref(&self) -> Arc<Mutex<State>> {
        Arc::clone(&self.state)
    }

    pub fn work(&mut self) {
        loop {
            // Check if we want to send a message
            match self.rx.try_recv() {
                Ok(msg) => self.ws.write_message(msg.into()).unwrap(),
                Err(mpsc::TryRecvError::Empty) => (),
                Err(mpsc::TryRecvError::Disconnected) => break,
            }

            // Receiving from WebSocket
            let msg = match self.ws.read_message() {
                Ok(msg) => msg.to_string(),
                Err(_) => String::new(),
            };

            // Adds Event from WebSocket to Queue
            if !msg.is_empty() {
                let msg_splits: Vec<String> = msg.splitn(2, " ").map(|s| s.to_owned()).collect();

                let (prefix, json) = (msg_splits[0].clone(), msg_splits[1].clone());
                let mut state = self.state.lock().unwrap();
                state.push_new_event(&prefix, json);
            }
        }
    }

    pub fn parse_ws_message(msg: &str, state: &mut MutexGuard<State>) {
        if !msg.is_empty() {
            let msg_splits: Vec<String> = msg.splitn(2, " ").map(|s| s.to_owned()).collect();

            let (prefix, json) = (msg_splits[0].clone(), msg_splits[1].clone());
            state.push_new_event(&prefix, json);
        }
    }
}
