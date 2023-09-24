use std::io;
use std::str::FromStr;

pub enum Direction {
    North,
    East,
    South,
    West,
}

impl FromStr for Direction {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Direction::North),
            "E" => Ok(Direction::East),
            "S" => Ok(Direction::South),
            "W" => Ok(Direction::West),
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Unexpected {} token for Command", s))),
        }
    }
}
