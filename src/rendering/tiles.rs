use std::usize;

use crate::cpu::CPU;
use macroquad::prelude::*;

pub fn update_tile_atlas(
    tile_index: u32,
    data: &[[u8; 8]; 8],
    atlas: &mut Image,
    palette: &[Color; 4],
) {
    let offset_x: u32 = (tile_index % 16) * 8;
    let offset_y: u32 = (tile_index / 16) * 8;

    for pixel_index in 0..64 {
        let pos_x = pixel_index % 8;
        let pos_y = pixel_index / 8;

        atlas.set_pixel(
            offset_x + pos_x,
            offset_y + pos_y,
            palette[data[pos_y as usize][pos_x as usize] as usize],
        );
    }
}

pub fn update_atlas_from_memory(
    cpu: &CPU,
    tile_count: usize,
    tile_atlas: &mut Image,
    palette: &[Color; 4],
) {
    for tile_index in 0..tile_count as u16 {
        let mut tile_data: [[u8; 8]; 8] = [[0; 8]; 8];

        for line in 0..8 {
            tile_data[line] = cpu.get_vram_tile_line(false, tile_index, line as u8);
        }

        update_tile_atlas(tile_index as u32, &tile_data, tile_atlas, palette);
    }
}

pub fn update_background_from_memory(cpu: &CPU, background: &mut Image, palette: &[Color; 4], high_map: bool, high_adressing: bool) {
    for line in 0..32 * 8 {
        for xtile in 0..32 {
            let tile_index = cpu.get_vram_tile_map(high_map, (line / 8)*32 + xtile);
            let line_data = cpu.get_vram_tile_line(high_adressing, tile_index as u16, (line % 8) as u8);

            for x_pixel in 0..8 {
                background.set_pixel(
                    xtile as u32 * 8 + x_pixel,
                    line as u32,
                    palette[line_data[x_pixel as usize] as usize],
                );
            }
        }
    }
}
