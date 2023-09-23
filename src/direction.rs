use std::error::Error;

pub enum Direction {
    North,
    East,
    South,
    West
}

impl Direction {
    pub fn from_string(value: &str) -> Result<Self, Box<dyn Error>> {
        return match value {
            "N" => Ok(Direction::North),
            "E" => Ok(Direction::East),
            "S" => Ok(Direction::South),
            "W" => Ok(Direction::West),
            _ => Err(Box::try_from(format!("Invalid argument for Direction: {value}")).unwrap())
        };
    }
}
