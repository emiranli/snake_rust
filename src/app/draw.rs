use crate::app::{MAP_HEIGHT, MAP_WIDTH};
use crate::app::game::Game;

enum Chars {
    Empty,
    Food,
    Snake,
}

impl Chars {
    fn get_value(&self) -> char {
        match self {
            Chars::Empty => ' ',
            Chars::Food => '*',
            Chars::Snake => 'Х',
        }
    }
}

fn clear_console() {
    print!("{esc}c", esc = 27 as char); // отчитска вывода
}

pub fn lose_message(game: &Game) {
    clear_console();
    println!("Game lost\r\nYou score: {}", game.get_score());
}

pub fn render_map(game: &Game) {
    let mut map = vec![];

    for _x in 0..MAP_HEIGHT {
        let mut line = vec![];
        for _y in 0..MAP_WIDTH {
            line.push(Chars::Empty);
        }
        map.push(line);
    }

    game.get_snake_position().iter().for_each(|cords| {
        map[cords.0 as usize][cords.1 as usize] = Chars::Snake;
    });

    let food = game.get_food_position();
    map[food.0 as usize][food.1 as usize] = Chars::Food;
    
    render(map);
}

fn render(map: Vec<Vec<Chars>>) {
    let mut result = String::new();
    let border_line = "-".repeat(MAP_WIDTH as usize+2) + "\r\n";
    result.push_str(&border_line);

    map.iter().for_each(|line_vec| {
        let mut line = String::from("|");

        line_vec.iter().for_each(|char| {
            line.push(char.get_value());
        });

        result.push_str((line + "|\r\n").as_str());
    });

    result.push_str(&border_line);

    clear_console();
    print!("{}", result);
}

