use std::io;
use crossterm::event::{read, KeyCode};
use crate::app::Direction;
use crate::app::game::Game;

pub fn match_event(game: &mut Game) -> io::Result<()> {
    let event = read().expect("err read key");
    if let Some(key) = event.as_key_press_event() {
        let new_direction = match key.code {
            KeyCode::Char('w') => Some(Direction::Up),
            KeyCode::Char('s') => Some(Direction::Down),
            KeyCode::Char('a') => Some(Direction::Left),
            KeyCode::Char('d') => Some(Direction::Right),
            _ => None
        };

        if let Some(new_direction) = new_direction {
            if !are_opposite_directions(&new_direction, &game.direction) {
                game.direction = new_direction;
            }
        }
    }
    Ok(())
}

fn are_opposite_directions(new: &Direction, current: &Direction) -> bool {
    matches!(
        (new, current),
        (Direction::Up, Direction::Down)
        | (Direction::Down, Direction::Up)
        | (Direction::Left, Direction::Right)
        | (Direction::Right, Direction::Left)
    )
}
