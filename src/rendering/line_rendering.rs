use macroquad::texture::Image;

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
pub fn draw_pixels(cpu: &CPU, game_diplay: &mut Image) {
    
}