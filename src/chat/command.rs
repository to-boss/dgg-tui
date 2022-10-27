use anyhow::bail;
use anyhow::Result;
use std::fmt::Display;

use super::action::Action;

#[derive(Debug)]
pub enum Command {
    Stalk(String, u8),
    Embeds,
}

pub fn parse_command_to_action(s: &String) -> Result<Action> {
    let whitespaces: Vec<&str> = s.split_whitespace().collect();
    let command = &whitespaces[0][1..];
    match command {
        "stalk" => match whitespaces.len() {
            2 => Ok(Action::Stalk(whitespaces[1].to_string(), 15)),
            3 => Ok(Action::Stalk(
                whitespaces[1].to_string(),
                whitespaces[2].parse::<u8>()?,
            )),
            _ => bail!("Invalid stalk usage! /stalk [name] [number]."),
        },
        "embeds" => Ok(Action::GetEmbeds),
        _ => bail!("Command not found."),
    }
}

impl Command {
    pub fn vec() -> Vec<String> {
        vec!["/stalk".to_string(), "/embeds".to_string()]
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "{:?}", self),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn command_strings_equals() {}
}
