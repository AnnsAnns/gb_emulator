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

    /// Check for interrupts and handle them
    /// Returns true if an interrupt was handled
    pub fn check_and_handle_interrupts(&mut self) -> bool {
        let interrupt_flag = self.memory.read_byte(INTERRUPT_FLAG_ADDRESS);
        let interrupt_enable = self.memory.read_byte(INTERRUPT_ENABLE_ADDRESS);

        if interrupt_flag == 0 || interrupt_enable == 0 {
            return false;
        }

        for i in 0..=4 {
            // Check if the interrupt flag is set and the interrupt is enabled
            if interrupt_flag & (1 << i) != 0 && interrupt_enable & (1 << i) != 0 {
                // Disable all interrupts
                self.memory.write_byte(INTERRUPT_ENABLE_ADDRESS, 0);

                // Clear the interrupt flag
                let interrupt_flag = self.memory.read_byte(INTERRUPT_FLAG_ADDRESS);
                self.memory
                    .write_byte(INTERRUPT_FLAG_ADDRESS, interrupt_flag & !(1 << i));

                // Call the interrupt handler at the appropriate address
                // https://gbdev.io/pandocs/Interrupt_Sources.html
                self.call_n16(INTERRUPT_CALL_ADDRESS + (i as u16 * 8));

                // We return early, interrupts are based on a priority system
                // and if one interrupt is handled, we don't want to handle another
                return true;
            }
        }
        return false;
    }
}