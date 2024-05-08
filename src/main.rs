use macroquad::prelude::*;

struct GbSettings {
    scaling: f32,
    palette: [Color; 4],
}

fn update_tile_atlas(
    tile_x: u32,
    tile_y: u32,
    data: &[u8; 16],
    atlas: &mut Image,
    palette: &[Color; 4],
) {
    let offset_x: u32 = tile_x * 8;
    let offset_y: u32 = tile_y * 8;

    for i in 0..64 {
        let data_index = (i / 8) * 2;
        let pal_idx =
            ((data[data_index] >> (i % 8)) & 1) + (((data[data_index + 1] >> (i % 8)) & 1) * 2);

        //println!("LO: {:#x} | HI: {:#x} -> ", data[data_index], data[(data_index)+1]);

        atlas.set_pixel(
            offset_x + 8 - (i as u32 % 8),
            offset_y + (i as u32 / 8),
            palette[pal_idx as usize],
        );
    }
}

fn draw_tile_viewer(offset_x: f32, offset_y: f32, tile_atlas: &Image, gb_settings: &GbSettings) {
    let tex2d_params = DrawTextureParams {
        dest_size: Option::Some(Vec2::new(
            tile_atlas.width() as f32 * gb_settings.scaling,
            tile_atlas.height() as f32 * gb_settings.scaling,
        )),
        source: None,
        rotation: 0.,
        flip_x: false,
        flip_y: false,
        pivot: None,
    };

    let tex2d = Texture2D::from_image(&tile_atlas);
    tex2d.set_filter(FilterMode::Nearest);
    draw_texture_ex(&tex2d, offset_x, offset_y, WHITE, tex2d_params);

    let mouse_pos = mouse_position();

    if mouse_pos.0 >= offset_x
        && mouse_pos.0 < offset_x + 16.0 * 8.0 * gb_settings.scaling
        && mouse_pos.1 >= offset_y
        && mouse_pos.1 < offset_y + 24.0 * 8.0 * gb_settings.scaling
    {
        let x_tile = ((mouse_pos.0 - offset_x) / gb_settings.scaling).floor() as u32 / 8;
        let y_tile = ((mouse_pos.1 - offset_y) / gb_settings.scaling).floor() as u32 / 8;
        let tile_index = y_tile * 16 + x_tile;
        let tile_addr = 0x8000 + tile_index * 16;

        let pos_text = format!(
            "Tile Index: {}, Tile X: {}, Tile Y: {}, ADR: {:#X}",
            tile_index, x_tile, y_tile, tile_addr
        );

        draw_text(
            &pos_text,
            offset_x + 4.0,
            offset_y + 24.0 * 8.0 * gb_settings.scaling + 16.0,
            16.0,
            WHITE,
        );
    }

    fn draw_emulator_controls(offset_x: f32, offset_y: f32) {
        
    }
}

fn draw_gb_display(offset_x: f32, offset_y: f32, image: &Image, gb_settings: &GbSettings) {
    let tex2d_params = DrawTextureParams {
        dest_size: Option::Some(Vec2::new(
            image.width() as f32 * gb_settings.scaling,
            image.height() as f32 * gb_settings.scaling,
        )),
        source: None,
        rotation: 0.,
        flip_x: false,
        flip_y: false,
        pivot: None,
    };

    let tex2d = Texture2D::from_image(&image);
    tex2d.set_filter(FilterMode::Nearest);
    draw_texture_ex(&tex2d, offset_x, offset_y, WHITE, tex2d_params);

    //TODO: Draw actual emulator content
    draw_text(
        "GAME BOII",
        offset_x + 100.0,
        offset_y + 250.0,
        120.0,
        BLACK,
    );
}

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
