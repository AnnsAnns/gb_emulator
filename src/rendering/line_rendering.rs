use crate::cpu::{interrupts::PpuMode, CPU};
use macroquad::{color::Color, texture::Image};

// Dots are PPU Cycle conters per Frame
const DOTS_PER_CYCLE: u32 = 4;
const DOTS_PER_LINE: u32 = 456;

const SCAN_DOTS: u32 = 80;
const MIN_DRAW_DOTS: u32 = 172;

const SCANLINES_ACTUAL: u8 = 144;
const SCANLINES_EXTRA: u8 = 10;

const TILES_PER_LINE: u16 = 21;

// Mode 2
pub fn oam_scan(_cpu: &CPU) {}

// Mode 3
pub fn draw_line(cpu: &mut CPU, game_diplay: &mut Image, palette: &[Color; 4]) {

    let scx = cpu.get_lcd_scx();
    let scy = cpu.get_lcd_scy();

    let line: u8 = cpu.get_lcd_y_coordinate();

    let mut display_x: u32 = 0;

    let mut bg_line_x_pos = scx as u32 % 8;

    let high_adressing = !cpu.get_lcdc_bg_window_tile_data();

    // Draw Background
    for xtile in 0..TILES_PER_LINE {
        // Fetch Background Line Data
        let bg_tile_idx = cpu.get_vram_tile_map_entry(
            cpu.get_lcdc_bg_tile_high_map(),
            // 32 tiles per line; 8 pixels per tile
            (((line + scy) / 8) as u16 % 0x100) * 32 + (xtile + (scx as u16 / 8)) % 32,
        );

        let bg_line = cpu.get_vram_tile_line(high_adressing, bg_tile_idx as u16, (line + scy) % 8);

        for x_pixel in (bg_line_x_pos as usize % 8)..8 {
            if display_x >= game_diplay.width() as u32 {
                break;
            }

            game_diplay.set_pixel(display_x, line as u32, palette[bg_line[x_pixel] as usize]);

            display_x += 1;
            bg_line_x_pos += 1;
        }
    }

    // Draw Window
    if cpu.get_lcdc_window_enable() {
        let wd_offset_y = cpu.get_window_wy();
        let wd_offset_x = cpu.get_window_wx() as i32 - 7;

        // Fetch Window Line Data for the 20 tiles on screen
        for xtile in 0..20 {
            // Fetch Window Line Data

            if line >= wd_offset_y {
                let wd_tile_idx = cpu.get_vram_tile_map_entry(
                    cpu.get_lcdc_window_tile_high_map(),
                    ((line - wd_offset_y) as u16 / 8) * 32 + (xtile - wd_offset_x) as u16,
                );
                let wd_line = cpu.get_vram_tile_line(high_adressing, wd_tile_idx as u16, line % 8);
    
                // Draw the 8 pixels in a tile line
                for x_pixel in 0..8 as usize {
                    let x_coord: i32 = xtile as i32 * 8 + x_pixel as i32 + wd_offset_x;
                    if x_coord >= 0 && line >= wd_offset_y {
                        game_diplay.set_pixel(
                            x_coord as u32,
                            line as u32,
                            palette[wd_line[x_pixel] as usize],
                        );
                    }
                }
            }
        }
    }

    // Draw Objects (Sprites)
    if cpu.get_lcdc_obj_enable() {
        // Draw Sprites; 40 sprites max are visible
        for sprite_idx in 0..40 {
            let sprite = cpu.get_oam_entry(sprite_idx);
    
            // Check if sprite is on the current line; Sprites are offset by 16 pixels on the Y axis; 8 pixels on the X axis
            if (line as i32) >= sprite.y_pos - 16 && (line as i32) < sprite.y_pos - 8 {
                let mut tile_line = (line + 16 - sprite.y_pos as u8) % 8;

                // Flip the tile y coordinate if the sprite is flipped
                if sprite.y_flip {
                    tile_line = 7 - tile_line;
                }
    
                let line_data = cpu.get_vram_tile_line(false, sprite.tile_idx, tile_line);
    
                // Draw the 8 pixels in a tile line
                for x_pixel_offset in 0..8 {
                    let x_pixel: i32 = (sprite.x_pos - 8) + x_pixel_offset;
    
                    if (x_pixel as usize) < game_diplay.width()
                        && x_pixel >= 0
                        && (line as usize) < game_diplay.height()
                    {
                        // Flip the tile x coordinate if the sprite is flipped
                        let pallete_idx = if sprite.x_flip {
                            7 - x_pixel_offset as usize
                        } else {
                            x_pixel_offset as usize
                        };
    
                        // Draw the pixel if color is not 0; 0 is transparent
                        if line_data[pallete_idx] != 0 {
                            game_diplay.set_pixel(
                                x_pixel as u32,
                                line as u32,
                                palette[line_data[pallete_idx] as usize],
                            );
                        }
                    }
                }
            }
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

        // Clear the screen if the PPU is disabled
        if !cpu.get_lcdc_ppu_enabled() && self.enabled{
            self.enabled = false;

            for pixel in final_image.get_image_data_mut() {
                pixel[0] = 0;
                pixel[1] = 227;
                pixel[2] = 48;
                pixel[3] = 255;
            }
            return;
        }

        // A dot is a PPU cycle; the PPU runs faster than the CPU; the emulation code will execute all the work on render mode transitions
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
                    draw_line(cpu, final_image, palette);
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
