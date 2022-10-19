use std::fmt::Display;

use super::{message::ChatMessage, user::UserList};
pub struct Event {
    pub action: Action,
    pub body: String,
}

impl Event {
    pub fn new(action: Action, body: String) -> Self {
        Event { action, body }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.action, self.body)
    }
}

#[derive(Debug)]
pub enum Action {
    Key(char),
    Stalk(String, usize),
    QuitApp,
    ScrollUp,
    ScrollDown,
    GetChatHistory,
    GetEmbeds,
    ChangeDebug,
    ChangeUserList,
    RecvMsg(ChatMessage),
    SendMsg,
    UserJoin,
    UserQuit,
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
        match prefix {
            "MSG" => Action::RecvMsg(ChatMessage::from_json(json)),
            "JOIN" => Action::UserJoin,
            "QUIT" => Action::UserQuit,
            "NAMES" => Action::UsersInit(UserList::from_json(json).unwrap()),
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

// impl FromStr for Action {
//     type Err = ();
//     fn from_str(s: &str) -> Result<Self, <Action as FromStr>::Err> {
//         match s {
//             // "MSG" => Ok(Action::RecvMsg()),
//             "JOIN" => Ok(Action::UserJoin),
//             "QUIT" => Ok(Action::UserQuit),
//             "NAMES" => Ok(Action::UsersInit),
//             "MUTE" => Ok(Action::Mute),
//             "UNMUTE" => Ok(Action::Unmute),
//             "BAN" => Ok(Action::Ban),
//             "UNBAN" => Ok(Action::Unban),
//             "SUBONLY" => Ok(Action::Subonly),
//             "BROADCAST" => Ok(Action::Broadcast),
//             "PRIVMSG" => Ok(Action::PrivMsg),
//             "PRIVMSGSENT" => Ok(Action::PrivMsg),
//             "PING" => Ok(Action::Ping),
//             "PONG" => Ok(Action::Pong),
//             "ERR" => Ok(Action::Err),
//             "REFRESH" => Ok(Action::Refresh),
//             "Binary" => Ok(Action::Binary),
//             _ => Err(()),
//         }
//     }
// }
