use crate::cpu::{interrupts::PpuMode, CPU};
use macroquad::{color::Color, texture::Image};

// Dots are PPU Cycle conters per Frame
const DOTS_PER_CYCLE: u32 = 4;
const DOTS_PER_LINE: u32 = 456;

const SCAN_DOTS: u32 = 80;
const MIN_DRAW_DOTS: u32 = 172;
const MIN_HBLANK_DOTS: u32 = 87;

const SCANLINES_ACTUAL: u8 = 144;
const SCANLINES_EXTRA: u8 = 10;

// Mode 2
pub fn oam_scan(_cpu: &CPU) {}

// Mode 3
pub fn draw_pixels(cpu: &mut CPU, game_diplay: &mut Image, palette: &[Color; 4]) {
    let high_map: bool = false;
    let high_addressing: bool = !cpu.get_lcdc_bg_window_tile_data();

    let _scx = cpu.get_lcd_scx();
    let _scy = cpu.get_lcd_scy();
    let line: u8 = cpu.get_lcd_y_coordinate();

    for xtile in 0..20 {
        let tile_index = cpu.get_vram_tile_map(high_map, (line as u16 / 8) * 32 + xtile);
        let line_data =
            cpu.get_vram_tile_line(high_addressing, tile_index as u16, line % 8);

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
    frame_cycles: u32,
    enabled: bool,
}

impl Default for Ppu {
    fn default() -> Self {
        Self::new()
    }
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            frame_cycles: 0,
            enabled: false,
        }
    }

    pub fn step(&mut self, cpu: &mut CPU, final_image: &mut Image, palette: &[Color; 4]) {
        if cpu.get_lcdc_ppu_enabled() && !self.enabled {
            self.frame_cycles = 0;
            self.enabled = true;
        }

        let dot = self.frame_cycles * DOTS_PER_CYCLE;
        self.frame_cycles += 1;

        let ppu_mode = PpuMode::try_from(cpu.get_ppu_mode()).expect("Invalid PPU Mode");
        let scanline = cpu.get_lcd_y_coordinate();

        match ppu_mode {
            PpuMode::OamScan => {
                if dot % DOTS_PER_LINE == SCAN_DOTS - DOTS_PER_CYCLE {
                    oam_scan(cpu);
                    cpu.set_ppu_mode(PpuMode::Drawing);
                } else if dot % DOTS_PER_LINE >= SCAN_DOTS {
                    panic!("dot must be < 80 in OAM Scan Mode");
                }
            }
            PpuMode::Drawing => {
                // TODO Implement Variable Drawing Mode duration
                if dot % DOTS_PER_LINE == SCAN_DOTS + MIN_DRAW_DOTS - DOTS_PER_CYCLE {
                    draw_pixels(cpu, final_image, palette);
                    cpu.set_ppu_mode(PpuMode::HorizontalBlank);
                } else if dot % DOTS_PER_LINE >= SCAN_DOTS + MIN_DRAW_DOTS {
                    panic!("dot has an invalid value");
                }
            }
            PpuMode::HorizontalBlank => {
                if dot % DOTS_PER_LINE == DOTS_PER_LINE - DOTS_PER_CYCLE {
                    cpu.set_lcd_y_coordinate(scanline + 1);

                    // Check if in extra scanlines area
                    if scanline + 1 < SCANLINES_ACTUAL {
                        cpu.set_ppu_mode(PpuMode::OamScan);
                    } else {
                        // Set the VBlank interrupt since we are done with the frame
                        cpu.set_vblank_interrupt();
                        cpu.set_ppu_mode(PpuMode::VerticalBlank);
                    };
                }
            }
            PpuMode::VerticalBlank => {
                //log::info!("Dot: {}", dot % DOTS_PER_LINE);
                if dot % DOTS_PER_LINE == DOTS_PER_LINE - DOTS_PER_CYCLE {
                    cpu.set_lcd_y_coordinate(scanline + 1);

                    if scanline + 1 == SCANLINES_ACTUAL + SCANLINES_EXTRA - 1 {
                        self.frame_cycles = 0;
                        cpu.set_lcd_y_coordinate(0);
                        cpu.set_ppu_mode(PpuMode::OamScan)
                    }
                }
            }
        }
    }

    pub fn get_dot(&self) -> u32 {
        self.frame_cycles * DOTS_PER_CYCLE
    }

    pub fn get_frame_cycles(&self) -> u32 {
        self.frame_cycles
    }
}
