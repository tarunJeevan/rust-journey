use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

use piston_window::{types::Color, *};
use rand::{Rng, rng};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0]; // Food's RGB color
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0]; // Border's RGB color
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5]; // Gameover's RGB color
const PAUSE_COLOR: Color = [0.0, 0.0, 0.0, 0.5]; // Pause screen overlay RGB color

const MOVING_PERIOD: f64 = 0.1; // Snake's FPS. Current speed is 10 FPS
const RESTART_TIME: f64 = 1.0; // Time to restart game after gameover

#[derive(Clone, Debug)]
pub enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
    Settings,
}

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    board_width: i32,
    board_height: i32,

    game_state: GameState,

    waiting_time: f64,
}

impl Game {
    pub fn new(board_width: i32, board_height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),

            food_exists: true,
            food_x: 6, // NOTE: Randomize?
            food_y: 4, // NOTE: Randomize?

            board_width,
            board_height,

            game_state: GameState::MainMenu,

            waiting_time: 0.0,
        }
    }

    // Run when arrow keys are pressed
    pub fn key_pressed(&mut self, key: Key) {
        // TODO: Change to work with other game states
        if let GameState::GameOver = self.game_state {
            return;
        }

        // Add key bindings
        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            Key::Escape => {
                // End current game
                self.game_state = match self.game_state {
                    // Ignore if already in main menu
                    GameState::MainMenu => GameState::MainMenu,
                    _ => GameState::GameOver,
                };
                None
            }
            _ => None,
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    // Draw game board
    pub fn draw(&self, con: &Context, g: &mut G2d) {
        // Draw the snake
        self.snake.draw(con, g);

        // Draw the food
        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        // Draw the border
        draw_rectangle(BORDER_COLOR, 0, 0, self.board_width, 1, con, g);
        draw_rectangle(
            BORDER_COLOR,
            0,
            self.board_height - 1,
            self.board_width,
            1,
            con,
            g,
        );
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.board_height, con, g);
        draw_rectangle(
            BORDER_COLOR,
            self.board_width - 1,
            0,
            1,
            self.board_height,
            con,
            g,
        );

        // Draw pause screen overlay over game board
        if let GameState::Paused = self.game_state {
            draw_rectangle(
                PAUSE_COLOR,
                0,
                0,
                self.board_width,
                self.board_height,
                con,
                g,
            );
        }

        // Draw gameover screen
        if let GameState::GameOver = self.game_state {
            draw_rectangle(
                GAMEOVER_COLOR,
                0,
                0,
                self.board_width,
                self.board_height,
                con,
                g,
            );
        }
    }

    // Update game state over time
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if let GameState::GameOver = self.game_state {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    // Get current game state
    pub fn get_game_state(&self) -> GameState {
        self.game_state.clone()
    }

    // Change game state
    pub fn change_game_state(&mut self, new_state: GameState) {
        self.game_state = new_state;
    }

    // Check if snake has eaten food
    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        // Grow snake if food is eaten
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    // Check snake collision. TODO: Fix to use GameState
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y): (i32, i32) = self.snake.next_head(dir);

        // Check if head overlaps body
        if self.snake.overlap_body(next_x, next_y) {
            return false;
        }

        // Check if out of bounds
        next_x > 0 && next_y > 0 && next_x < self.board_width - 1 && next_y < self.board_height - 1
    }

    // Spawn new food on game board
    fn add_food(&mut self) {
        // Initialize randomizer
        let mut rng = rng();

        // Generate random position for food
        let mut new_x = rng.random_range(1..self.board_width - 1);
        let mut new_y = rng.random_range(1..self.board_height - 1);

        // If food is eaten, generate new random position for food
        while self.snake.overlap_body(new_x, new_y) {
            new_x = rng.random_range(1..self.board_width - 1);
            new_y = rng.random_range(1..self.board_height - 1);
        }

        // Update game state
        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    // Update snake's state
    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_state = GameState::GameOver;
        }
        self.waiting_time = 0.0;
    }

    // Reset game state
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_state = GameState::Playing;
    }
}
