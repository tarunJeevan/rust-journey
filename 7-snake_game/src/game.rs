use crate::draw::{draw_block, draw_button, draw_screen};
use crate::snake::{Direction, Snake};

use piston_window::CharacterCache;
use piston_window::{Context, G2d, Glyphs, Key, Text, Transformed, types::Color};
use rand::{Rng, rng};

const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0]; // Food's RGB color
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0]; // Border's RGB color
const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5]; // Gameover's RGB color
const PAUSE_COLOR: Color = [0.0, 0.0, 0.0, 0.5]; // Pause screen overlay RGB color
const MENU_COLOR: Color = [0.0, 0.0, 0.0, 1.0]; // Settings screen RGB color
const FONT_DEFAULT_COLOR: Color = [1.0, 1.0, 1.0, 1.0]; // Default font color
const FONT_SELECTED_COLOR: Color = [0.7, 0.0, 0.0, 1.0]; // Selected font color
const BUTTON_SELECTED_COLOR: Color = [0.0, 0.8, 0.5, 1.0]; // Selected button color
const BUTTON_DEFAULT_COLOR: Color = [0.0, 0.0, 0.0, 0.0]; // Unselected button color

const BLOCK_SIZE: f64 = 25.0; // Size of each block in pixels

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

    main_menu_selected: usize, // Index of selected menu item in main menu,
    // NOTE: Add more fields for pause (quit/resume game, go to settings) and settings (difficulty, controls, etc.)
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

            main_menu_selected: 0, // Start with first menu item selected

            waiting_time: 0.0,
        }
    }

    // Handle key press events for every game state
    pub fn key_pressed(&mut self, key: Key) {
        // TODO: Finish implementing key bindings for all game states
        match self.game_state {
            // Handle main menu key presses
            GameState::MainMenu => match key {
                // Navigate through menu options
                Key::Up => {
                    if self.main_menu_selected > 0 {
                        self.main_menu_selected -= 1;
                    }
                }
                Key::Down => {
                    if self.main_menu_selected < 2 {
                        // NOTE: Assuming 3 menu options
                        self.main_menu_selected += 1;
                    }
                }
                Key::Right => {
                    // Alternative to Down key for navigating through options
                    if self.main_menu_selected < 2 {
                        // NOTE: Assuming 3 menu options
                        self.main_menu_selected += 1;
                    }
                }
                Key::Left => {
                    // Alternative to Up key for navigating through options
                    if self.main_menu_selected > 0 {
                        self.main_menu_selected -= 1;
                    }
                }
                Key::Return => {
                    // Select chosen option
                    match self.main_menu_selected {
                        0 => {
                            // Start game
                            self.game_state = GameState::Playing;
                        }
                        1 => {
                            // Go to settings
                            self.game_state = GameState::Settings;
                        }
                        2 => {
                            // Quit game
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            // Handle playing state key presses
            GameState::Playing => {
                // Add key bindings
                let dir = match key {
                    Key::Up => Some(Direction::Up),
                    Key::Down => Some(Direction::Down),
                    Key::Left => Some(Direction::Left),
                    Key::Right => Some(Direction::Right),
                    Key::P => {
                        // End current game
                        self.game_state = GameState::Paused;
                        None
                    }
                    Key::S => {
                        // Go to settings
                        self.game_state = GameState::Settings;
                        None
                    }
                    _ => None,
                };

                // If new direction is opposite of current direction, ignore it
                if dir.unwrap() == self.snake.head_direction().opposite() {
                    return;
                }

                self.update_snake(dir);
            }
            // Handle paused state key presses
            GameState::Paused => match key {
                // Navigate through menu options
                Key::Up => {}
                Key::Down => {}
                Key::Right => {}
                Key::Left => {}
                Key::Return => {} // Select chosen option
                Key::P => {
                    // Resume game
                    self.game_state = GameState::Playing;
                }
                _ => {}
            },
            // Handle game over state key presses
            GameState::GameOver => match key {
                // Navigate through menu options
                Key::Up => {}
                Key::Down => {}
                Key::Right => {}
                Key::Left => {}
                Key::Return => {
                    // Restart game
                    self.restart();
                }
                _ => {}
            },
            // Handle settings state key presses
            GameState::Settings => match key {
                // Navigate through menu options
                Key::Up => {}
                Key::Down => {}
                Key::Right => {}
                Key::Left => {}
                Key::Return => {} // Select chosen option
                _ => {}
            },
        }
    }

    // Draw game board
    pub fn draw_game_board(&self, con: &Context, g: &mut G2d) {
        // Draw the snake
        self.snake.draw(con, g);

        // Draw the food
        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        // Draw the border
        draw_screen(BORDER_COLOR, 0, 0, self.board_width, 1, con, g);
        draw_screen(
            BORDER_COLOR,
            0,
            self.board_height - 1,
            self.board_width,
            1,
            con,
            g,
        );
        draw_screen(BORDER_COLOR, 0, 0, 1, self.board_height, con, g);
        draw_screen(
            BORDER_COLOR,
            self.board_width - 1,
            0,
            1,
            self.board_height,
            con,
            g,
        );
    }

    // Draw pause screen
    pub fn draw_pause(&self, con: &Context, g: &mut G2d) {
        draw_screen(
            PAUSE_COLOR, // Background color
            0,
            0,
            self.board_width,
            self.board_height,
            con,
            g,
        );
        // NOTE: Draw pause text or options here
    }

    // Draw settings screen
    pub fn draw_settings(&self, con: &Context, g: &mut G2d) {
        draw_screen(
            MENU_COLOR, // Background color
            0,
            0,
            self.board_width,
            self.board_height,
            con,
            g,
        );
        // NOTE: Draw settings options, controls, etc.
    }

    // Draw game over screen
    pub fn draw_game_over(&self, con: &Context, g: &mut G2d) {
        draw_screen(
            GAMEOVER_COLOR, // Background color
            0,
            0,
            self.board_width,
            self.board_height,
            con,
            g,
        );
        // NOTE: Draw game over text
    }

    // Draw main menu
    pub fn draw_main_menu(
        &self,
        con: &Context,
        g: &mut G2d,
        title_glyphs: &mut Glyphs,
        text_glyphs: &mut Glyphs,
        button_glyphs: &mut Glyphs,
    ) {
        draw_screen(
            MENU_COLOR, // Background color
            0,
            0,
            self.board_width,
            self.board_height,
            con,
            g,
        );

        // Calculate parameters for drawing title
        let title = "Snake Game";
        let title_font_size = 32;
        let title_width = title_glyphs.width(title_font_size, title).unwrap_or(0.0);

        // Calculate title text position for centering
        let title_x = (self.board_width as f64 * BLOCK_SIZE - title_width) / 2.0;
        let title_y = 80.0; // Position from the top

        // Draw title text
        Text::new_color(FONT_DEFAULT_COLOR, title_font_size)
            .draw(
                title,
                title_glyphs,
                &con.draw_state,
                con.transform.trans(title_x, title_y),
                g,
            )
            .unwrap();

        // Calculate parameters for drawing intro text
        let intro = "Classic Snake Game written in Rust";
        let intro_font_size = 12;
        let intro_width = text_glyphs.width(intro_font_size, intro).unwrap_or(0.0);

        // Calculate intro text position for centering
        let intro_x = (self.board_width as f64 * BLOCK_SIZE - intro_width) / 2.0;
        let intro_y = 100.0; // Position from the top

        // Draw intro text
        Text::new_color(FONT_DEFAULT_COLOR, intro_font_size)
            .draw(
                intro,
                text_glyphs,
                &con.draw_state,
                con.transform.trans(intro_x, intro_y),
                g,
            )
            .unwrap();

        // Draw menu options as buttons
        let menu_options = ["Start Game", "Settings", "Quit"];
        let option_font_size = 16;

        // Calculate button dimensions and positions
        let button_height = 40.0; // Height for each menu button
        let button_width = 150.0; // Width for each menu button
        let start_y = 200.0; // Starting position for menu buttons

        // Loop through menu options and draw them as buttons
        for (i, option) in menu_options.iter().enumerate() {
            // Calculate option width dynamically
            let button_x = (self.board_width as f64 * BLOCK_SIZE - button_width) / 2.0;
            let button_y = start_y + i as f64 * (button_height + 10.0);

            // Highlight selected option.
            let button_color = if i == self.main_menu_selected {
                BUTTON_SELECTED_COLOR // Highlight color for selected button
            } else {
                BUTTON_DEFAULT_COLOR // Default color for unselected buttons
            };

            // Draw option background rectangle to represent button
            draw_button(
                button_color,
                button_x,
                button_y,
                button_width,
                button_height,
                con,
                g,
            );

            // Center button text
            let option_width = button_glyphs.width(option_font_size, option).unwrap_or(0.0);
            let option_x = button_x + (button_width - option_width) / 2.0;
            let option_y = button_y + button_height / 2.0 + option_font_size as f64 / 2.5;

            let option_color = if i == self.main_menu_selected {
                FONT_SELECTED_COLOR // Highlight color for selected option
            } else {
                FONT_DEFAULT_COLOR // Default font color
            };

            // Draw each menu option
            Text::new_color(option_color, option_font_size)
                .draw(
                    option,
                    button_glyphs,
                    &con.draw_state,
                    con.transform.trans(option_x, option_y),
                    g,
                )
                .unwrap();
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
