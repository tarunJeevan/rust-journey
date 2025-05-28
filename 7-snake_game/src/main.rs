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
        // TODO: Use match to handle all game states
        match game.get_game_state() {
            GameState::MainMenu => {
                // Handle main menu logic here
                // For now, we will just start the game when any key is pressed
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
                    }
                    game.key_pressed(key);
                }
            }
            GameState::Paused => {
                // Handle paused state logic
                if let Some(Button::Keyboard(key)) = event.press_args() {
                    if key == Key::P {
                        // Press 'P' to resume
                        game.change_game_state(GameState::Playing);
                    }
                }
            }
            GameState::GameOver => {
                // Handle game over logic
                if let Some(Button::Keyboard(key)) = event.press_args() {
                    if key == Key::R {
                        // Press 'R' to restart
                        game = Game::new(width, height);
                    } else if key == Key::Q {
                        // Press 'Q' to quit
                        return;
                    }
                }
            }
        }
        // Handle key presses
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }

        // Handle screen rendering
        window.draw_2d(&event, |c, g, _d| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        // Handle event loop
        event.update(|arg| {
            game.update(arg.dt);
        });
    }
}
