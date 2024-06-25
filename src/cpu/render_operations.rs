use crate::mmu::MemoryOperations;

use super::CPU;

const LCDY_ADDRESS: u16 = 0xFF44;
const SCY_ADDRESS: u16 = 0xFF42;
const SCX_ADDRESS: u16 = 0xFF43;
const OAM_ADDRESS: u16 = 0xFE00;

const SPRITE_SIZE: u16 = 4;

pub struct Sprite {
    pub y_pos: i32,
    pub x_pos: i32,
    pub tile_idx: u16,
    pub prio_bg: bool,
    pub y_flip: bool,
    pub x_flip: bool,
    pub high_palette: bool,
}

impl CPU {
    // LCD Control getters
    pub fn get_lcdc_ppu_enabled(&self) -> bool {
        self.mmu.read_byte(0xFF40) & (1 << 7) == (1 << 7)
    }

    pub fn get_lcdc_window_tile_map_high(&self) -> bool {
        self.mmu.read_byte(0xFF40) & (1 << 6) == (1 << 6)
    }

    pub fn get_lcdc_window_enable(&self) -> bool {
        self.mmu.read_byte(0xFF40) & (1 << 5) == (1 << 5)
    }

    pub fn get_lcdc_bg_window_tile_data(&self) -> bool {
        self.mmu.read_byte(0xFF40) & (1 << 4) == (1 << 4)
    }

    pub fn get_lcdc_bg_tile_map(&self) -> bool {
        self.mmu.read_byte(0xFF40) & (1 << 3) == (1 << 3)
    }

    pub fn get_lcdc_obj_size(&self) -> bool {
        self.mmu.read_byte(0xFF40) & (1 << 2) == (1 << 2)
    }

    pub fn get_lcdc_obj_enable(&self) -> bool {
        self.mmu.read_byte(0xFF40) & (1 << 1) == (1 << 1)
    }

    pub fn get_lcdc_bg_window_enable(&self) -> bool {
        self.mmu.read_byte(0xFF40) & 1 == 1
    }

    // LCD Status getters
    pub fn get_lcd_y_coordinate(&mut self) -> u8 {
        self.mmu.read_byte(LCDY_ADDRESS)
    }

    pub fn get_lcd_scy(&mut self) -> u8 {
        self.mmu.read_byte(SCY_ADDRESS)
    }

    pub fn get_lcd_scx(&mut self) -> u8 {
        self.mmu.read_byte(SCX_ADDRESS)
    }

    // VRAM getters
    pub fn get_vram_tile_line(
        &self,
        high_addressing: bool,
        tile_index: u16,
        tile_line: u8,
    ) -> [u8; 8] {
        let mut line_data: [u8; 8] = [0; 8];

        let mut line_addr: u16 = 0x8000 + (tile_line * 2) as u16 + 16 * tile_index;

        if high_addressing && tile_index < 0x80 {
            line_addr += 0x1000;
        }

        let lo = self.mmu.read_byte(line_addr);
        let hi = self.mmu.read_byte(line_addr + 1);

        for i in 0..8 {
            line_data[7 - i] = ((lo >> i) & 1) + (((hi >> i) & 1) * 2);
        }

        line_data
    }

    pub fn get_vram_tile_map_entry(&self, high_map: bool, map_index: u16) -> u8 {
        let addr: u16 = if high_map { 0x9C00 } else { 0x9800 } + map_index;
        self.mmu.read_byte(addr)
    }

    pub fn get_oam_entry(&self, index: u8) -> Sprite {

        let entry_base_addr = OAM_ADDRESS + index as u16 * SPRITE_SIZE;

        let attribute_byte = self.mmu.read_byte(entry_base_addr + 3);

        let sprite = Sprite {
            y_pos: self.mmu.read_byte(entry_base_addr) as i32,
            x_pos: self.mmu.read_byte(entry_base_addr + 1) as i32,
            tile_idx: self.mmu.read_byte(entry_base_addr + 2) as u16,
            prio_bg: (attribute_byte >> 7) & 1 == 1,
            y_flip: (attribute_byte >> 6) & 1 == 1,
            x_flip: (attribute_byte >> 5) & 1 == 1,
            high_palette: (attribute_byte >> 4) & 1 == 1,
        };

        sprite
    }
}
