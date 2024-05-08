use std::usize;

use macroquad::prelude::*;

use crate::{memory::Memory, GbSettings};

pub fn update_tile_atlas(
    tile_index: u32,
    data: &[u8; 16],
    atlas: &mut Image,
    palette: &[Color; 4],
) {
    let offset_x: u32 = (tile_index % 16) * 8;
    let offset_y: u32 = (tile_index / 16) * 8;

    for i in 0..64 {
        let data_index = (i / 8) * 2;
        let pal_idx =
            ((data[data_index] >> (i % 8)) & 1) + (((data[data_index + 1] >> (i % 8)) & 1) * 2);

        //println!("LO: {:#x} | HI: {:#x} -> ", data[data_index], data[(data_index)+1]);

        atlas.set_pixel(
            offset_x + 7 - (i as u32 % 8),
            offset_y + (i as u32 / 8),
            palette[pal_idx as usize],
        );
    }
}

pub fn update_atlas_from_memory(
    memory: &Memory,
    tile_count: usize,
    tile_atlas: &mut Image,
    palette: &[Color; 4],
) {
    for tile_index in 0..tile_count as u16 {
        let mut tile_data: [u8; 16] = [0; 16];

        for byte_index in 0..16 as u16 {
            let address: u16 = 0x8000 + byte_index + tile_index * 16;
            tile_data[byte_index as usize] = memory.read_byte(address);
            //println!("{:#X}", tile_data[byte_index as usize]);
            //println!("{}", address);
        }

        update_tile_atlas(tile_index as u32, &tile_data, tile_atlas, palette);
    }
}
