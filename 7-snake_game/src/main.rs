extern crate find_folder;
extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use piston_window::{
    Button, PistonWindow, PressEvent, UpdateEvent, WindowSettings, clear, types::Color,
};

use draw::to_coord_u32;
use game::{Game, GameState};

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0]; // Game board background color

fn main() {
    // Default game width and height (in units)
    let (width, height) = (20, 20);

    // Customize game window
    let mut window: PistonWindow =
        WindowSettings::new("Snake Game", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap(); // FIXME: Return gracefully using error handling

    // TODO: Add scoring system
    // TODO: Add difficulty modes in settings
    // TODO: Add toggleable wall wrapping in settings
    // TODO: Add color customization in settings

    let mut game = Game::new(width, height);

    // Load font for text rendering
    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let font_path = &assets.join("FiraCode-Bold.ttf");
    let mut glyphs = window.load_font(font_path).unwrap();

    while let Some(event) = window.next() {
        // Use match to handle all game states
        match game.get_game_state() {
            GameState::MainMenu => {
                // Draw main menu
                window.draw_2d(&event, |c, g, device| {
                    clear(BACK_COLOR, g);
                    game.draw_main_menu(&c, g, &mut glyphs);
                    // Flush glyphs to the device
                    glyphs.factory.encoder.flush(device);
                });

                if let Some(Button::Keyboard(key)) = event.press_args() {
                    game.key_pressed(key);
                }
            }
            GameState::Playing => {
                // Draw game board
                window.draw_2d(&event, |c, g, _| {
                    clear(BACK_COLOR, g);
                    game.draw_game_board(&c, g);
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
                window.draw_2d(&event, |c, g, _| {
                    clear(BACK_COLOR, g);
                    game.draw_pause(&c, g);
                });
                // Handle playing state logic
                if let Some(Button::Keyboard(key)) = event.press_args() {
                    game.key_pressed(key);
                }
            }
            GameState::GameOver => {
                // Draw game over screen
                window.draw_2d(&event, |c, g, _| {
                    clear(BACK_COLOR, g);
                    game.draw_game_over(&c, g);
                });
                // Handle game over logic
                if let Some(Button::Keyboard(key)) = event.press_args() {
                    game.key_pressed(key);
                }
            }
            GameState::Settings => {
                // Draw settings screen
                window.draw_2d(&event, |c, g, _| {
                    clear(BACK_COLOR, g);
                    game.draw_settings(&c, g);
                });
                // Handle settings logic
                if let Some(Button::Keyboard(key)) = event.press_args() {
                    game.key_pressed(key);
                }
            }
        }
    }
}
