use crate::draw::draw_block;

use piston_window::{Context, G2d, types::Color};
use std::collections::LinkedList;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0]; // Snake's RGB color

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Clone)]
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();

        // Creates default snake with length of 3 blocks
        body.push_back(Block { x: x + 2, y });
        body.push_back(Block { x: x + 1, y });
        body.push_back(Block { x, y });

        // Default snake is 3 blocks long horizontally and moving to the right
        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    // Draw snake on screen
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, con, g);
        }
    }

    // Return head of snake as tuple
    pub fn head_position(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    // Move snake
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        if let Some(d) = dir {
            self.direction = d;
        }

        let (last_x, last_y): (i32, i32) = self.head_position();

        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };

        self.body.push_front(new_block);
        self.tail = self.body.pop_back();
    }

    // Return a clone of the snake's direction
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    // Helper method to find snake's head's next position
    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;

        if let Some(d) = dir {
            moving_dir = d;
        }

        match moving_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    // Restore snake's tail after eating food
    pub fn restore_tail(&mut self) {
        let tail = self.tail.clone().unwrap();
        self.body.push_back(tail);
    }

    // Check if snake's head overlaps with its body
    pub fn overlap_body(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }
            // Avoid fail state when head overlaps with tail momentarily before tail moves
            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }
        false
    }
}
