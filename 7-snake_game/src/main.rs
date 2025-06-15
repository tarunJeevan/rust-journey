extern crate find_folder;
extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use std::collections::HashMap;

use piston_window::{
    Button, Flip, PistonWindow, PressEvent, Texture, TextureSettings, UpdateEvent, WindowSettings,
    clear, types::Color,
};

use draw::to_coord_u32;
use game::{Game, GameState};

use crate::{
    game::{FoodTextures, SnakeTextures},
    snake::{BodyOrientation, Direction},
};

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0]; // Game board background color

// Load food textures
fn load_food_textures(window: &mut PistonWindow) -> FoodTextures {
    FoodTextures {
        apple: Texture::from_path(
            &mut window.create_texture_context(),
            "assets/food/apple.png",
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap(),
    }
}

// Load snake textures
fn load_snake_textures(window: &mut PistonWindow) -> SnakeTextures {
    let mut head = HashMap::new();
    let mut body = HashMap::new();
    let mut tail = HashMap::new();

    // Load head textures for loop
    let head_directions = [
        (Direction::Up, "assets/snake/head_up.png"),
        (Direction::Down, "assets/snake/head_down.png"),
        (Direction::Left, "assets/snake/head_left.png"),
        (Direction::Right, "assets/snake/head_right.png"),
    ];

    // Load tail textures for loop
    let tail_directions = [
        (Direction::Up, "assets/snake/tail_down.png"),
        (Direction::Down, "assets/snake/tail_up.png"),
        (Direction::Right, "assets/snake/tail_left.png"),
        (Direction::Left, "assets/snake/tail_right.png"),
    ];

    // Load body textures for loop
    let body_orientations = [
        (
            BodyOrientation::Horizontal,
            "assets/snake/body_horizontal.png",
        ),
        (BodyOrientation::Vertical, "assets/snake/body_vertical.png"),
        (BodyOrientation::TurnUL, "assets/snake/body_topleft.png"),
        (BodyOrientation::TurnUR, "assets/snake/body_topright.png"),
        (BodyOrientation::TurnBL, "assets/snake/body_bottomleft.png"),
        (BodyOrientation::TurnBR, "assets/snake/body_bottomright.png"),
    ];

    // Load head textures into HashMap
    for (dir, path) in &head_directions {
        head.insert(
            *dir,
            Texture::from_path(
                &mut window.create_texture_context(),
                path,
                Flip::None,
                &TextureSettings::new(),
            )
            .unwrap(),
        );
    }

    // Load tail textures into HashMap
    for (dir, path) in &tail_directions {
        tail.insert(
            *dir,
            Texture::from_path(
                &mut window.create_texture_context(),
                path,
                Flip::None,
                &TextureSettings::new(),
            )
            .unwrap(),
        );
    }

    // Load body textures into HashMap
    for (orientation, path) in &body_orientations {
        // Insert body textures into HashMap
        body.insert(
            *orientation,
            Texture::from_path(
                &mut window.create_texture_context(),
                path,
                Flip::None,
                &TextureSettings::new(),
            )
            .unwrap(),
        );
    }

    SnakeTextures { head, body, tail }
}

fn main() {
    // Default game width and height (in units)
    let (width, height) = (25, 25);

    // Customize game window
    let mut window: PistonWindow =
        WindowSettings::new("Snake Game", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap(); // FIXME: Return gracefully using error handling. Use unwrap_or_else()?

    // TODO: Add color customization in settings
    // TODO: Add mouse inputs for buttons

    // Load textures
    let snake_textures = load_snake_textures(&mut window);
    let food_textures = load_food_textures(&mut window);

    let mut game = Game::new(width, height, snake_textures, food_textures);

    // Load font for text rendering
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("fonts")
        .unwrap();
    let font_bold = &assets.join("FiraCode-Bold.ttf");
    let font_light = &assets.join("FiraCode-Light.ttf");
    let font_regular = &assets.join("FiraCode-Regular.ttf");

    let mut glyphs_bold = window.load_font(font_bold).unwrap();
    let mut glyphs_light = window.load_font(font_light).unwrap();
    let mut glyphs_regular = window.load_font(font_regular).unwrap();

    while let Some(event) = window.next() {
        // Use match to handle all game states
        match game.get_game_state() {
            GameState::MainMenu => {
                // Draw main menu
                window.draw_2d(&event, |c, g, device| {
                    clear(BACK_COLOR, g);
                    game.draw_main_menu(
                        &c,
                        g,
                        &mut glyphs_bold,
                        &mut glyphs_light,
                        &mut glyphs_regular,
                    );
                    // Flush glyphs to the device
                    glyphs_bold.factory.encoder.flush(device);
                    glyphs_light.factory.encoder.flush(device);
                    glyphs_regular.factory.encoder.flush(device);
                });

                if let Some(Button::Keyboard(key)) = event.press_args() {
                    game.key_pressed(key);
                }
            }
            GameState::Playing => {
                // Draw game board
                window.draw_2d(&event, |c, g, device| {
                    game.draw_game_board(&c, g, &mut glyphs_bold);
                    // Flush glyphs to the device
                    glyphs_bold.factory.encoder.flush(device);
                });
                // Handle playing state logic
                if let Some(Button::Keyboard(key)) = event.press_args() {
                    game.key_pressed(key);
                }
                // Update game state
                event.update(|arg| {
                    game.update(arg.dt);
                });
            }
            GameState::Paused => {
                // Draw pause screen
                window.draw_2d(&event, |c, g, device| {
                    game.draw_pause(
                        &c,
                        g,
                        &mut glyphs_bold,
                        &mut glyphs_light,
                        &mut glyphs_regular,
                    );
                    // Flush glyphs to the device
                    glyphs_bold.factory.encoder.flush(device);
                    glyphs_light.factory.encoder.flush(device);
                    glyphs_regular.factory.encoder.flush(device);
                });
                // Handle playing state logic
                if let Some(Button::Keyboard(key)) = event.press_args() {
                    game.key_pressed(key);
                }
            }
            GameState::GameOver => {
                // Draw game over screen
                window.draw_2d(&event, |c, g, device| {
                    clear(BACK_COLOR, g);
                    game.draw_game_over(&c, g, &mut glyphs_bold, &mut glyphs_regular);
                    // Flush glyphs to the device
                    glyphs_bold.factory.encoder.flush(device);
                    glyphs_regular.factory.encoder.flush(device);
                });
                // Handle game over logic
                if let Some(Button::Keyboard(key)) = event.press_args() {
                    game.key_pressed(key);
                }
            }
            GameState::Settings => {
                // Draw settings screen
                window.draw_2d(&event, |c, g, device| {
                    clear(BACK_COLOR, g);
                    game.draw_settings(
                        &c,
                        g,
                        &mut glyphs_bold,
                        &mut glyphs_light,
                        &mut glyphs_regular,
                    );
                    // Flush glyphs to the device
                    glyphs_bold.factory.encoder.flush(device);
                    glyphs_light.factory.encoder.flush(device);
                    glyphs_regular.factory.encoder.flush(device);
                });
                // Handle settings logic
                if let Some(Button::Keyboard(key)) = event.press_args() {
                    game.key_pressed(key);
                }
            }
            GameState::KeyBinding => {
                // Draw key binding screen
                window.draw_2d(&event, |c, g, device| {
                    clear(BACK_COLOR, g);
                    game.draw_key_bindings(
                        &c,
                        g,
                        &mut glyphs_bold,
                        &mut glyphs_light,
                        &mut glyphs_regular,
                    );
                    // Flush glyphs to the device
                    glyphs_bold.factory.encoder.flush(device);
                    glyphs_light.factory.encoder.flush(device);
                    glyphs_regular.factory.encoder.flush(device);
                });
                // Handle key binding logic
                if let Some(Button::Keyboard(key)) = event.press_args() {
                    game.key_pressed(key);
                }
            }
        }
    }
}
