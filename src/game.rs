use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use crate::snake::{Direction, Snake};
use crate::draw::{draw_block, draw_rectangle};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0]; //red
const GAME_OVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5]; // bright red (transparent)

const MOVING_TIME: f64 = 0.1; // 0.1 = 10x per second, 0.5 = 2x per second, etc.
const RESTART_TIME: f64 = 1.0; // in secs

const SNAKE_INIT_X: i32 = 2;
// const SNAKE_INIT_Y: i32 = 2;
// const FOOD_INIT_X: i32 = 20;
// const FOOD_INIT_Y: i32 = 4;

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,

    score: u32,
    high_score: u32,
    moving_time: f64,
}
impl Game {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            snake: Snake::new(SNAKE_INIT_X, (height/2)-2),
            waiting_time: 0.0,
            food_exists: true,
            food_x: height/4*3,
            food_y: (height/2)-2,
            width,
            height,
            game_over: false,
            score: 0,
            high_score: 0,
            moving_time: MOVING_TIME,
        }
    }
    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up | Key::W | Key::I => Some(Direction::Up),
            Key::Down | Key::S | Key::K => Some(Direction::Down),
            Key::Left | Key::A | Key::J => Some(Direction::Left),
            Key::Right | Key::D | Key::L => Some(Direction::Right),
            _ => None,
        };

        if dir == None {
            return;
        }

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }
        if dir.unwrap() == self.snake.head_direction() {
            if self.moving_time == MOVING_TIME {
                self.moving_time -= MOVING_TIME/1.5;
            }
        } else {
            self.moving_time = MOVING_TIME;
        }

        self.update_snake(dir);
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        if self.game_over {
            draw_rectangle(GAME_OVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_score();
            self.add_food();
        }

        if self.waiting_time > self.moving_time {
            self.update_snake(None);
        }
    }
    pub fn add_score(&mut self) {
        self.score += 1;
        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }
    pub fn get_score(&self) -> u32 {
        return self.score;
    }
    pub fn get_high_score(&self) -> u32 {
        return self.high_score;
    }
    pub fn reset_score(&mut self) {
        self.score = 0;
    }
    pub fn get_snake_dir(&self) -> Direction {
        return self.snake.head_direction();
    }
    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlaps_tail(next_x, next_y) {
            return false;
        }

        return next_x >= 0 && next_x < self.width && next_y >= 0 && next_y < self.height; // snake hit window border
    }
    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(0..self.width);
        let mut new_y = rng.gen_range(0..self.height);
        while self.snake.overlaps_tail(new_x, new_y) {
            new_x = rng.gen_range(0..self.width);
            new_y = rng.gen_range(0..self.height);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }
    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }
    fn restart(&mut self) {
        self.snake = Snake::new(SNAKE_INIT_X, (self.height/2)-2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = self.height/4*3;
        self.food_y = (self.height/2)-2;
        self.game_over = false;
        self.reset_score();
    }
}