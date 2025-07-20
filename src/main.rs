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
            match_event(&mut game).expect("Error while reading user input");
        } else {
            match game.move_snake() {
                Ok(_) => render_map(&game),
                Err(err) => {
                    lose_message(&game, &err.to_string());
                    break;
                }
            }

            render_map(&game);
        }
    }
    Ok(())
}