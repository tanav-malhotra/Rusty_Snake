use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;
use crate::draw;
use draw::draw_block;

// const SNAKE_COLOR: Color = [0.368627, 0.839216, 0.337255, 1.0]; // green
// const SNAKE_HEAD_COLOR: Color = [0.356863, 0.949020, 0.313725, 1.0]; // dark green
const SNAKE_COLOR: Color = [0.258824, 0.529412, 0.960784, 1.0]; // blue
const SNAKE_HEAD_COLOR: Color = [0.254902, 0.266667, 0.949020, 1.0]; // dark blue
const SNAKE_START_LENGTH: i32 = 4;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Waiting,//TODO: remove this if waiting_time works instead
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        return match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right | Direction::Waiting => Direction::Left,
        }
    }
}

#[derive(Clone)]
struct Block {
    x: i32,
    y: i32,
}
impl Block {
    fn new(x: i32, y: i32) -> Self {
        return Self{x, y};
    }
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Self {
        let mut body = LinkedList::new();
        for i in 0..SNAKE_START_LENGTH {
            body.push_front(Block::new(x+i, y));
        }
        return Snake{
            direction: Direction::Waiting,
            body,
            tail: None,
        };
    }
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        let head_block = self.body.front().unwrap();
        for block in &self.body {
            if block.x == head_block.x && block.y == head_block.y {
                draw_block(SNAKE_HEAD_COLOR, block.x, block.y, con, g);
            } else {
                draw_block(SNAKE_COLOR, block.x, block.y, con, g);
            }
        }
    }
    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        return (head_block.x, head_block.y);
    }
    pub fn head_direction(&self) -> Direction {
        return self.direction;
    }
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block::new(last_x, last_y-1),
            Direction::Down => Block::new(last_x, last_y+1),
            Direction::Left => Block::new(last_x-1, last_y),
            Direction::Right => Block::new(last_x+1, last_y),
            Direction::Waiting => return (),
        };
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) = self.head_position();
        let mut moving_dir = self.direction;

        match dir {
            Some(d) => moving_dir = d,
            None => {},
        }

        return match moving_dir {
            Direction::Up => (head_x, head_y -1),
            Direction::Down => (head_x, head_y +1),
            Direction::Left => (head_x -1, head_y),
            Direction::Right | Direction::Waiting => (head_x +1, head_y),
        };
    }
    pub fn restore_tail(&mut self) {
        let tail_block = self.tail.clone().unwrap();
        self.body.push_back(tail_block);
    }
    pub fn overlaps_tail(&self, x: i32, y: i32) -> bool {
        let mut c = 0;
        for block in &self.body {
            if block.x == x && block.y == y {
                return true;
            }

            c += 1;
            if c == self.body.len() -1 {
                break;
            }
        }
        return false;
    }
}