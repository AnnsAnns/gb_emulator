
use macroquad::prelude::*;

pub fn update_tile_atlas(
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
