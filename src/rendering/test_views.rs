use crate::cpu;
use crate::memory::raw_memory_operations::test_helper::*;
use crate::rendering::tiles::*;
use crate::rendering::views::*;
use macroquad::prelude::*;

#[macroquad::test]
async fn golden_image_tile_viewer() {
    // Inititalize General Settings
    const PALETTE: [Color; 4] = [
        Color::new(1.00, 1.00, 1.00, 1.00),
        Color::new(0.18, 0.83, 0.18, 1.00),
        Color::new(0.12, 0.54, 0.12, 1.00),
        Color::new(0.06, 0.15, 0.06, 1.00),
    ];
    const SCALING: f32 = 4.0;

    let mut atlas = Image::gen_image_color(8 * 16, 8 * 24, WHITE);

    let mut cpu = cpu::CPU::new();

    file_to_memory(&mut cpu.get_memory(), 0x8000, "test_files/cgbBCE1-VRAM.bin");

    let mut tile_viewer = TileViewer {
        offset_x: 0.0,
        offset_y: 0.0,
        scaling: SCALING,
    };

    let window_size = tile_viewer.size();

    request_new_screen_size(window_size.x, window_size.y);

    update_atlas_from_memory(&cpu.get_memory(), 16 * 24, &mut atlas, &PALETTE);

    loop {
        tile_viewer.draw(&atlas);

        next_frame().await;
    }
}

#[macroquad::test]
async fn golden_image_background_viewer() {
    // Inititalize General Settings
    const PALETTE: [Color; 4] = [
        Color::new(1.00, 1.00, 1.00, 1.00),
        Color::new(0.18, 0.83, 0.18, 1.00),
        Color::new(0.12, 0.54, 0.12, 1.00),
        Color::new(0.06, 0.15, 0.06, 1.00),
    ];
    const SCALING: f32 = 4.0;

    let mut background_image = Image::gen_image_color(32 * 8, 32 * 8, PINK);
    let mut tile_atlas = Image::gen_image_color(8 * 16, 8 * 24, WHITE);

    let mut cpu = cpu::CPU::new();

    let mut background_viewer = BackgroundViewer {
        offset_x: 0.0,
        offset_y: 0.0,
        scaling: SCALING,
    };

    file_to_memory(&mut cpu.get_memory(), 0x8000, "test_files/Mindy1-VRAM.bin");

    let window_size = background_viewer.size();

    request_new_screen_size(window_size.x, window_size.y);

    loop {
        update_atlas_from_memory(&cpu.get_memory(), 16 * 24, &mut tile_atlas, &PALETTE);
        update_background_from_memory(&cpu.get_memory(), &tile_atlas, &mut background_image);
        background_viewer.draw(&background_image);
        next_frame().await;
    }
}

#[macroquad::test]
async fn golden_image_layout() {
    // Inititalize General Settings
    const PALETTE: [Color; 4] = [
        Color::new(1.00, 1.00, 1.00, 1.00),
        Color::new(0.18, 0.83, 0.18, 1.00),
        Color::new(0.12, 0.54, 0.12, 1.00),
        Color::new(0.06, 0.15, 0.06, 1.00),
    ];
    const SCALING: f32 = 4.0;

    let final_image = Image::gen_image_color(160, 144, GREEN);
    let mut gb_display = GbDisplay {
        offset_x: 5.0,
        offset_y: 5.0,
        scaling: SCALING,
    };
    let gb_display_size = gb_display.size(&final_image);

    let mut background_viewer = BackgroundViewer {
        offset_x: gb_display_size.x + 10.0,
        offset_y: 5.0,
        scaling: SCALING / 2.0,
    };
    let mut background_image = Image::gen_image_color(32 * 8, 32 * 8, PINK);
    let background_viewer_size = background_viewer.size();

    let mut tile_atlas = Image::gen_image_color(8 * 16, 8 * 24, WHITE);
    let mut tile_viewer = TileViewer {
        offset_x: gb_display_size.x + background_viewer_size.x + 15.0,
        offset_y: 5.0,
        scaling: SCALING,
    };
    let tile_viewer_size = tile_viewer.size();

    request_new_screen_size(
        background_viewer_size.x + tile_viewer_size.x + gb_display_size.x + 20.0,
        tile_viewer_size.y + 10.0,
    );

    let mut cpu = cpu::CPU::new();

    file_to_memory(&mut cpu.get_memory(), 0x8000, "test_files/Mindy1-VRAM.bin");

    update_atlas_from_memory(&cpu.get_memory(), 16 * 24, &mut tile_atlas, &PALETTE);
    update_background_from_memory(&cpu.get_memory(), &tile_atlas, &mut background_image);

    loop {
        //update_tile_atlas(9, &test_tile, &mut tile_atlas, &PALETTE);

        background_viewer.draw(&background_image);

        gb_display.draw(&final_image);

        tile_viewer.draw(&tile_atlas);

        next_frame().await;
    }
}
