use crate::game::{BLOCK_SIZE, SnakeTextures};

use piston_window::{Context, G2d, Image, Transformed};
use std::collections::LinkedList;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
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

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum BodyOrientation {
    Horizontal,
    Vertical,
    TurnUL,
    TurnUR,
    TurnBL,
    TurnBR,
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
    pub fn draw(&self, con: &Context, g: &mut G2d, textures: &SnakeTextures) {
        let mut iter = self.body.iter();
        let len = self.body.len();

        // Head section
        if let Some(head) = iter.next() {
            let head_dir = self.direction;
            Image::new().draw(
                &textures.head[&head_dir],
                &con.draw_state,
                con.transform
                    .trans((head.x as f64) * BLOCK_SIZE, (head.y as f64) * BLOCK_SIZE),
                g,
            );
        }
        // Body and tail sections
        let body_vec: Vec<&Block> = self.body.iter().collect();
        for i in 1..len {
            let curr = body_vec[i];

            // Tail
            if i == len - 1 {
                let prev = body_vec[i - 1];
                let tail_dir = match (prev.x - curr.x, prev.y - curr.y) {
                    (1, 0) => Direction::Right,
                    (-1, 0) => Direction::Left,
                    (0, 1) => Direction::Down, // Vertical is flipped as y-axis goes down
                    (0, -1) => Direction::Up,  // Vertical is flipped as y-axis goes down
                    _ => self.direction,       // Fallback
                };
                Image::new().draw(
                    &textures.tail[&tail_dir],
                    &con.draw_state,
                    con.transform
                        .trans((curr.x as f64) * BLOCK_SIZE, (curr.y as f64) * BLOCK_SIZE),
                    g,
                );
            }
            // Body
            else {
                let prev = body_vec[i - 1];
                let next = body_vec[i + 1];

                let orientation = if prev.x == next.x {
                    BodyOrientation::Vertical
                } else if prev.y == next.y {
                    BodyOrientation::Horizontal
                } else {
                    // Calculate turn direction
                    if (prev.x < curr.x && next.y < curr.y) || (next.x < curr.x && prev.y < curr.y)
                    {
                        BodyOrientation::TurnUL
                    } else if (prev.x > curr.x && next.y < curr.y)
                        || (next.x > curr.x && prev.y < curr.y)
                    {
                        BodyOrientation::TurnUR
                    } else if (prev.x < curr.x && next.y > curr.y)
                        || (next.x < curr.x && prev.y > curr.y)
                    {
                        BodyOrientation::TurnBL
                    } else {
                        BodyOrientation::TurnBR
                    }
                };
                Image::new().draw(
                    &textures.body[&orientation],
                    &con.draw_state,
                    con.transform
                        .trans((curr.x as f64) * BLOCK_SIZE, (curr.y as f64) * BLOCK_SIZE),
                    g,
                );
            }
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

    pub fn move_forward_to(&mut self, pos: (i32, i32), dir: Option<Direction>) {
        if let Some(d) = dir {
            self.direction = d;
        }

        let new_block = Block { x: pos.0, y: pos.1 };

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

    // Extend snake's tail after eating food
    pub fn extend_tail(&mut self) {
        let tail = self.tail.clone().unwrap();
        self.body.push_back(tail);
    }

    // Check if snake's head overlaps with its body excluding the tail
    pub fn overlap_body(&self, x: i32, y: i32) -> bool {
        for block in self.body.iter().take(self.body.len().saturating_sub(1)) {
            if x == block.x && y == block.y {
                return true;
            }
        }
        false
    }
}
