use std::str::FromStr;
pub struct Event {
    pub action: Action,
    pub body: String,
}

impl Event {
    pub fn new(action: Action, body: String) -> Self {
        Event { action, body }
    }
}

pub enum Action {
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

impl Action {
    pub fn consume(self) {
        // match prefix {
        //     "MSG" => {
        //         let msg = Message::from_json(json).unwrap();
        //         c_dbg!(self, msg.to_string());
        //         self.state.lock().unwrap().add_message(msg);
        //     }
        //     "JOIN" => {
        //         let mut state = self.state.lock().unwrap();
        //         state.ul.add(User::from_json(json).unwrap());
        //         c_dbg!(self, "User joined.");
        //     }
        //     "QUIT" => {
        //         let mut state = self.state.lock().unwrap();
        //         state.ul.remove(User::from_json(json).unwrap());
        //         c_dbg!(self, "User quit.");
        //     }
        //     "NAMES" => {
        //         let mut ul = UserList::from_json(json).unwrap();
        //         println!(
        //             "Connected. Serving {} connections and {} users.",
        //             ul.conn_count,
        //             ul.users.len()
        //         );
        //         let mut state = self.state.lock().unwrap();
        //         state.ul.append(&mut ul);
        //     }
        //     "MUTE" => println!("{:#?}::{:#?}", msg, json),
        //     "UNMUTE" => println!("{:#?}::{:#?}", msg, json),
        //     "BAN" => println!("{:#?}::{:#?}", msg, json),
        //     "UNBAN" => println!("{:#?}::{:#?}", msg, json),
        //     "SUBONLY" => println!("{:#?}::{:#?}", msg, json),
        //     "BROADCAST" => println!("{:#?}::{:#?}", msg, json),
        //     "PRIVMSG" => println!("{:#?}::{:#?}", msg, json),
        //     "PRIVMSGSENT" => println!("{:#?}::{:#?}", msg, json),
        //     "PING" => println!("{:#?}::{:#?}", msg, json),
        //     "PONG" => println!("{:#?}::{:#?}", msg, json),
        //     "ERR" => println!("{:#?}::{:#?}", msg, json),
        //     "REFRESH" => println!("{:#?}::{:#?}", msg, json),
        //     "Binary" => (),
        //     _ => panic!("Couldn't find prefix."),
        // }
    }
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
