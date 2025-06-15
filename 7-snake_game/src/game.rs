use std::collections::HashMap;

use crate::draw::{draw_block, draw_button, draw_screen};
use crate::snake::{BodyOrientation, Direction, Snake};

use piston_window::{CharacterCache, G2dTexture};
use piston_window::{Context, G2d, Glyphs, Key, Text, Transformed, types::Color};
use rand::{Rng, rng};

const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0]; // Food's RGB color
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0]; // Border's RGB color
const GAME_BOARD_COLOR: Color = [0.5, 0.5, 0.5, 1.0]; // Game board background color
const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5]; // Gameover's RGB color
const PAUSE_COLOR: Color = [0.0, 0.0, 0.0, 0.5]; // Pause screen overlay RGB color
const MENU_COLOR: Color = [0.0, 0.0, 0.0, 1.0]; // Settings screen RGB color
const FONT_DEFAULT_COLOR: Color = [1.0, 1.0, 1.0, 1.0]; // Default font color
const MAIN_MENU_FONT_SELECTED_COLOR: Color = [0.7, 0.0, 0.0, 1.0]; // Selected font color for main menu
const GAMEOVER_FONT_SELECTED_COLOR: Color = [0.0, 0.0, 0.0, 1.0]; // Selected font color in game over screen
const BUTTON_SELECTED_COLOR: Color = [0.0, 0.8, 0.5, 1.0]; // Selected button color
const BUTTON_DEFAULT_COLOR: Color = [0.0, 0.0, 0.0, 0.0]; // Unselected button color

const BLOCK_SIZE: f64 = 25.0; // Size of each block in pixels

const SLOW_SNAKE_SPEED: f64 = 0.2; // Slow snake's FPS. Current speed is 5 FPS
const NORMAL_SNAKE_SPEED: f64 = 0.1; // Snake's FPS. Current speed is 10 FPS
const FAST_SNAKE_SPEED: f64 = 0.05; // Fast snake's FPS. Current speed is 20 FPS

// Snake textures
pub struct SnakeTextures {
    pub head: HashMap<Direction, G2dTexture>,
    pub body: HashMap<BodyOrientation, G2dTexture>,
    pub tail: HashMap<Direction, G2dTexture>,
}

#[derive(Clone, Debug)]
pub enum GameState {
    MainMenu,
    Playing,
    Paused,
    GameOver,
    Settings,
    KeyBinding,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnakeSpeed {
    Slow,
    Normal,
    Fast,
}

impl SnakeSpeed {
    // Convert SnakeSpeed to f64 for game speed
    pub fn as_f64(&self) -> f64 {
        match self {
            SnakeSpeed::Slow => SLOW_SNAKE_SPEED,
            SnakeSpeed::Normal => NORMAL_SNAKE_SPEED,
            SnakeSpeed::Fast => FAST_SNAKE_SPEED,
        }
    }

    // Go to the next speed in the cycle
    pub fn next(&self) -> SnakeSpeed {
        match self {
            SnakeSpeed::Slow => SnakeSpeed::Normal,
            SnakeSpeed::Normal => SnakeSpeed::Fast,
            SnakeSpeed::Fast => SnakeSpeed::Slow, // Loop back to slow
        }
    }

    // Get string representation of the speed
    pub fn as_str(&self) -> &'static str {
        match self {
            SnakeSpeed::Slow => "Slow",
            SnakeSpeed::Normal => "Normal",
            SnakeSpeed::Fast => "Fast",
        }
    }
}

#[derive(Clone)]
pub struct GameSettings {
    pub key_bindings: KeyBindings, // e.g., "Arrow keys", "WASD".
    pub wall_wrapping: bool,       // Whether the snake can wrap around walls
    pub snake_speed: SnakeSpeed,   // e.g., 0.1 for 10 FPS
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            wall_wrapping: false,
            snake_speed: SnakeSpeed::Normal,
            key_bindings: KeyBindings::default(),
        }
    }
}

