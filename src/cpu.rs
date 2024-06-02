use registers::{Register16Bit, Register8Bit};

use crate::memory::Memory;

use self::instructions::{InstructionResult, Instructions};

pub mod decode;
/// These are the actual abstractions and implementations of the CPU
mod flags;
pub mod instructions;
pub mod registers;
pub mod render_operations;
mod step;
mod interrupts;
mod joypad;
mod timer;


/// 4.194304 MHz
/// This is the frequency of the CPU
pub const CPU_FREQUENCY: u128 = 4_194_304;

/// The CPU of the Gameboy
pub struct CPU {
    /// The registers of the CPU
    /// The CPU has 8 8-bit registers (A, F, B, C, D, E, H, L)
    /// that can be combined to form 4 16-bit registers (AF, BC, DE, HL)
    /// and two purely 16-bit registers (SP, PC)
    registers: [u8; 12],
    memory: Memory,
    next_instruction: Instructions,
    last_step_result: InstructionResult,
    /// The global interrupt master enable flag
    ime_flag: bool, 
    /// 0 if nothing to do, 2 if ime needs to be set after next instruction, 1 if ime needs to be set after this instruction
    enable_ime: i32,
    last_execution_time: std::time::Instant,
    cycles: u64,
}

/// Note, please look at the relevant modules for the actual implementations
impl CPU {
    /// Create a new CPU
    pub fn new(enable_bootrom: bool) -> CPU {
        CPU {
            registers: [0; 12],
            memory: Memory::new(enable_bootrom),
            next_instruction: Instructions::NOP,
            last_step_result: InstructionResult::default(),
            enable_ime: 0,
            ime_flag: false,
            last_execution_time: std::time::Instant::now(),
            cycles: 0,
        }
    }

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

    pub fn load_from_file(&mut self, file: &str, offset: usize) {
        self.memory.load_from_file(file, offset);
    }

    pub fn get_next_opcode(&mut self) -> u8 {
        self.memory
            .read_byte(self.get_16bit_register(registers::Register16Bit::PC))
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
