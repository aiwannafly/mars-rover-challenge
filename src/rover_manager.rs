use crate::map::Map;
use crate::position::Position;
use crate::rover::Rover;
use crate::rover_command::Command;

pub struct RoverManager<'a> {
    rover: Rover,
    commands: Vec<Command>,
    map: &'a dyn Map,
}

impl<'b> RoverManager<'b> {
    pub fn new(rover: Rover, commands: Vec<Command>, map: &'b dyn Map) -> Self {
        RoverManager {
            rover,
            commands,
            map,
        }
    }

    pub fn execute_mission(&mut self) -> Result<&Position, &'static str> {
        for command in &self.commands {
            self.rover.execute(command);

            self.map.place_object(&self.rover.pos)?;
        }
        Ok(&self.rover.pos)
    }
}
