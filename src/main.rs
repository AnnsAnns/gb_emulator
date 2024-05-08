use macroquad::prelude::*;
use rendering::{
    render_settings::*, tiles::*, views::*
};

pub mod rendering;

#[macroquad::main("GB Emulator")]
async fn main() {
    // Inititalize General Settings
    let gb_settings = GbSettings {
        scaling: 4.0,
        palette: [
            Color::from_hex(0xFFFFFF),
            Color::from_hex(0x81C784),
            Color::from_hex(0x43A047),
            Color::from_hex(0x1B5E20),
        ],
    };

    let mut tile_atlas = Image::gen_image_color(8 * 16, 8 * 24, WHITE);
    let combined_image = Image::gen_image_color(160, 144, GREEN);

    #[rustfmt::skip]
    let test_tile: [u8; 16] = [
        0xFF, 0x00, 0x7E, 0xFF, 0x85, 0x81, 0x89, 0x83, 
        0x93, 0x85, 0xA5, 0x8B, 0xC9, 0x97, 0x7E, 0xFF
    ];

    request_new_screen_size(
        (160.0 + 8.0 * 16.0) * gb_settings.scaling + 15.0,
        (8.0 * 24.0) * gb_settings.scaling + 25.0,
    );

    loop {
        update_tile_atlas(1, 1, &test_tile, &mut tile_atlas, &gb_settings.palette);

        draw_gb_display(5.0, 5.0, &combined_image, &gb_settings);
        draw_tile_viewer(
            combined_image.width() as f32 * gb_settings.scaling + 10.0,
            5.0,
            &tile_atlas,
            &gb_settings,
        );

        next_frame().await;
    }
}
