use crossterm::event::poll;
use std::{io, time};

use untitled::app::SPEED;
use untitled::app::game::Game;
use untitled::app::draw::{lose_message, render_map};
use untitled::app::user_input::match_event;

fn main() -> io::Result<()> {
    let mut game = Game::new();

    loop {
        if poll(time::Duration::from_millis(SPEED))? {
            match_event(&mut game).expect("TODO: panic message");
        } else {
            if game.move_snake().is_err() {
                lose_message(&game);
                break;
            }
            render_map(&game);
        }
    }
    Ok(())
}