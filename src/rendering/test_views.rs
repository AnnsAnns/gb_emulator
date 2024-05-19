use macroquad::prelude::*;
use crate::cpu;
use crate::memory::raw_memory_operations::test_helper::*;
use crate::rendering::tiles::*;
use crate::rendering::views::*;


#[macroquad::test]
async fn golden_image_vram_viewer() {
    // Inititalize General Settings
    const palette: [Color; 4] = [
        Color::new(1.00, 1.00, 1.00, 1.00),
        Color::new(0.18, 0.83, 0.18, 1.00),
        Color::new(0.12, 0.54, 0.12, 1.00),
        Color::new(0.06, 0.15, 0.06, 1.00)
    ];
    const scaling: f32 = 4.0;

    let mut tile_atlas = Image::gen_image_color(8 * 16, 8 * 24, WHITE);

    request_new_screen_size(
        8.0 * 16.0 * scaling,
        8.0 * 24.0 * scaling,
    );

    let mut cpu = cpu::CPU::new();

    file_to_memory(
        &mut cpu.get_memory(),
        0x8000,
        "test_files/cgbBCE1-VRAM.bin",
    );

    update_atlas_from_memory(
        &cpu.get_memory(),
        16 * 24,
        &mut tile_atlas,
        &palette,
    );

    loop {
        draw_tile_viewer(0.0, 0.0, &tile_atlas, scaling);

        next_frame().await;
    }
}

#[macroquad::test]
async fn golden_image_layout() {
    // Inititalize General Settings
    const palette: [Color; 4] = [
        Color::new(1.00, 1.00, 1.00, 1.00),
        Color::new(0.18, 0.83, 0.18, 1.00),
        Color::new(0.12, 0.54, 0.12, 1.00),
        Color::new(0.06, 0.15, 0.06, 1.00)
    ];
    const scaling: f32 = 4.0;


    let mut tile_atlas = Image::gen_image_color(8 * 16, 8 * 24, WHITE);
    let combined_image = Image::gen_image_color(160, 144, GREEN);

    #[rustfmt::skip]
    let test_tile: [u8; 16] = [
        0xFF, 0x00, 0x7E, 0xFF, 0x85, 0x81, 0x89, 0x83, 
        0x93, 0x85, 0xA5, 0x8B, 0xC9, 0x97, 0x7E, 0xFF
    ];

    request_new_screen_size(
        (160.0 + 8.0 * 16.0) * scaling + 15.0,
        (8.0 * 24.0) * scaling + 25.0,
    );

    loop {
        update_tile_atlas(9, &test_tile, &mut tile_atlas, &palette);

        draw_gb_display(5.0, 5.0, &combined_image, scaling);
        draw_tile_viewer(
            combined_image.width() as f32 * scaling + 10.0,
            5.0,
            &tile_atlas,
            scaling,
        );

        next_frame().await;
    }
}

#[macroquad::test]
async fn golden_image_background_viewer() {
    // Inititalize General Settings
    const palette: [Color; 4] = [
        Color::new(1.00, 1.00, 1.00, 1.00),
        Color::new(0.18, 0.83, 0.18, 1.00),
        Color::new(0.12, 0.54, 0.12, 1.00),
        Color::new(0.06, 0.15, 0.06, 1.00)
    ];
    const scaling: f32 = 4.0;

    let mut background_image = Image::gen_image_color(32 * 8, 32 * 8, PINK);
    let mut tile_atlas = Image::gen_image_color(8 * 16, 8 * 24, WHITE);

    let mut cpu = cpu::CPU::new();

    file_to_memory(
        &mut cpu.get_memory(),
        0x8000,
        "test_files/Mindy1-VRAM.bin",
    );

    request_new_screen_size(
        (32.0 * 8.0 + 16.0 * 8.0) * scaling,
        (32.0 * 8.0) * scaling,
    );

    loop {

        update_atlas_from_memory(&cpu.get_memory(), 16*24, &mut tile_atlas, &palette);

        update_background_from_memory(&cpu.get_memory(), &tile_atlas, &mut background_image);

        draw_tile_viewer(
            (32.0 * 8.0) * scaling + 10.0,
            0.0,
            &tile_atlas,
            scaling,
        );

        draw_background_viewer(0.0, 0.0, &background_image, scaling);

        next_frame().await;
    }
}