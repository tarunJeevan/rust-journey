extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use piston_window::{types::Color, *};

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

    while let Some(event) = window.next() {
        // Use match to handle all game states
        match game.get_game_state() {
            GameState::MainMenu => {
                // Draw main menu
                window.draw_2d(&event, |c, g, _| {
                    clear(BACK_COLOR, g);
                    game.draw_main_menu(&c, g);
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
