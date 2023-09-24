use crate::rover_command::Command;
use crate::{direction::Direction, position::Position};

pub struct Rover {
    dir: Direction,
    pub pos: Position,
}

impl Rover {
    pub fn new(dir: Direction, pos: Position) -> Self {
        Rover { dir, pos }
    }

    pub fn execute(&mut self, command: &Command) {
        match command {
            Command::Move => match self.dir {
                Direction::North => self.pos.y += 1,
                Direction::West => self.pos.x -= 1,
                Direction::South => self.pos.y -= 1,
                Direction::East => self.pos.x += 1,
            },
            Command::TurnLeft => match self.dir {
                Direction::North => self.dir = Direction::West,
                Direction::West => self.dir = Direction::South,
                Direction::South => self.dir = Direction::East,
                Direction::East => self.dir = Direction::North,
            },
            Command::TurnRight => match self.dir {
                Direction::North => self.dir = Direction::East,
                Direction::East => self.dir = Direction::South,
                Direction::South => self.dir = Direction::West,
                Direction::West => self.dir = Direction::North,
            },
        };
    }
}
