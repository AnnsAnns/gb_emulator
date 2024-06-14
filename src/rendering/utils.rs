use macroquad::{color::WHITE, text::draw_text};

pub fn draw_scaled_text(
    offset_x: f32,
    offset_y: f32,
    text: &str,
    scaling: f32,
) {
    draw_text(
        text,
        offset_x + 4.0,
        offset_y + 24.0 * 8.0 * scaling + 16.0,
        16.0,
        WHITE,
    );
}