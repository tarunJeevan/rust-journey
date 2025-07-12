use piston_window::{
    CharacterCache, Context, G2d, Glyphs, Image, Text, Transformed, rectangle, types::Color,
};

use crate::game::{BoardTextures, FoodTextures};

pub const BLOCK_SIZE: f64 = 25.0; // Block scaling factor

const CURSOR_OFFSET: f64 = 2.0;
const INPUT_COLOR: Color = [0.0, 0.0, 0.0, 0.7]; // Input field background color

pub struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Rect {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    fn to_arr(&self) -> [f64; 4] {
        [self.x, self.y, self.width, self.height]
    }
}

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
pub fn draw_button(color: Color, rect: Rect, con: &Context, g: &mut G2d) {
    rectangle(color, rect.to_arr(), con.transform, g);
}

// Draw input text box
pub fn draw_input_field(
    show_cursor: bool,
    content: &str,
    input_glyphs: &mut Glyphs,
    text_color: Color,
    rect: Rect,
    con: &Context,
    g: &mut G2d,
) {
    // Draw input field rectangle
    rectangle(INPUT_COLOR, rect.to_arr(), con.transform, g);

    // Calculate parameters for drawing input text
    let input = content;
    let input_font_size = 32;
    let input_width = input_glyphs.width(input_font_size, input).unwrap_or(0.0);

    // Calculate input text position to be inside rectangle
    let input_x = rect.x + (rect.width - input_width) / 2.0;
    let input_y = rect.y + rect.height / 2.0 + input_font_size as f64 / 2.5;

    // Draw title text
    Text::new_color(text_color, input_font_size)
        .draw(
            input,
            input_glyphs,
            &con.draw_state,
            con.transform.trans(input_x, input_y),
            g,
        )
        .unwrap();

    // Draw cursor if needed
    if show_cursor {
        // Calculate cursor position
        let cursor_x = input_x + input_width + CURSOR_OFFSET;
        let cursor_y = input_y - input_font_size as f64 * 0.8;
        let cursor_height = input_font_size as f64 * 1.1;
        let cursor_width = 3.0;

        // Draw cursor
        rectangle(
            text_color,
            [cursor_x, cursor_y, cursor_width, cursor_height],
            con.transform,
            g,
        );
    }
}
