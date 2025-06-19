use piston_window::{Context, G2d, Image, rectangle, types::Color};

use crate::game::{BoardTextures, FoodTextures};

pub const BLOCK_SIZE: f64 = 25.0; // Block scaling factor

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

// Draw snake food
pub fn draw_food(x: i32, y: i32, con: &Context, g: &mut G2d, textures: &FoodTextures) {
    let x = to_coord_u32(x) as f64;
    let y = to_coord_u32(y) as f64;

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
    let x = to_coord_u32(x) as f64;
    let y = to_coord_u32(y) as f64;
    let width = to_coord_u32(width) as f64;
    let height = to_coord_u32(height) as f64;

    rectangle(color, [x, y, width, height], con.transform, g);
}

// Draw game board background
pub fn draw_tiled_background(
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
    textures: &BoardTextures,
    tile_tints: &[Vec<[f32; 4]>],
) {
    for x in 0..width {
        for y in 0..height {
            let tint = tile_tints[x as usize][y as usize];
            // Draw grass texture
            Image::new_color(tint)
                .rect([
                    to_coord_u32(x) as f64,
                    to_coord_u32(y) as f64,
                    BLOCK_SIZE,
                    BLOCK_SIZE,
                ])
                .draw(&textures.grass, &con.draw_state, con.transform, g);
        }
    }
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
