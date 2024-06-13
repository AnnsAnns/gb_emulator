use macroquad::{color::Color, texture::Image};

use crate::cpu::CPU;

// Dots are PPU Cycle conters per Frame
const DOTS_PER_CPU_CYCLE: u32 = 4;
const DOTS_PER_LINE: u32 = 456;

#[derive(Copy, Clone)]
pub enum PpuMode {
    HorizontalBlank = 0,
    VerticalBlank = 1,
    OamScan = 2,
    Drawing = 3,
}

// Mode 2
pub fn oam_scan(cpu: &CPU) {}

// Mode 3
pub fn draw_pixels(cpu: &mut CPU, game_diplay: &mut Image, palette: &[Color; 4]) {
    let high_map: bool = false;
    let high_addressing: bool = false;

    let scx = cpu.get_lcd_scx();
    let scy = cpu.get_lcd_scy();
    let line: u8 = cpu.get_lcd_y_coordinate();

    for xtile in 0..20 {
        let tile_index = cpu.get_vram_tile_map(high_map, (line as u16 / 8) * 32 + xtile);
        let line_data =
            cpu.get_vram_tile_line(high_addressing, tile_index as u16, (line % 8) as u8);

        for x_pixel in 0..8 {
            //log::info!("Drawing pixel at x: {}, y: {}, xtile: {}, line: {}, color: {}", xtile as u32 * 8 + x_pixel, line as u32, xtile, line, line_data[x_pixel as usize]);

            let width = game_diplay.width();
            let x = xtile as u32 * 8 + x_pixel;
            let y = line as u32;
            let image_len = game_diplay.get_image_data().len();

            if (y * width as u32 + x) as usize >= image_len {
                log::warn!("Pixel out of bounds: x: {}, y: {}", x, y);
                continue;
            }

            game_diplay.set_pixel(x, y, palette[line_data[x_pixel as usize] as usize]);
        }
    }
}

pub struct Ppu {
    ppu_mode: PpuMode,
    scanline: u8,
    frame_cycles: u32
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            ppu_mode: PpuMode::OamScan,
            scanline: 0,
            frame_cycles: 0
        }
    }

    pub fn step(&mut self, cpu: &mut CPU, final_image: &mut Image, palette: &[Color; 4]) {
        let dot = (self.frame_cycles) * DOTS_PER_CPU_CYCLE;
        //log::info!("Dot calculation was: {}", dot);
        //log::info!("Scanline: {}", scanline);
        cpu.set_lcd_y_coordinate(self.scanline);

        let mut do_frame_cycle = true;

        match self.ppu_mode {
            PpuMode::OamScan => {
                if dot % DOTS_PER_LINE >= 80 {
                    oam_scan(&cpu);
                    self.ppu_mode = PpuMode::Drawing;
                }
            }
            PpuMode::Drawing => {
                if dot % DOTS_PER_LINE >= 172 + 80 {
                    draw_pixels(cpu, final_image, &palette);
                    self.ppu_mode = PpuMode::HorizontalBlank;
                }
            }
            PpuMode::HorizontalBlank => {
                if dot % DOTS_PER_LINE == 0 {
                    self.scanline += 1;
                    self.ppu_mode = if self.scanline <= 143 {
                        PpuMode::OamScan
                    } else {
                        // Set the VBlank interrupt since we are done with the frame
                        cpu.set_vblank_interrupt();
                        PpuMode::VerticalBlank
                    };
                }
            }
            PpuMode::VerticalBlank => {
                //log::info!("Dot: {}", dot % DOTS_PER_LINE);
                if dot % DOTS_PER_LINE >= 450 {
                    //log::info!("Scanline: {}", scanline);
                    if self.scanline >= 153 {
                        //log::info!("End of frame");
                        self.ppu_mode = PpuMode::OamScan;
                        self.scanline = 0;
                        //log::info!("Frame: {} - Resetting", frame_cycles);
                        self.frame_cycles = 0;
                        do_frame_cycle = false;
                    }

                    if do_frame_cycle {
                        self.scanline += 1;
                    }
                }
            }
        }

        cpu.set_ppu_mode(self.ppu_mode as u8);
        if do_frame_cycle {
            self.frame_cycles += 1;
        }
    }

    pub fn get_dot(&self) -> u32 {
        self.frame_cycles * DOTS_PER_CPU_CYCLE
    }

    pub fn get_frame_cycles(&self) -> u32 {
        self.frame_cycles
    }
}
