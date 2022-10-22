use std::fmt::Display;

use super::{
    message::ChatMessage,
    user::{User, UserList},
};

#[derive(Debug)]
pub enum Action {
    Key(char),
    Stalk(String, u8),
    QuitApp,
    ScrollUp,
    ScrollDown,
    GetChatHistory,
    GetMe,
    GetEmbeds,
    ChangeDebug,
    ChangeUserList,
    RecvMsg(ChatMessage),
    SendMsg,
    UserJoin(User),
    UserQuit(User),
    UsersInit(UserList),
    Mute,
    Unmute,
    Ban,
    Unban,
    Subonly,
    Broadcast,
    PrivMsg,
    Ping,
    Pong,
    Refresh,
    Binary,
    Err(String),
    Unreachable(String),
}

impl Action {
    pub fn from_prefix_and_json(prefix: &str, json: &str) -> Action {
        // These actions come from the websocket only!
        match prefix {
            "MSG" => Action::RecvMsg(ChatMessage::from_json(json)),
            "JOIN" => Action::UserJoin(User::from_json(json)),
            "QUIT" => Action::UserQuit(User::from_json(json)),
            "NAMES" => Action::UsersInit(UserList::from_json(json)),
            "MUTE" => Action::Mute,
            "UNMUTE" => Action::Unmute,
            "BAN" => Action::Ban,
            "UNBAN" => Action::Unban,
            "SUBONLY" => Action::Subonly,
            "BROADCAST" => Action::Broadcast,
            "PRIVMSG" => Action::PrivMsg,
            "PRIVMSGSENT" => Action::PrivMsg,
            "PING" => Action::Ping,
            "PONG" => Action::Pong,
            "REFRESH" => Action::Refresh,
            "Binary" => Action::Binary,
            "ERR" => Action::Err(json.to_string()),
            _ => Action::Unreachable(json.to_string()),
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "{:?}", self),
        }
    }
}
