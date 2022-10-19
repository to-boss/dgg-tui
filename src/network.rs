use std::sync::{mpsc::Sender, Arc, RwLock};

use futures::{channel::mpsc::Receiver, lock, SinkExt, StreamExt};
use tokio::sync::Mutex;
use tokio_tungstenite::connect_async;
use tungstenite::{handshake::client::Request, Message};

use crate::chat::{api::ApiCaller, event::Action, state::State};

pub struct Network<'a> {
    state: &'a Arc<Mutex<State>>,
    api_caller: ApiCaller,
    chat_msg_sender: futures::channel::mpsc::Sender<Message>,
    running: Arc<RwLock<bool>>,
}

impl<'a> Network<'a> {
    pub fn new(
        state: &'a Arc<Mutex<State>>,
        chat_msg_sender: futures::channel::mpsc::Sender<Message>,
    ) -> Network {
        let api_caller = ApiCaller::new();
        let running = Arc::new(RwLock::new(false));
        Network {
            state,
            api_caller,
            chat_msg_sender,
            running,
        }
    }

    pub async fn start_websocket(
        &mut self,
        io_sender: Sender<Action>,
        chat_recv: Receiver<Message>,
    ) {
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

        let (ws_stream, _) = connect_async(request)
            .await
            .expect("Failed to connect to WebSocket.");

        let mut running = self.running.write().unwrap();
        *running = true;
        drop(running);
        let c1_running = Arc::clone(&self.running);
        let c2_running = Arc::clone(&self.running);

        let (write, mut read) = ws_stream.split();

        tokio::spawn(async move {
            loop {
                if *c1_running.read().unwrap() {
                    if let Some(Ok(msg)) = read.next().await {
                        if let Message::Text(text) = msg {
                            io_sender.send(parse_msg(text)).unwrap();
                        }
                    }
                } else {
                    break;
                }
            }
        });

        tokio::spawn(async move {
            if *c2_running.read().unwrap() {
                chat_recv.map(Ok).forward(write).await.unwrap();
            }
        });
    }

    fn close(&mut self) {
        let mut running = self.running.write().unwrap();
        *running = true;
    }

    async fn stalk(&mut self, name: String, num: usize) {
        match self.api_caller.stalk(name, num).await {
            Ok(stalks) => {
                let mut state = self.state.lock().await;
                stalks
                    .iter()
                    .for_each(|msg| state.add_debug(msg.to_string()));
            }
            Err(err) => self.state.lock().await.add_error(err.to_string()),
        }
    }

    async fn get_chat_history(&self) {
        match self.api_caller.get_chat_history().await {
            Ok(chat_history) => {
                let state = self.state.lock().await;
                chat_history
                    .into_iter()
                    .take(50)
                    .for_each(|msg| state.dispatch(parse_msg(msg)));
            }
            Err(err) => self.state.lock().await.add_error(err.to_string()),
        }
    }

    async fn send_chat_message(&mut self) {
        let mut state = self.state.lock().await;
        let msg = format!(r#"MSG {{"data":"{}"}}"#, state.chat_input);
        state.chat_input.clear();
        drop(state);

        let msg = Message::Text(msg);
        self.chat_msg_sender.send(msg).await.unwrap();
    }

    pub async fn handle_io(&mut self, action: Action) {
        self.state.lock().await.add_debug(action.to_string());
        match action {
            Action::Key(_) => (),
            Action::Stalk(name, num) => self.stalk(name, num).await,
            Action::QuitApp => self.close(),
            Action::ScrollUp => (),
            Action::ScrollDown => (),
            Action::GetChatHistory => self.get_chat_history().await,
            Action::GetEmbeds => (),
            Action::ChangeDebug => (),
            Action::ChangeUserList => (),
            Action::RecvMsg(chat_msg) => self.state.lock().await.add_message(chat_msg),
            Action::SendMsg => self.send_chat_message().await,
            Action::UserJoin => (),
            Action::UserQuit => (),
            Action::UsersInit(mut user_list) => self.state.lock().await.ul.append(&mut user_list),
            Action::Mute => (),
            Action::Unmute => (),
            Action::Ban => (),
            Action::Unban => (),
            Action::Subonly => (),
            Action::Broadcast => (),
            Action::PrivMsg => (),
            Action::Ping => (),
            Action::Pong => (),
            Action::Refresh => (),
            Action::Binary => (),
            Action::Err(err_msg) => self.state.lock().await.add_error(err_msg.to_string()),
            Action::Unreachable(un_msg) => self
                .state
                .lock()
                .await
                .add_error(format!("NETWORK: Unreachable = {}", un_msg)),
        }
    }
}

pub fn parse_msg(msg: String) -> Action {
    let msg_splits: Vec<String> = msg
        .to_string()
        .splitn(2, " ")
        .map(|s| s.to_string())
        .collect();

    let (prefix, json) = (&msg_splits[0], &msg_splits[1]);
    Action::from_prefix_and_json(&prefix, &json)
}
