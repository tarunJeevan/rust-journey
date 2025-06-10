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
    // TODO: Add main menu and settings menu
    // TODO: Add game states such as pause, game over, etc.
    // TODO: Add difficulty modes in settings
    // TODO: Add toggleable wall wrapping in settings
    // TODO: Add color customization in settings
    // TODO: Add customizable key bindings in settings

    let mut game = Game::new(width, height);

    while let Some(event) = window.next() {
        // Use match to handle all game states
        match game.get_game_state() {
            GameState::MainMenu => {
                // Handle main menu logic here
                // For now, we will just start the game when any key is pressed
                window.draw_2d(&event, |c, g, _| {
                    clear(BACK_COLOR, g);
                    // TODO: Draw main menu text
                });

                if let Some(Button::Keyboard(_)) = event.press_args() {
                    game.change_game_state(GameState::Playing);
                }
            }
            GameState::Playing => {
                // Handle playing state logic
                if let Some(Button::Keyboard(key)) = event.press_args() {
                    if key == Key::P {
                        // Press 'P' to resume
                        game.change_game_state(GameState::Paused);
                    } else {
                        game.key_pressed(key);
                    }
                }

                // Draw and update the game board
                window.draw_2d(&event, |c, g, _| {
                    clear(BACK_COLOR, g);
                    game.draw(&c, g);
                });
                event.update(|arg| {
                    game.update(arg.dt);
                });
            }
            GameState::Paused => {
                window.draw_2d(&event, |c, g, _| {
                    clear(BACK_COLOR, g);
                    // TODO: Draw pause screen
                });

                // Toggle pause state
                if let Some(Button::Keyboard(key)) = event.press_args() {
                    if key == Key::P {
                        // Press 'P' to resume
                        game.change_game_state(GameState::Playing);
                    }
                }
            }
            GameState::GameOver => {
                window.draw_2d(&event, |c, g, _| {
                    clear(BACK_COLOR, g);
                    // TODO: Draw game over screen
                });
                // Handle game over logic
                if let Some(Button::Keyboard(key)) = event.press_args() {
                    // Press 'R' to restart or 'Q' to quit
                    if key == Key::R {
                        game = Game::new(width, height);
                    } else if key == Key::Q {
                        return;
                    }
                }
            }
            GameState::Settings => {
                window.draw_2d(&event, |c, g, _| {
                    clear(BACK_COLOR, g);
                    // TODO: Draw settings menu
                });
                // Handle settings logic
                // NOTE: For now, we will just return to the main menu when any key is pressed
                if let Some(Button::Keyboard(_)) = event.press_args() {
                    game.change_game_state(GameState::MainMenu);
                }
            }
        }
    }
}
