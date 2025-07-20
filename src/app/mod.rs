pub mod game;
pub mod draw;
pub mod user_input;

const MAP_WIDTH: i32 = 20;
const MAP_HEIGHT: i32 = 17;
pub const SPEED: u64 = 300;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn get_offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Cords (i32, i32);

impl Cords {
    pub fn is_within_bounds(&self) -> bool {
        self.0 >= 0 && self.0 < MAP_HEIGHT && self.1 >= 0 && self.1 < MAP_WIDTH
    }
}
