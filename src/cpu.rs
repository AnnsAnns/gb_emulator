use crate::memory::Memory;

use self::instructions::{InstructionResult, Instructions};

pub mod decode;
/// These are the actual abstractions and implementations of the CPU
mod flags;
pub mod instructions;
pub mod registers;
mod step;
mod joypad;

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
    interrupt_master_enable: bool,
    /// 0 if nothing to do, 2 if ime needs to be set abfer next instruction, 1 if ime needs to be set after this instruction
    enable_ime: i32,
    low_power_mode: bool,
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
            interrupt_master_enable: false,
            enable_ime: 0,
            low_power_mode: false,
        }
    }

    pub fn load_from_file(&mut self, file: &str) {
        self.memory.load_from_file(file);
    }

    pub fn get_next_opcode(&mut self) -> u8 {
        self.memory
            .read_byte(self.get_16bit_register(registers::Register16Bit::PC))
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
