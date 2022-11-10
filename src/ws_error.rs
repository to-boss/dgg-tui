use std::{fmt::Display, time::Duration};

use serde_json::{Number, Value};

// Taken from https://github.com/destinygg/chat/blob/df26c113dce83af6a6d902d229d7e9f8823e45ea/connection.go
#[derive(Debug)]
pub enum WsError {
    TooManyConnections,
    NeedLogin,
    NoPermission,
    InvalidMsg,
    Submode,
    Throttled,
    ProtocolError,
    UserNotFound,
    NeedBanReason,
    Duplicate,
    Muted(Duration),
    Generic(String),
}

impl WsError {
    pub fn from_json(json: &str) -> WsError {
        let v: Value = serde_json::from_str(json).unwrap();

        if let Value::Number(mute_time_left) = &v["muteTimeLeft"] {
            return Self::from_muted_str(mute_time_left);
        }

        Self::from_error_str(v["description"].as_str().unwrap())
    }

    pub fn from_error_str(err: &str) -> WsError {
        match err {
            "toomanyconnections" => WsError::TooManyConnections,
            "protocolerror" => WsError::ProtocolError,
            "needlogin" => WsError::NeedLogin,
            "duplicate" => WsError::Duplicate,
            "invalidmsg" => WsError::InvalidMsg,
            "nopermission" => WsError::NoPermission,
            "submode" => WsError::Submode,
            "throttled" => WsError::Throttled,
            "notfound" => WsError::UserNotFound,
            "needbanreason" => WsError::NeedBanReason,
            _ => WsError::Generic(format!("err: {}", err)),
        }
    }

    pub fn from_muted_str(num: &Number) -> WsError {
        WsError::Muted(Duration::from_secs(num.as_u64().unwrap()))
    }
}

impl Display for WsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WsError::Muted(mute_time_left) => {
                if *mute_time_left > Duration::from_secs(60) {
                    let minutes = mute_time_left.div_f32(60.);
                    let seconds = *mute_time_left - (Duration::from_secs(minutes.as_secs() * 60));

                    if minutes == Duration::from_secs(1) {
                        write!(
                            f,
                            "You are still muted for {} minute and {} seconds.",
                            minutes.as_secs(),
                            seconds.as_secs(),
                        )
                    } else {
                        write!(
                            f,
                            "You are still muted for {} minutes and {} seconds.",
                            minutes.as_secs(),
                            seconds.as_secs(),
                        )
                    }
                } else {
                    write!(f, "You are still muted for {:?} seconds.", self)
                }
            }
            WsError::TooManyConnections => write!(
                f,
                "You have to many connections to DGG open. The current limit is 5 connections."
            ),
            WsError::NeedLogin => write!(
                f,
                "You are not logged in. Check your config.json file if you should be!"
            ),
            WsError::NoPermission => {
                write!(f, "You do not have the required permissions for that.")
            }
            WsError::InvalidMsg => write!(f, "Your last message was not valid."),
            WsError::Submode => write!(f, "The chat is currently in Subscriber-Only Mode."),
            WsError::Throttled => write!(f, "You are sending messages too fast! Slow down."),
            WsError::ProtocolError => write!(f, "Protocol Error."),
            WsError::UserNotFound => write!(f, "The specified user was not found."),
            WsError::NeedBanReason => write!(f, "You need to specify a ban reason."),
            WsError::Duplicate => write!(f, "You can not send the same message twice!"),
            _ => write!(f, "{:?}", self),
        }
    }
}
