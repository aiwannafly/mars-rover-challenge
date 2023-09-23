use std::error::Error;
use std::fmt::{Debug};
use std::str::FromStr;
use itertools::Itertools;
use crate::direction::Direction;
use crate::map::SimpleMap;
use crate::position::Position;
use crate::rover::Rover;
use crate::rover_command::Command;
use crate::rover_manager::RoverManager;

mod direction;
mod rover;
mod rover_command;
mod position;
mod rover_manager;
mod map;

fn read_two_nums<T>(line: &str) -> Result<(T, T), Box<dyn Error>> where <T as FromStr>::Err: Debug, T: FromStr {
    return line.split(" ")
        .map(|s| s.trim())
        .take(2)
        .map(|s| s.parse::<T>().unwrap())
        .collect_tuple().ok_or(Box::new(std::fmt::Error));
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let mut grid_size_line = String::new();

    stdin.read_line(&mut grid_size_line)?;
    let (width, height) = read_two_nums::<u32>(&grid_size_line)?;

    let map = SimpleMap::new(height, width);

    let mut rover_managers = Vec::new();
    loop {
        let mut start_pos_line = String::new();
        if let Err(_) = stdin.read_line(&mut start_pos_line) {
            break;
        }
        if start_pos_line.trim().is_empty() {
            break;
        }
        let (x, y) = read_two_nums(&start_pos_line)?;
        let start_dir = Direction::from_string(start_pos_line.split(' ')
            .last()
            .ok_or(Box::new(std::fmt::Error))?
            .trim())?;
        let mut commands_line = String::new();
        stdin.read_line(&mut commands_line)?;

        let commands = commands_line.trim().chars()
            .map(|c| c.to_string())
            .map(|c| Command::from_string(c).unwrap())
            .collect::<Vec<Command>>();

        rover_managers.push(RoverManager::new(
            Rover::new(start_dir, Position::new(x, y)),
                commands,
                Box::new(&map),
            )
        );
    }

    for mut rover_manager in rover_managers {
        let final_pos = rover_manager.execute_mission().expect("Failed to execute mission");
        println!("{} {}", final_pos.x, final_pos.y)
    }
    Ok(())
}
