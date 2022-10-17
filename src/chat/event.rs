use std::{fmt::Display, str::FromStr};
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
    Stalk(String),
    QuitApp,
    ScrollUp,
    ScrollDown,
    GetChatHistory,
    GetEmbeds,
    ChangeDebug,
    ChangeUserList,
    RecvMsg,
    SendMsg,
    UserJoin,
    UserQuit,
    UsersInit,
    Mute,
    Unmute,
    Ban,
    Unban,
    Subonly,
    Broadcast,
    PrivMsg,
    Ping,
    Pong,
    Err,
    Refresh,
    Binary,
}

impl FromStr for Action {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, <Action as FromStr>::Err> {
        match s {
            "MSG" => Ok(Action::RecvMsg),
            "JOIN" => Ok(Action::UserJoin),
            "QUIT" => Ok(Action::UserQuit),
            "NAMES" => Ok(Action::UsersInit),
            "MUTE" => Ok(Action::Mute),
            "UNMUTE" => Ok(Action::Unmute),
            "BAN" => Ok(Action::Ban),
            "UNBAN" => Ok(Action::Unban),
            "SUBONLY" => Ok(Action::Subonly),
            "BROADCAST" => Ok(Action::Broadcast),
            "PRIVMSG" => Ok(Action::PrivMsg),
            "PRIVMSGSENT" => Ok(Action::PrivMsg),
            "PING" => Ok(Action::Ping),
            "PONG" => Ok(Action::Pong),
            "ERR" => Ok(Action::Err),
            "REFRESH" => Ok(Action::Refresh),
            "Binary" => Ok(Action::Binary),
            _ => Err(()),
        }
    }
}
