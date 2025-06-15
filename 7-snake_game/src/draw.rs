use piston_window::{Context, G2d, Image, rectangle, types::Color};

use crate::game::FoodTextures;

pub const BLOCK_SIZE: f64 = 25.0; // Block scaling factor

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

// Draw snake food
pub fn draw_food(x: i32, y: i32, con: &Context, g: &mut G2d, textures: &FoodTextures) {
    let x = to_coord(x);
    let y = to_coord(y);

    Image::new().rect([x, y, BLOCK_SIZE, BLOCK_SIZE]).draw(
        &textures.apple,
        &con.draw_state,
        con.transform,
        g,
    );
}

// Draw screen
pub fn draw_screen(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);
    let width = to_coord(width);
    let height = to_coord(height);

    rectangle(color, [x, y, width, height], con.transform, g);
}

// Draw screen buttons
pub fn draw_button(
    color: Color,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    con: &Context,
    g: &mut G2d,
) {
    rectangle(color, [x, y, width, height], con.transform, g);
}
