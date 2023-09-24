use crate::direction::Direction;
use crate::map::SimpleMap;
use crate::position::Position;
use crate::rover::Rover;
use crate::rover_command::Command;
use crate::rover_manager::RoverManager;
use itertools::Itertools;
use std::error::Error;
use std::fmt::Debug;
use std::str::FromStr;

mod direction;
mod map;
mod position;
mod rover;
mod rover_command;
mod rover_manager;

fn read_two_nums<T>(line: &str) -> Result<(T, T), Box<dyn Error>>
where
    <T as FromStr>::Err: Debug,
    T: FromStr,
{
    return line
        .split(' ')
        .map(|s| s.trim())
        .take(2)
        .map(|s| s.parse::<T>().unwrap())
        .collect_tuple()
        .ok_or(Box::new(std::fmt::Error));
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
        if stdin.read_line(&mut start_pos_line).is_err() {
            break;
        }
        if start_pos_line.trim().is_empty() {
            break;
        }
        let (x, y) = read_two_nums(&start_pos_line)?;
        let start_dir = Direction::from_str(
            start_pos_line
                .split(' ')
                .last()
                .ok_or(Box::new(std::fmt::Error))?
                .trim(),
        )?;
        let mut commands_line = String::new();
        stdin.read_line(&mut commands_line)?;

        let commands = commands_line
            .trim()
            .chars()
            .map(|c| c.to_string())
            .map(|c| Command::from_str(&c).unwrap())
            .collect::<Vec<Command>>();

        rover_managers.push(RoverManager::new(
            Rover::new(start_dir, Position::new(x, y)),
            commands,
            &map,
        ));
    }

    for mut rover_manager in rover_managers {
        let final_pos = rover_manager
            .execute_mission()
            .expect("Failed to execute mission");
        println!("{} {}", final_pos.x, final_pos.y)
    }
    Ok(())
}
