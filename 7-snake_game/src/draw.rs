use piston_window::{Context, G2d, rectangle, types::Color};

pub const BLOCK_SIZE: f64 = 32.0; // Block scaling factor

pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

// Draw blocks for snake body and food
pub fn draw_block(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(color, [x, y, BLOCK_SIZE, BLOCK_SIZE], con.transform, g);
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
