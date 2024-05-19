use macroquad::prelude::*;

pub fn draw_tile_viewer(
    offset_x: f32,
    offset_y: f32,
    tile_atlas: &Image,
    scaling: f32,
) {
    let tex2d_params = DrawTextureParams {
        dest_size: Option::Some(Vec2::new(
            tile_atlas.width() as f32 * scaling,
            tile_atlas.height() as f32 * scaling,
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
        && mouse_pos.0 < offset_x + 16.0 * 8.0 * scaling
        && mouse_pos.1 >= offset_y
        && mouse_pos.1 < offset_y + 24.0 * 8.0 * scaling
    {
        let x_tile = ((mouse_pos.0 - offset_x) / scaling).floor() as u32 / 8;
        let y_tile = ((mouse_pos.1 - offset_y) / scaling).floor() as u32 / 8;
        let tile_index = y_tile * 16 + x_tile;
        let tile_addr = 0x8000 + tile_index * 16;

        let pos_text = format!(
            "Tile Index: {}, Tile X: {}, Tile Y: {}, ADR: {:#X}",
            tile_index, x_tile, y_tile, tile_addr
        );

        draw_text(
            &pos_text,
            offset_x + 4.0,
            offset_y + 24.0 * 8.0 * scaling + 16.0,
            16.0,
            WHITE,
        );
    }
}

pub fn draw_gb_display(offset_x: f32, offset_y: f32, image: &Image, scaling: f32) {
    let tex2d_params = DrawTextureParams {
        dest_size: Option::Some(Vec2::new(
            image.width() as f32 * scaling,
            image.height() as f32 * scaling,
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

pub fn draw_background_viewer(offset_x: f32, offset_y: f32, image: &Image, scaling: f32) {
    let tex2d_params = DrawTextureParams {
        dest_size: Option::Some(Vec2::new(
            image.width() as f32 * scaling,
            image.height() as f32 * scaling,
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
}