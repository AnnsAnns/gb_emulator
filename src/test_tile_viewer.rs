use self::memory::raw_memory_operations::test_helper::file_to_memory;
use super::*;

#[macroquad::test]
async fn golden_image_vram_viewer() {
    // Inititalize General Settings
    let gb_settings = GbSettings {
        ..Default::default()
    };

    let mut tile_atlas = Image::gen_image_color(8 * 16, 8 * 24, WHITE);

    request_new_screen_size(
        8.0 * 16.0 * gb_settings.scaling,
        8.0 * 24.0 * gb_settings.scaling,
    );

    let mut cpu = cpu::CPU::new();

    file_to_memory(
        &mut cpu.get_memory(),
        0x8000,
        "test_files/vram-read-cgbBCE-vram.txt",
    );

    update_atlas_from_memory(
        &cpu.get_memory(),
        16 * 24,
        &mut tile_atlas,
        &gb_settings.palette,
    );

    loop {
        draw_tile_viewer(0.0, 0.0, &tile_atlas, &gb_settings);

        next_frame().await;
    }
}

#[macroquad::test]
async fn golden_image_layout() {
    // Inititalize General Settings
    let gb_settings = GbSettings {
        ..Default::default()
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
        update_tile_atlas(9, &test_tile, &mut tile_atlas, &gb_settings.palette);

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
