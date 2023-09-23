use std::error::Error;

#[derive(Debug)]
pub enum Command {
    TurnLeft,
    TurnRight,
    Move
}

impl Command {
    pub fn from_string(value: String) -> Result<Self, Box<dyn Error>> {
        return match value.as_str() {
            "L" => Ok(Command::TurnLeft),
            "R" => Ok(Command::TurnRight),
            "M" => Ok(Command::Move),
            _ => Err(Box::try_from(format!("Invalid argument for Command: {value}")).unwrap())
        }
    }
}