#[derive(Clone)]
pub struct KeyBindings {
    pub up: Key,
    pub down: Key,
    pub left: Key,
    pub right: Key,
    pub pause: Key,
}

impl Default for KeyBindings {
    fn default() -> Self {
        KeyBindings {
            up: Key::Up,
            down: Key::Down,
            left: Key::Left,
            right: Key::Right,
            pause: Key::P,
        }
    }
}

pub struct Game {
    snake: Snake,
    waiting_time: f64,
    score: i32,

    snake_textures: SnakeTextures,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    board_width: i32,
    board_height: i32,

    game_state: GameState,
    game_settings: GameSettings,
    waiting_for_key: Option<&'static str>, // Used to listen for key input

    main_menu_selected: usize,   // Index of selected menu item in main menu
    game_over_selected: usize,   // Index of selected menu item in game over screen
    pause_selected: usize,       // Index of selected menu item in pause screen
    settings_selected: usize,    // Index of selected menu item in settings screen
    key_binding_selected: usize, // Index of selected key binding in key binding screen
}

impl Game {
    pub fn new(board_width: i32, board_height: i32, snake_textures: SnakeTextures) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            score: 0,

            snake_textures,

            food_exists: true,
            food_x: 6, // NOTE: Randomize?
            food_y: 4, // NOTE: Randomize?

            board_width,
            board_height,

            game_state: GameState::MainMenu, // Start with main menu
            game_settings: GameSettings::default(),
            waiting_for_key: None, // No key is being waited for

