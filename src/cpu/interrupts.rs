use num_enum::IntoPrimitive;

use super::{instructions::InstructionResult, CPU};

#[derive(Debug, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum InterruptTypes {
    VBlank = 0,
    LCDC = 1,
    Timer = 2,
    Serial = 3,
    Joypad = 4,
}

const INTERRUPT_FLAG_ADDRESS: u16 = 0xFF0F;
const INTERRUPT_ENABLE_ADDRESS: u16 = 0xFFFF;
const INTERRUPT_CALL_ADDRESS: u16 = 0x0040;
const LCDY_ADDRESS: u16 = 0xFF44;
const LYC_ADDRESS: u16 = 0xFF45;
const STAT_ADDRESS: u16 = 0xFF41;

impl CPU {
    pub fn set_vblank_interrupt(&mut self) {
        log::debug!("VBlank interrupt set");
        self.memory.write_byte(0xFF44, 144);
        self.set_interrupt_flag(InterruptTypes::VBlank);
    }

    /// Set the interrupt flag for the given interrupt
    pub fn set_interrupt_flag(&mut self, interrupt: InterruptTypes) {
        let interrupt_flag = self.memory.read_byte(INTERRUPT_FLAG_ADDRESS);
        self.memory.write_byte(
            INTERRUPT_FLAG_ADDRESS,
            interrupt_flag | (1 << interrupt as u8),
        );
    }

    pub fn set_lcd_y_coordinate(&mut self, value: u8) {
        //log::info!("Setting LCD Y coordinate: {}", value);
        self.memory.write_byte(LCDY_ADDRESS, value);

        if self.is_lyc_equal_ly() {
            self.set_interrupt_flag(InterruptTypes::LCDC);

            // Set bit 2 of STAT register
            let stat = self.memory.read_byte(STAT_ADDRESS);
            self.memory.write_byte(STAT_ADDRESS, stat | 0b100);
        } 
    }

    /// Set the PPU mode and check if an interrupt should be triggered
    pub fn set_ppu_mode(&mut self, mode: u8) {
        //log::info!("Setting PPU mode: {}", mode);

        let stat = self.memory.read_byte(STAT_ADDRESS);
        self.memory.write_byte(STAT_ADDRESS, stat | mode);

        // Check if the mode 0 interrupt is enabled
        if mode == 0 && (stat & 0b1000) != 0 {
            self.set_interrupt_flag(InterruptTypes::LCDC);
        }

        // Check if the mode 1 interrupt is enabled
        if mode == 1 && (stat & 0b10000) != 0 {
            self.set_interrupt_flag(InterruptTypes::LCDC);
        }

        // Check if the mode 2 interrupt is enabled
        if mode == 2 && (stat & 0b100000) != 0 {
            self.set_interrupt_flag(InterruptTypes::LCDC);
        }
    }

    pub fn get_ppu_mode(&self) -> u8 {
        self.memory.read_byte(STAT_ADDRESS) & 0b11
    }

    pub fn is_lyc_equal_ly(&self) -> bool {
        self.memory.read_byte(LYC_ADDRESS) == self.memory.read_byte(LCDY_ADDRESS)
    }

    /// Check for interrupts and handle them
    /// Returns true if an interrupt was handled
    pub fn check_and_handle_interrupts(&mut self) -> bool {
        let interrupt_flag = self.memory.read_byte(INTERRUPT_FLAG_ADDRESS);
        let interrupt_enable = self.memory.read_byte(INTERRUPT_ENABLE_ADDRESS);
        let mut was_interrupt_called: bool = false;

        for i in 0..=4 {
            // Check if the interrupt flag is set and the interrupt is enabled
            if interrupt_flag & (1 << i) != 0 && interrupt_enable & (1 << i) != 0 && self.ime_flag {
                // Disable all interrupts
                self.ime_flag = false;

                // Clear the interrupt flag
                let interrupt_flag = self.memory.read_byte(INTERRUPT_FLAG_ADDRESS);
                self.memory
                    .write_byte(INTERRUPT_FLAG_ADDRESS, interrupt_flag & !(1 << i));

                // Call the interrupt handler at the appropriate address
                // https://gbdev.io/pandocs/Interrupt_Sources.html
                self.call_n16(INTERRUPT_CALL_ADDRESS + (i as u16 * 8));

                was_interrupt_called = true;
            }
        }
        was_interrupt_called
    }
}