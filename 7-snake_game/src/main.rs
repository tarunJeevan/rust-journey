extern crate piston_window;
extern crate rand;

mod draw;
mod game;
mod snake;

use piston_window::{types::Color, *};

use draw::to_coord_u32;
use game::Game;

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