            main_menu_selected: 0, // Start with first option selected in main menu
            game_over_selected: 0, // Start with first option selected in game over screen
            pause_selected: 0,     // Start with first option selected in pause screen
            settings_selected: 0,  // Start with first option selected in settings screen
            key_binding_selected: 0, // Start with first key binding selected in key binding screen
        }
    }

    // Handle key press events for every game state
    pub fn key_pressed(&mut self, key: Key) {
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
                            // Start new game
                            self.restart();
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
            GameState::Playing => match key {
                k if k == self.game_settings.key_bindings.up => {
                    // Move snake up unless it is already moving down
                    if self.snake.head_direction().opposite() != Direction::Up {
                        self.update_snake(Some(Direction::Up));
                    }
                }
                k if k == self.game_settings.key_bindings.down => {
                    // Move snake down unless it is already moving up
                    if self.snake.head_direction().opposite() != Direction::Down {
                        self.update_snake(Some(Direction::Down));
                    }
                }
                k if k == self.game_settings.key_bindings.left => {
                    // Move snake left unless it is already moving right
                    if self.snake.head_direction().opposite() != Direction::Left {
                        self.update_snake(Some(Direction::Left));
                    }
                }
                k if k == self.game_settings.key_bindings.right => {
                    // Move snake right unless it is already moving left
                    if self.snake.head_direction().opposite() != Direction::Right {
                        self.update_snake(Some(Direction::Right));
                    }
                }
                k if k == self.game_settings.key_bindings.pause => {
                    // Pause current game
                    self.game_state = GameState::Paused;
                }
                _ => {}
            },
            // Handle paused state key presses
            GameState::Paused => match key {
                // Navigate through menu options
                Key::Up => {
                    if self.pause_selected > 0 {
                        self.pause_selected -= 1;
                    }
                }
                Key::Down => {
                    if self.pause_selected < 1 {
                        // NOTE: Assuming 2 menu options
                        self.pause_selected += 1;
                    }
                }
                Key::Right => {
                    // Alternative to Down key for navigating through options
                    if self.pause_selected < 1 {
                        // NOTE: Assuming 2 menu options
                        self.pause_selected += 1;
                    }
                }
                Key::Left => {
                    // Alternative to Up key for navigating through options
                    if self.pause_selected > 0 {
                        self.pause_selected -= 1;
                    }
                }
                Key::Return => {
                    // Select chosen option
                    match self.pause_selected {
                        0 => {
                            // Go to main menu
                            self.game_state = GameState::MainMenu;
                        }
                        1 => {
                            // Quit game
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
                Key::P => {
                    // Resume game
                    self.game_state = GameState::Playing;
                }
                _ => {}
            },
            // Handle game over state key presses
            GameState::GameOver => match key {
                // Navigate through menu options
                Key::Up => {
                    if self.game_over_selected > 0 {
                        self.game_over_selected -= 1;
                    }
                }
                Key::Down => {
                    if self.game_over_selected < 3 {
                        // NOTE: Assuming 4 menu options
                        self.game_over_selected += 1;
                    }
                }
                Key::Right => {
                    // Alternative to Down key for navigating through options
                    if self.game_over_selected < 3 {
                        // NOTE: Assuming 4 menu options
                        self.game_over_selected += 1;
                    }
                }
                Key::Left => {
                    // Alternative to Up key for navigating through options
                    if self.game_over_selected > 0 {
                        self.game_over_selected -= 1;
                    }
                }
                Key::Return => {
                    // Select chosen option
                    match self.game_over_selected {
                        0 => {
                            // Restart game
                            self.restart();
                        }
                        1 => {
                            // Return to main menu
                            self.game_state = GameState::MainMenu;
                        }
                        2 => {
                            // Go to settings
                            self.game_state = GameState::Settings;
                        }
                        3 => {
                            // Quit game
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            // Handle settings state key presses
            GameState::Settings => match key {
                // Navigate through menu options
                Key::Up => {
                    if self.settings_selected > 0 {
                        self.settings_selected -= 1;
                    }
                }
                Key::Down => {
                    if self.settings_selected < 2 {
                        // NOTE: Assuming 3 menu options
                        self.settings_selected += 1;
                    }
                }
                Key::Right => {
                    if self.settings_selected < 2 {
                        // NOTE: Assuming 3 menu options
                        self.settings_selected += 1;
                    }
                }
                Key::Left => {
                    if self.settings_selected > 0 {
                        self.settings_selected -= 1;
                    }
                }
                Key::Return => {
                    // Select chosen option
                    match self.settings_selected {
                        0 => {
                            // Cycle through snake speeds
                            self.game_settings.snake_speed = self.game_settings.snake_speed.next();
                        }
                        1 => {
                            // Toggle wall wrapping
                            self.game_settings.wall_wrapping = !self.game_settings.wall_wrapping;
                        }
                        2 => {
                            // Activate overlay for setting key bindings
                            self.game_state = GameState::KeyBinding;
                        }
                        _ => {}
                    }
                }
                Key::Backspace => {
                    // Return to main menu
                    self.game_state = GameState::MainMenu;
                }
                _ => {}
            },
            // Handle key binding state key presses
            GameState::KeyBinding => match key {
                // Navigate through key binding options
                Key::Up => {
                    if self.key_binding_selected > 0 {
                        self.key_binding_selected -= 1;
                    }
                }
                Key::Down => {
                    if self.key_binding_selected < 4 {
                        // NOTE: Assuming 5 menu options
                        self.key_binding_selected += 1;
                    }
                }
                Key::Right => {
                    if self.key_binding_selected < 4 {
                        // NOTE: Assuming 5 menu options
                        self.key_binding_selected += 1;
                    }
                }
                Key::Left => {
                    if self.key_binding_selected > 0 {
                        self.key_binding_selected -= 1;
                    }
                }
                Key::Return => {
                    // Confirm key to be rebound
                    self.waiting_for_key = Some(match self.key_binding_selected {
                        0 => "up",
                        1 => "down",
                        2 => "left",
                        3 => "right",
                        4 => "pause",
                        _ => unreachable!(), // Should never happen
                    });
                }
                Key::Backspace => {
                    // Return to settings menu
                    self.game_state = GameState::Settings;
                    self.waiting_for_key = None;
                }
                _ => {
                    // If waiting for a key, update the corresponding key binding
                    if let Some(binding) = self.waiting_for_key {
                        match binding {
                            "up" => self.game_settings.key_bindings.up = key,
                            "down" => self.game_settings.key_bindings.down = key,
                            "left" => self.game_settings.key_bindings.left = key,
                            "right" => self.game_settings.key_bindings.right = key,
                            "pause" => self.game_settings.key_bindings.pause = key,
                            _ => {}
                        }
                        // Clear waiting state after rebinding
                        self.waiting_for_key = None;
                    }
                }
            },
        }
    }

    // Draw game board
    pub fn draw_game_board(&self, con: &Context, g: &mut G2d, title_glyphs: &mut Glyphs) {
        // Draw background
        draw_screen(
            GAME_BOARD_COLOR, // Background color
            0,
            0,
            self.board_width,
            self.board_height,
            con,
            g,
        );
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

        // Calculate parameters for drawing score
        let score_text = format!("Score: {}", self.score);
        let score_font_size = 12;
        let score_width = title_glyphs
            .width(score_font_size, &score_text)
            .unwrap_or(0.0);

        // Calculate score text position for centering
        let score_x = (self.board_width as f64 * BLOCK_SIZE - score_width) / 2.0;
        let score_y = 40.0; // Position from the top

        // Draw score text
        Text::new_color(FONT_DEFAULT_COLOR, score_font_size)
            .draw(
                &score_text,
                title_glyphs,
                &con.draw_state,
                con.transform.trans(score_x, score_y),
                g,
            )
            .unwrap();
    }

    // Draw pause screen
    pub fn draw_pause(
        &self,
        con: &Context,
        g: &mut G2d,
        title_glyphs: &mut Glyphs,
        text_glyphs: &mut Glyphs,
        button_glyphs: &mut Glyphs,
    ) {
        // Draw game board
        self.draw_game_board(con, g, title_glyphs);
        // Draw pause overlay
        draw_screen(
            PAUSE_COLOR, // Background color
            0,
            0,
            self.board_width,
            self.board_height,
            con,
            g,
        );

        // Calculate parameters for drawing title
        let title = "Paused";
        let title_font_size = 48;
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
        let intro = "Press 'P' to resume game";
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
        let menu_options = ["Main Menu", "Quit"];
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
            let button_color = if i == self.pause_selected {
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

            let option_color = if i == self.pause_selected {
                MAIN_MENU_FONT_SELECTED_COLOR // Highlight color for selected option
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

    // Draw settings screen
    pub fn draw_settings(
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
        let title = "Settings";
        let title_font_size = 48;
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

        // Calculate parameters for drawing instructions text
        let instructions = "Press 'Backspace' to return to Main Menu";
        let instructions_font_size = 12;
        let instructions_width = text_glyphs
            .width(instructions_font_size, instructions)
            .unwrap_or(0.0);

        // Calculate instructions text position for centering
        let instructions_x = (self.board_width as f64 * BLOCK_SIZE - instructions_width) / 2.0;
        let instructions_y = 110.0; // Position from the top

        // Draw instructions text
        Text::new_color(FONT_DEFAULT_COLOR, instructions_font_size)
            .draw(
                instructions,
                text_glyphs,
                &con.draw_state,
                con.transform.trans(instructions_x, instructions_y),
                g,
            )
            .unwrap();

        // Draw menu options as description and button
        let menu_options = [
            (
                "Snake Speed: ",
                self.game_settings.snake_speed.as_str().to_string(),
            ),
            (
                "Wall Wrapping: ",
                if self.game_settings.wall_wrapping {
                    "Enabled"
                } else {
                    "Disabled"
                }
                .to_string(),
            ),
            ("Key Bindings: ", "Customize".to_string()),
        ];
        let option_font_size = 16;
        let row_height = 40.0;
        let start_y = 180.0;
        let left_margin = 60.0;
        let right_margin = self.board_width as f64 * BLOCK_SIZE - 220.0;

        // Loop through menu options and draw descriptions and buttons
        for (i, (desc, value)) in menu_options.iter().enumerate() {
            let y = start_y + i as f64 * (row_height + 10.0);

            // Draw description text
            Text::new_color(FONT_DEFAULT_COLOR, option_font_size)
                .draw(
                    desc,
                    text_glyphs,
                    &con.draw_state,
                    con.transform.trans(left_margin, y),
                    g,
                )
                .unwrap();

            // Highlight selected option.
            let button_color = if i == self.settings_selected {
                BUTTON_SELECTED_COLOR // Highlight color for selected button
            } else {
                BUTTON_DEFAULT_COLOR // Default color for unselected buttons
            };

            // Draw button
            draw_button(button_color, right_margin, y - 25.0, 150.0, 40.0, con, g);

            let option_color = if i == self.settings_selected {
                MAIN_MENU_FONT_SELECTED_COLOR // Highlight color for selected option
            } else {
                FONT_DEFAULT_COLOR // Default font color
            };

            // Draw value text
            Text::new_color(option_color, option_font_size)
                .draw(
                    value,
                    button_glyphs,
                    &con.draw_state,
                    con.transform.trans(right_margin + 20.0, y),
                    g,
                )
                .unwrap();
        }
    }

    // Draw game over screen
    pub fn draw_game_over(
        &self,
        con: &Context,
        g: &mut G2d,
        title_glyphs: &mut Glyphs,
        button_glyphs: &mut Glyphs,
    ) {
        draw_screen(
            GAMEOVER_COLOR, // Background color
            0,
            0,
            self.board_width,
            self.board_height,
            con,
            g,
        );
        // Calculate parameters for drawing title
        let title = "Game Over";
        let title_font_size = 48;
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

        // Draw menu options as buttons
        let menu_options = ["Restart", "Main Menu", "Settings", "Quit"];
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
            let button_color = if i == self.game_over_selected {
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

            let option_color = if i == self.game_over_selected {
                GAMEOVER_FONT_SELECTED_COLOR // Highlight color for selected option
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

    // Draw key bindings overlay
    pub fn draw_key_bindings(
        &self,
        con: &Context,
        g: &mut G2d,
        title_glyphs: &mut Glyphs,
        text_glyphs: &mut Glyphs,
        button_glyphs: &mut Glyphs,
    ) {
        // Draw key bindings overlay
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
        let title = "Rebind Controls";
        let title_font_size = 36;
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
        let instructions = [
            "Select a key to rebind",
            "Press 'Enter'",
            "Press new key.",
            "Press 'Backspace' to return to Settings",
        ];
        let instructions_font_size = 12;

        for (i, instruction) in instructions.iter().enumerate() {
            let instruction_width = text_glyphs
                .width(instructions_font_size, instruction)
                .unwrap_or(0.0);

            // Calculate intro text position for centering
            let intro_x = (self.board_width as f64 * BLOCK_SIZE - instruction_width) / 2.0;
            let intro_y = 120.0 + i as f64 * 20.0; // Position from the top

            // Draw instruction text
            Text::new_color(FONT_DEFAULT_COLOR, instructions_font_size)
                .draw(
                    instruction,
                    text_glyphs,
                    &con.draw_state,
                    con.transform.trans(intro_x, intro_y),
                    g,
                )
                .unwrap();
        }

        // Draw menu options as buttons
        let menu_options = [
            (
                "Up Key: ",
                format!("{:.?}", self.game_settings.key_bindings.up),
            ),
            (
                "Down Key: ",
                format!("{:.?}", self.game_settings.key_bindings.down),
            ),
            (
                "Left Key: ",
                format!("{:.?}", self.game_settings.key_bindings.left),
            ),
            (
                "Right Key: ",
                format!("{:.?}", self.game_settings.key_bindings.right),
            ),
            (
                "Pause Key: ",
                format!("{:.?}", self.game_settings.key_bindings.pause),
            ),
        ];
        let option_font_size = 16;
        let row_height = 40.0;
        let start_y = 240.0;
        let left_margin = 60.0;
        let right_margin = self.board_width as f64 * BLOCK_SIZE - 220.0;

        // Loop through menu options and draw descriptions and buttons
        for (i, (desc, value)) in menu_options.iter().enumerate() {
            let y = start_y + i as f64 * (row_height + 10.0);

            // Draw description text
            Text::new_color(FONT_DEFAULT_COLOR, option_font_size)
                .draw(
                    desc,
                    text_glyphs,
                    &con.draw_state,
                    con.transform.trans(left_margin, y),
                    g,
                )
                .unwrap();

            // Highlight selected button
            let button_color = if i == self.key_binding_selected {
                BUTTON_SELECTED_COLOR // Highlight color for selected button
            } else {
                BUTTON_DEFAULT_COLOR // Default color for unselected buttons
            };

            // Draw button
            draw_button(button_color, right_margin, y - 25.0, 150.0, 40.0, con, g);

            // Highlight selected option
            let option_color = if i == self.key_binding_selected {
                MAIN_MENU_FONT_SELECTED_COLOR // Highlight color for selected option
            } else {
                FONT_DEFAULT_COLOR // Default font color
            };

            // Draw value text
            Text::new_color(option_color, option_font_size)
                .draw(
                    value,
                    button_glyphs,
                    &con.draw_state,
                    con.transform.trans(right_margin + 20.0, y),
                    g,
                )
                .unwrap();
        }
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
        let title_font_size = 48;
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
                MAIN_MENU_FONT_SELECTED_COLOR // Highlight color for selected option
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

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > self.game_settings.snake_speed.as_f64() {
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
        // Grow snake if food is eaten and increment score
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.extend_tail();
            self.score += 1;
        }
    }

    // Check snake collision.
    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (mut next_x, mut next_y) = self.snake.next_head(dir);

        if self.game_settings.wall_wrapping {
            // Wrap around walls if wall wrapping is enabled
            if next_x <= 0 {
                next_x = self.board_width - 2; // Wrap to right side
            } else if next_x >= self.board_width - 1 {
                next_x = 1; // Wrap to left side
            }
            if next_y <= 0 {
                next_y = self.board_height - 2; // Wrap to bottom side
            } else if next_y >= self.board_height - 1 {
                next_y = 1; // Wrap to top side
            }
        }

        // Check if head overlaps body
        if self.snake.overlap_body(next_x, next_y) {
            return false;
        }

        // If wall wrapping is disabled, check for wall collisions
        if !self.game_settings.wall_wrapping {
            next_x > 0
                && next_y > 0
                && next_x < self.board_width - 1
                && next_y < self.board_height - 1
        } else {
            true
        }
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
        let (mut next_x, mut next_y) = self.snake.next_head(dir);

        if self.game_settings.wall_wrapping {
            // Wrap around walls if wall wrapping is enabled
            if next_x <= 0 {
                next_x = self.board_width - 2; // Wrap to right side
            } else if next_x >= self.board_width - 1 {
                next_x = 1; // Wrap to left side
            }
            if next_y <= 0 {
                next_y = self.board_height - 2; // Wrap to bottom side
            } else if next_y >= self.board_height - 1 {
                next_y = 1; // Wrap to top side
            }
            // Check if snake is going to overlap with its body
            if self.snake.overlap_body(next_x, next_y) {
                self.game_state = GameState::GameOver;
            }
            // Move snake to wrapped position
            self.snake.move_forward_to((next_x, next_y), dir);
            self.check_eating();
            self.waiting_time = 0.0;
        } else {
            if self.check_if_snake_alive(dir) {
                self.snake.move_forward(dir);
                self.check_eating();
            } else {
                self.game_state = GameState::GameOver;
            }
            self.waiting_time = 0.0;
        }
    }

    // Start new game without changing settings
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.score = 0;

        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;

        self.game_state = GameState::Playing;
        // self.game_settings = GameSettings::default();
        self.waiting_for_key = None;

        self.main_menu_selected = 0;
        self.game_over_selected = 0;
        self.pause_selected = 0;
        self.settings_selected = 0;
    }
}
