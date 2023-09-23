use crate::position::Position;

pub trait Map {
    fn place_object(&self, pos: &Position) -> Result<(), &'static str>;
}

pub struct SimpleMap {
    height: u32,
    width: u32,
}

impl SimpleMap {
    pub fn new(height: u32, width: u32) -> Self {
        SimpleMap { height, width }
    }
}

impl Map for SimpleMap {
    fn place_object(&self, pos: &Position) -> Result<(), &'static str> {
        if pos.x >= self.width || pos.y >= self.height {
            Err("Out of bounds error")
        } else {
            Ok(())
        }
    }
}
