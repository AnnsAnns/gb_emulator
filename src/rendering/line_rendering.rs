use macroquad::{color::Color, texture::Image};

use crate::cpu::CPU;

#[derive(Copy, Clone)]
pub enum PpuMode {
    HorizontalBlank = 0,
    VerticalBlank = 1,
    OamScan = 2,
    Drawing = 3
}

// Mode 2
pub fn oam_scan(cpu: &CPU) {

}

// Mode 3
pub fn draw_pixels(cpu: &mut CPU, game_diplay: &mut Image, palette: &[Color; 4]) {
        
        let high_map: bool = false;
        let high_addressing: bool = false;

        let scx = cpu.get_lcd_scx();
        let scy = cpu.get_lcd_scy();
        let line: u8 = cpu.get_lcd_y_coordinate();

        for xtile in 0..20 {
            let tile_index = cpu.get_vram_tile_map(high_map, (line as u16 / 8)*32 + xtile);
            let line_data = cpu.get_vram_tile_line(high_addressing, tile_index as u16, (line % 8) as u8);

            for x_pixel in 0..8 {
                game_diplay.set_pixel(
                    xtile as u32 * 8 + x_pixel,
                    line as u32,
                    palette[line_data[x_pixel as usize] as usize],
                );
            }
        }
}