use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::{cpu::registers::Register16Bit, mmu::MemoryOperations};

use super::{CPU};

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

#[repr(u8)]
#[derive(Copy, Clone, TryFromPrimitive)]
pub enum PpuMode {
    HorizontalBlank = 0,
    VerticalBlank = 1,
    OamScan = 2,
    Drawing = 3,
}

impl CPU {
    pub fn set_vblank_interrupt(&mut self) {
        log::debug!("VBlank interrupt set");
        self.mmu.write_byte(0xFF44, 144);
        self.set_interrupt_flag(InterruptTypes::VBlank);
    }

    /// Set the interrupt flag for the given interrupt
    pub fn set_interrupt_flag(&mut self, interrupt: InterruptTypes) {
        let interrupt_flag = self.mmu.read_byte(INTERRUPT_FLAG_ADDRESS);
        self.mmu.write_byte(
            INTERRUPT_FLAG_ADDRESS,
            interrupt_flag | (1 << interrupt as u8),
        );

        log::debug!("Interrupt Flag set: {:#X}", self.mmu.read_byte(INTERRUPT_FLAG_ADDRESS));
    }

    pub fn set_lcd_y_coordinate(&mut self, value: u8) {
        //log::info!("Setting LCD Y coordinate: {}", value);
        self.mmu.write_byte(LCDY_ADDRESS, value);

        if self.is_lyc_equal_ly() {
            self.set_interrupt_flag(InterruptTypes::LCDC);

            // Set bit 2 of STAT register
            let stat = self.mmu.read_byte(STAT_ADDRESS);
            self.mmu.write_byte(STAT_ADDRESS, stat | 0b100);
        } 
    }

    /// Set the PPU mode and check if an interrupt should be triggered
    pub fn set_ppu_mode(&mut self, mode: PpuMode) {
        //log::info!("Setting PPU mode: {}", mode);

        let stat = self.mmu.read_byte(STAT_ADDRESS) & 0b1111_1100;
        self.mmu.write_byte(STAT_ADDRESS, stat | mode as u8);

        // Check if the mode 0 interrupt is enabled
        if mode as u8 == 0 && (stat & 0b1000) != 0 {
            self.set_interrupt_flag(InterruptTypes::LCDC);
        }

        // Check if the mode 1 interrupt is enabled
        if mode as u8 == 1 && (stat & 0b10000) != 0 {
            self.set_interrupt_flag(InterruptTypes::LCDC);
        }

        // Check if the mode 2 interrupt is enabled
        if mode as u8 == 2 && (stat & 0b100000) != 0 {
            self.set_interrupt_flag(InterruptTypes::LCDC);
        }
    }

    pub fn get_ppu_mode(&self) -> u8 {
        self.mmu.read_byte(STAT_ADDRESS) & 0b11
    }

    pub fn is_lyc_equal_ly(&self) -> bool {
        self.mmu.read_byte(LYC_ADDRESS) == self.mmu.read_byte(LCDY_ADDRESS)
    }

    // Checks for interrupts and returns the interrupt type
    pub fn check_interrupts(&mut self, check_for_ime: bool) -> Option<i32> {
        let interrupt_flag = self.mmu.read_byte(INTERRUPT_FLAG_ADDRESS);
        let interrupt_enable = self.mmu.read_byte(INTERRUPT_ENABLE_ADDRESS);
        let mut interrupt_type : Option<i32> = None;

        for i in 0..=4 {
            // Check if the interrupt is enabled and the IME flag is set
            let handle_interrupt = interrupt_enable & (1 << i) != 0;

            // Check if the interrupt flag is set and the interrupt is enabled
            // Also check whether we should check for the IME flag (for example, when we are in a HALT state)
            if interrupt_flag & (1 << i) != 0 && handle_interrupt && (!check_for_ime || self.ime_flag) {
                log::debug!("Interrupt Flag {:?} found", i);
                interrupt_type = Some(i);
                break;
            }
        }

        interrupt_type
    }

    pub fn handle_interrupt(&mut self, interrupt: i32) {
        // Disable all interrupts
        self.ime_flag = false;

        // Clear the interrupt flag
        let interrupt_flag = self.mmu.read_byte(INTERRUPT_FLAG_ADDRESS);
        self.mmu
            .write_byte(INTERRUPT_FLAG_ADDRESS, interrupt_flag & !(1 << interrupt));
        log::debug!("Previous flags: {:#X}, New flags: {:#X}, interrupt type: {:?}", interrupt_flag, self.mmu.read_byte(INTERRUPT_FLAG_ADDRESS), interrupt);

        // Call the interrupt handler at the appropriate address
        // https://gbdev.io/pandocs/Interrupt_Sources.html
        let interrupt_address = INTERRUPT_CALL_ADDRESS + (interrupt as u16 * 8);

        // Get current PC
        let mut current_pc = self.get_16bit_register(Register16Bit::PC);

        if self.is_halted {
            //log::info!("Interrupted while in HALT state, type: {:?}", interrupt);
            current_pc += 1;
            self.is_halted = false;
        }

        // Push PC to Stack
        self.dec_sp();
        let memory_address = self.get_16bit_register(Register16Bit::SP);
        let value1 = (current_pc >> 8) as u8;
        let value2 = current_pc as u8;
        self.mmu.write_byte(memory_address, value1);
        self.dec_sp();
        self.mmu.write_byte(memory_address-1, value2);

        // Jump to interrupt address
        self.set_16bit_register(Register16Bit::PC, interrupt_address);

        // Since the PC was changed, we need to decode the next instruction
        self.prepare_and_decode_next_instruction().unwrap();
    }

    /// Check for interrupts and handle them
    /// Returns true if an interrupt was handled
    pub fn check_and_handle_interrupts(&mut self) -> bool {
        match self.check_interrupts(true) {
            Some(interrupt) => {
                self.handle_interrupt(interrupt);
                true
            }
            None => false,
        }
    }
}