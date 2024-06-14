use crate::memory::Memory;

use super::{instructions::{InstructionResult, Instructions}, registers::{Register16Bit, Register8Bit}, CPU};



impl CPU {
    /// Skip the bootrom
    /// Set the registers to the correct values
    /// Used for: https://robertheaton.com/gameboy-doctor/
    pub fn skip_boot_rom(&mut self) {
        self.set_8bit_register(Register8Bit::A, 0x01);
        self.set_zero_flag();
        self.set_half_carry_flag();
        self.set_carry_flag();
        self.set_8bit_register(Register8Bit::B, 0x00);
        self.set_8bit_register(Register8Bit::C, 0x13);
        self.set_8bit_register(Register8Bit::D, 0x00);
        self.set_8bit_register(Register8Bit::E, 0xD8);
        self.set_8bit_register(Register8Bit::H, 0x01);
        self.set_8bit_register(Register8Bit::L, 0x4D);
        self.set_16bit_register(Register16Bit::SP, 0xFFFE);
        self.set_16bit_register(Register16Bit::PC, 0x0100);
        self.memory.boot_rom_enabled = false;
        // Set Joypad register
        self.memory.write_byte(0xFF00, 0b1111_1111);
    }

    /// Polls the inputs
    /// Warning, this will loop till input is received when self.stop_mode is true
    pub fn poll_inputs(&mut self) {
        loop { 
            self.update_key_input();

            if !self.is_in_stop_mode() {
                break;
            }
        }
    }

    pub fn is_in_stop_mode(&self) -> bool {
        self.stop_mode
    }

    // Print blarg serial output
    pub fn blarg_print(&mut self) {
        let serial_data = self.memory.read_byte(0xFF02);
        if serial_data == 0x81 {
            let data = self.memory.read_byte(0xFF01);
            print!("{}", data as char);
            self.memory.write_byte(0xFF02, 0x0);
        }
    }

    pub fn load_from_file(&mut self, file: &str, offset: usize) {
        self.memory.load_from_file(file, offset);
    }

    pub fn get_next_opcode(&mut self) -> u8 {
        self.memory
            .read_byte(self.get_16bit_register(Register16Bit::PC))
    }

    pub fn write_memory(&mut self, address: u16, value: u8) {
        self.memory.write_byte(address, value);
    }

    /// Set the next instruction to be executed
    /// This is used for testing
    pub fn set_instruction(&mut self, instruction: Instructions) {
        self.next_instruction = instruction;
    }

    pub fn dump_memory(&self) {
        self.memory.dump_to_file();
    }

    /// Get the last step result
    /// This is used for testing purposes
    pub fn get_last_step_result(&self) -> InstructionResult {
        self.last_step_result.clone()
    }

    pub fn get_cycles(&self) -> u64 {
        self.cycles
    }

    pub fn is_boot_rom_enabled(&self) -> bool {
        self.memory.is_boot_rom_enabled().clone()
    }

    pub fn get_instruction(&self) -> &Instructions {
        &self.next_instruction
    }

    #[cfg(test)]
    pub fn get_registry_dump(&self) -> [u8; 12] {
        self.registers.clone()
    }

    /// Gets the full memory of the CPU
    /// This is used for testing purposes
    /// @warning This is a very expensive operation
    pub fn get_memory(&self) -> Memory {
        self.memory.clone()
    }
}