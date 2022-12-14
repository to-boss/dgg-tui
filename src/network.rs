use crate::chat::{action::Action, api::ApiCaller, message::ChatMessage, state::State};
use futures::{channel::mpsc::Receiver, SinkExt, StreamExt};
use reqwest::StatusCode;
use std::sync::{mpsc::Sender, Arc};
use tokio::sync::Mutex;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{self, handshake::client::Request, Message},
};

pub struct Network<'a> {
    state: &'a Arc<Mutex<State>>,
    api_caller: ApiCaller<'a>,
    token: &'a str,
    chat_msg_sender: futures::channel::mpsc::Sender<Message>,
}

impl<'a> Network<'a> {
    pub fn new(
        token: &'a str,
        state: &'a Arc<Mutex<State>>,
        chat_msg_sender: futures::channel::mpsc::Sender<Message>,
    ) -> Network<'a> {
        let api_caller = ApiCaller::new(&token);
        Network {
            state,
            api_caller,
            token,
            chat_msg_sender,
        }
    }

    pub async fn start_websocket(
        &mut self,
        io_sender: Sender<Action>,
        chat_recv: Receiver<Message>,
    ) {
        let socket_url = "wss://destiny.gg/ws";
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
            .header("cookie", format!("authtoken={}", self.token))
            .uri(socket_url)
            .body(())
            .unwrap();

        let (ws_stream, res) = connect_async(request)
            .await
            .expect("Failed to connect to WebSocket.");

        // TODO: proper error handling
        match res.status() {
            StatusCode::OK => (),
            _ => (),
        }

        let (write, mut read) = ws_stream.split();

        tokio::spawn(async move {
            loop {
                match read.next().await.unwrap() {
                    Ok(msg) => match msg {
                        Message::Text(text) => {
                            io_sender.send(parse_msg(&text)).unwrap();
                        }
                        _ => (),
                    },
                    Err(_) => break,
                }
            }
        });

        tokio::spawn(async move {
            chat_recv.map(Ok).forward(write).await.unwrap();
        });
    }

    fn close(&mut self) {}

    async fn get_last_embeds(&mut self) {
        match self.api_caller.get_last_embeds().await {
            Ok(embeds) => {
                let mut state = self.state.lock().await;
                embeds.iter().for_each(|msg| {
                    state.add_message(ChatMessage::from_string(
                        "EMBED".to_string(),
                        msg.to_string(),
                    ))
                });
            }
            Err(err) => self.state.lock().await.add_error(err.to_string()),
        }
    }

    async fn stalk(&mut self, name: String, num: u8) {
        match self.api_caller.stalk(name, num).await {
            Ok(stalks) => {
                let mut state = self.state.lock().await;
                stalks.iter().for_each(|msg| {
                    state.add_message(ChatMessage::from_string(
                        "STALK".to_string(),
                        msg.to_string(),
                    ))
                });
            }
            Err(err) => self.state.lock().await.add_error(err.to_string()),
        }
    }

    async fn get_chat_history(&self) {
        match self.api_caller.get_chat_history().await {
            Ok(chat_history) => {
                let mut state = self.state.lock().await;
                chat_history[chat_history.len() - 50..]
                    .into_iter()
                    .for_each(|msg| state.dispatch(parse_msg(msg)));
                state.loaded = true;
            }
            Err(err) => self.state.lock().await.add_error(err.to_string()),
        }
    }

    async fn get_me(&self) {
        match self.api_caller.get_me().await {
            Ok(me) => {
                self.state.lock().await.username = me.username;
            }
            Err(err) => self.state.lock().await.add_error(err.to_string()),
        }
    }

    async fn send_chat_message(&mut self) {
        let mut state = self.state.lock().await;
        let msg = format!(r#"MSG {{"data":"{}"}}"#, state.chat_input.current_message);
        state.chat_input.add();
        drop(state);

        let msg = Message::Text(msg);
        self.chat_msg_sender.send(msg).await.unwrap();
    }

    pub async fn handle_io(&mut self, action: Action) {
        self.state.lock().await.add_debug(action.to_string());
        match action {
            Action::RecvMsg(mut chat_msg) => {
                let mut state = self.state.lock().await;
                chat_msg.parse(&state.username);
                state.add_message(chat_msg)
            }
            Action::Stalk(name, num) => self.stalk(name, num).await,
            Action::QuitApp => self.close(),
            Action::GetChatHistory => self.get_chat_history().await,
            Action::GetMe => self.get_me().await,
            Action::GetEmbeds => self.get_last_embeds().await,
            Action::SendMsg => self.send_chat_message().await,
            Action::UserJoin(user) => self.state.lock().await.ul.add(user),
            Action::UserQuit(user) => self.state.lock().await.ul.remove(user),
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
            Action::Err(ws_err) => self.state.lock().await.add_error(ws_err.to_string()),
            Action::Unreachable(un_msg) => self
                .state
                .lock()
                .await
                .add_error(format!("NETWORK: Unreachable = {}", un_msg)),
        }
    }
}

pub fn parse_msg(msg: &String) -> Action {
    let msg_splits: Vec<String> = msg
        .to_string()
        .splitn(2, " ")
        .map(|s| s.to_string())
        .collect();

    let (prefix, json) = (&msg_splits[0], &msg_splits[1]);
    Action::from_prefix_and_json(&prefix, &json)
}
