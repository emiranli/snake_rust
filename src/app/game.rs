use std::{fmt, io};
use rand::random_range;
use crate::app::{Cords, Direction, MAP_HEIGHT, MAP_WIDTH};

#[derive(Debug)]
pub enum GameError {
    CollisionWithWall,
    CollisionWithSelf,
    UnknownError,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameError::CollisionWithWall => write!(f, "Змейка врезалась в стену!"),
            GameError::CollisionWithSelf => write!(f, "Змейка врезалась в себя!"),
            GameError::UnknownError => write!(f, "Непредвиденная ошибка!"),
        }
    }
}

impl std::error::Error for GameError {}

impl From<GameError> for io::Error {
    fn from(error: GameError) -> Self {
        io::Error::new(io::ErrorKind::Other, error.to_string())
    }
}


pub struct Game {
    pub direction: Direction,
    food: Cords,
    score: i32,
    snake: Vec<Cords>,
}

impl Game {
    pub fn new() -> Self {
        let mut snake = vec![];

        for i in 0..4 {
            snake.push(Cords(i, 1));
        }
        snake.reverse();

        let mut game = Game {
            direction: Direction::Down,
            food: Cords(0, 0),
            score: 0,
            snake
        };

        game.food = game.next_food_position();
        game
    }

    pub fn get_snake_position(&self) -> &Vec<Cords> {
        &self.snake
    }
    
    pub fn get_food_position(&self) -> Cords {
        self.food
    }
    
    pub fn get_score(&self) -> i32 {
        self.score
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

    pub fn move_snake(&mut self) -> Result<(), io::Error> {
        let head = match self.snake.first() {
            Some(head) => head,
            None => return Err(io::Error::from(GameError::UnknownError)),
        };

        let (ox, oy) =  self.direction.get_offset();

        let new_head = Cords(
            head.0 + ox,
            head.1 + oy,
        );

        if !new_head.is_within_bounds()  {
            return Err(io::Error::from(GameError::CollisionWithWall));

        }
        if self.snake.contains(&new_head)  {
            return Err(io::Error::from(GameError::CollisionWithSelf));
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