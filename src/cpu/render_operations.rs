use super::CPU;

const LCDY_ADDRESS: u16 = 0xFF44;
const SCY_ADDRESS: u16 = 0xFF42;
const SCX_ADDRESS: u16 = 0xFF43;

impl CPU {
    // LCD Control getters
    pub fn get_lcdc_ppu_enabled(&self) -> bool {
        self.memory.read_byte(0xFF40) & (1 << 7) == (1 << 7)
    }

    pub fn get_lcdc_window_tile_map_high(&self) -> bool {
        self.memory.read_byte(0xFF40) & (1 << 6) == (1 << 6)
    }

    pub fn get_lcdc_window_enable(&self) -> bool {
        self.memory.read_byte(0xFF40) & (1 << 5) == (1 << 5)
    }

    pub fn get_lcdc_bg_window_tile_data(&self) -> bool {
        self.memory.read_byte(0xFF40) & (1 << 4) == (1 << 4)
    }

    pub fn get_lcdc_bg_tile_map(&self) -> bool {
        self.memory.read_byte(0xFF40) & (1 << 3) == (1 << 3)
    }

    pub fn get_lcdc_obj_size(&self) -> bool {
        self.memory.read_byte(0xFF40) & (1 << 2) == (1 << 2)
    }

    pub fn get_lcdc_obj_enable(&self) -> bool {
        self.memory.read_byte(0xFF40) & (1 << 1) == (1 << 1)
    }

    pub fn get_lcdc_bg_window_enable(&self) -> bool {
        self.memory.read_byte(0xFF40) & 1 == 1
    }

    // LCD Status getters
    pub fn get_lcd_y_coordinate(&mut self) -> u8 {
        self.memory.read_byte(LCDY_ADDRESS)
    }

    pub fn get_lcd_scy(&mut self) -> u8 {
        self.memory.read_byte(SCY_ADDRESS)
    }

    pub fn get_lcd_scx(&mut self) -> u8 {
        self.memory.read_byte(SCX_ADDRESS)
    }

    // VRAM getters
    pub fn get_vram_tile_line(
        &self,
        high_addressing: bool,
        tile_index: u16,
        tile_line: u8,
    ) -> [u8; 8] {
        let mut line_data: [u8; 8] = [0; 8];

        let mut line_addr: u16 = 0x8000 + (tile_line * 2) as u16 + 16 * tile_index as u16;

        if high_addressing && tile_index < 0x80 {
            line_addr += 0x1000;
        }

        let lo = self.memory.read_byte(line_addr);
        let hi = self.memory.read_byte(line_addr + 1);

        for i in 0..8 {
            line_data[7 - i] = ((lo >> i) & 1) + (((hi >> i) & 1) * 2);
        }

        line_data
    }

    pub fn get_vram_tile_map(&self, high_map: bool, map_index: u16) -> u8 {
        let addr: u16 = if high_map { 0x9C00 } else { 0x9800 } + map_index;
        self.memory.read_byte(addr)
    }
}
