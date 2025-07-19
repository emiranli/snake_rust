use crossterm::event::{read, KeyCode};
use std::io;
use std::sync::{Arc, Mutex};
use std::thread;

use untitled::game::{Direction, Game};

fn main() -> io::Result<()> {
    let direction_mutex = Arc::new(Mutex::new(Direction::Down));
    let direction_mutex_clone = Arc::clone(&direction_mutex);
    thread::spawn(move || {
        if let Err(e) = wait_move(direction_mutex_clone) {
            println!("Error: {e:?}\r");
        }
    });

    let mut game = Game::new(direction_mutex);
    game.run();
    Ok(())
}

fn wait_move(direction_mutex: Arc<Mutex<Direction>>) -> io::Result<()> {
    while let Ok(event) = read() {
        let Some(event) = event.as_key_press_event() else {
            continue;
        };

        let direction = match event.code {
            KeyCode::Char('w') => Option::from(Direction::Up),
            KeyCode::Char('s') => Option::from(Direction::Down),
            KeyCode::Char('a') => Option::from(Direction::Left),
            KeyCode::Char('d') => Option::from(Direction::Right),
            _ => None
        };

        if let Some(direction) = direction {
            let mut direction_mutex_val = direction_mutex.lock().expect("Error locking mutex");
            if (direction == Direction::Up && *direction_mutex_val == Direction::Down)
                || (direction == Direction::Down && *direction_mutex_val == Direction::Up)
                || (direction == Direction::Left && *direction_mutex_val == Direction::Right)
                || (direction == Direction::Right && *direction_mutex_val == Direction::Left) {
                continue;
            };
            *direction_mutex_val = direction;
        }
    }
    Ok(())
}