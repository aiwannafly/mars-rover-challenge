use std::io;
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    TurnLeft,
    TurnRight,
    Move,
}

impl FromStr for Command {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Command::TurnLeft),
            "R" => Ok(Command::TurnRight),
            "M" => Ok(Command::Move),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Unexpected {} token for Command", s))),
        }
    }
}
