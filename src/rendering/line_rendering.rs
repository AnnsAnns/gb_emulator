use crate::cpu::{interrupts::PpuMode, CPU};
use macroquad::{color::Color, texture::Image};

// Dots are PPU Cycle conters per Frame
const DOTS_PER_CYCLE: u32 = 4;
const DOTS_PER_LINE: u32 = 456;

const SCAN_DOTS: u32 = 80;
const MIN_DRAW_DOTS: u32 = 172;
#[allow(dead_code)]
const MIN_HBLANK_DOTS: u32 = 87;

const SCANLINES_ACTUAL: u8 = 144;
const SCANLINES_EXTRA: u8 = 10;

const TILES_PER_LINE: u16 = 21;

// Mode 2
pub fn oam_scan(_cpu: &CPU) {}

// Mode 3
pub fn draw_pixels(cpu: &mut CPU, game_diplay: &mut Image, palette: &[Color; 4]) {
    let high_map: bool = false;
    let high_addressing: bool = !cpu.get_lcdc_bg_window_tile_data();

    let scx = cpu.get_lcd_scx();
    let scy = cpu.get_lcd_scy();
    let line: u8 = cpu.get_lcd_y_coordinate();

    let mut display_x: u32 = 0;
    let mut line_x_pos = scx as u32 % 8;

    for xtile in 0..TILES_PER_LINE {
        let tile_index = cpu.get_vram_tile_map(
            high_map,
            (((line + scy) / 8) as u16 % 0x100) * 32 + (xtile + (scx as u16 / 8)) % 32,
        );
        let line_data =
            cpu.get_vram_tile_line(high_addressing, tile_index as u16, (line + scy) % 8);

        for x_pixel in (line_x_pos % 8)..8 {
            if display_x >= game_diplay.width() as u32 {
                break;
            }

            if (line as usize * game_diplay.width() + display_x as usize)
                >= game_diplay.get_image_data().len()
            {
                log::warn!("Pixel out of bounds: x: {}, y: {}", display_x, line);
                continue;
            }

            game_diplay.set_pixel(display_x, line as u32, palette[line_data[x_pixel as usize] as usize]);

            display_x += 1;
            line_x_pos += 1;
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
