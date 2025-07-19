use std::sync::{Arc, Mutex};
use std::{thread, time};
use rand::random_range;

const MAP_WIDTH: i32 = 20;
const MAP_HEIGHT: i32 = 17;
const SPEED: u64 = 300;
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
fn clear_console() {
    print!("{esc}c", esc = 27 as char); // отчитска вывода
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Cords (i32, i32);

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Map {
    data: Vec<Vec<Chars>>,
}

impl Map {
    pub fn new() -> Self {
        let mut data = vec![];

        for _x in 0..MAP_HEIGHT {
            let mut line = vec![];
            for _y in 0..MAP_WIDTH {
                line.push(Chars::Empty);
            }
            data.push(line);
        }

        Map {
            data
        }
    }

    pub fn render(&self) {
        let mut result = String::new();
        let border_line = "-".repeat(MAP_WIDTH as usize+2) + "\r\n";
        result.push_str(&border_line);

        self.data.iter().for_each(|line_vec| {
            let mut line = String::from("|");

            line_vec.iter().for_each(|char| {
                line.push(char.get_value());
            });

            result.push_str((line + "|\r\n").as_str());
        });

        result.push_str(&border_line);

        print!("{}", result);
    }
}

#[derive(Debug)]
pub struct Game {
    direction_mutex: Arc<Mutex<Direction>>,
    food: Cords,
    score: i32,
    snake: Vec<Cords>,
}

impl Game {
    pub fn new(direction_mutex: Arc<Mutex<Direction>>) -> Self {
        let mut snake = vec![];

        for i in 0..4 {
            snake.push(Cords(i, 1));
        }
        snake.reverse();

        let mut game = Game {
            direction_mutex,
            food: Cords(0, 0),
            score: 0,
            snake
        };

        game.food = game.next_food_position();
        game
    }

    pub fn run(&mut self) {
        loop {
            if self.tick().is_err() {
                self.lose();
                break;
            }
            thread::sleep(time::Duration::from_millis(SPEED));
        }
    }

    fn lose(&mut self) {
        clear_console();
        println!("Game lost\r\nYou score: {}", self.score);
    }

    fn tick(&mut self) -> Result<(), &str> {
        self.move_snake()?;

        let mut map = Map::new();
        self.snake.iter().for_each(|cords| {
            map.data[cords.0 as usize][cords.1 as usize] = Chars::Snake;
        });
        map.data[self.food.0 as usize][self.food.1 as usize] = Chars::Food;

        clear_console();
        map.render();

        Ok(())
    }

    fn next_food_position(&mut self) -> Cords {
        let mut x = random_range(0..MAP_HEIGHT);
        let mut y = random_range(0..MAP_WIDTH);
        while self.snake.contains(&Cords(x, y)) {
            x = random_range(0..MAP_HEIGHT);
            y = random_range(0..MAP_WIDTH);
        }
        Cords(x, y)
    }

    fn move_snake(&mut self) -> Result<(), &'static str> {
        let head = self.snake.first().expect("Error getting head");

        let (ox, oy) =  self.direction_mutex.lock().expect("Error locking mutex").get_offset();

        let new_head = Cords(
            head.0 + ox,
            head.1 + oy,
        );

        if new_head.0 < 0 || new_head.0 == MAP_HEIGHT
            || new_head.1 < 0 || new_head.1 == MAP_WIDTH
            || self.snake.contains(&new_head) {
            return Err("Error moving snake");
        }

        self.snake.insert(0, new_head);
        if self.snake[0] == self.food {
            self.food = self.next_food_position();
            self.score += 1;
        } else {
            self.snake.pop();
        }

        Ok(())
    }
}