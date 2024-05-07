use crate::memory::Memory;

use self::instructions::{InstructionResult, Instructions};



/// These are the actual abstractions and implementations of the CPU
mod flags;
mod registers;
pub mod instructions;
pub mod decode;
mod step;

/// The CPU of the Gameboy
pub struct CPU {
    /// The registers of the CPU
    /// The CPU has 8 8-bit registers (A, F, B, C, D, E, H, L)
    /// that can be combined to form 4 16-bit registers (AF, BC, DE, HL)
    /// and two purely 16-bit registers (SP, PC)
    registers: [u8; 8],
    memory: Memory,
    next_instruction: Instructions,
    last_step_result: InstructionResult,
}

/// Note, please look at the relevant modules for the actual implementations
impl CPU {
    /// Create a new CPU
    pub fn new() -> CPU {
        CPU {
            registers: [0; 8],
            memory: Memory::new(),
            next_instruction: Instructions::NOP,
            last_step_result: InstructionResult::default(),
        }
    }

    /// Set the next instruction to be executed
    /// This is used for testing 
    #[cfg(test)]
    pub fn set_instruction(&mut self, instruction: Instructions) {
        self.next_instruction = instruction;
    }

    /// Get the last step result
    /// This is used for testing purposes
    #[cfg(test)]
    pub fn get_last_step_result(&self) -> InstructionResult {
        self.last_step_result.clone()
    }

    #[cfg(test)]
    pub fn get_registry_dump(&self) -> [u8; 8] {
        self.registers.clone()
    }

    #[cfg(test)]
    pub fn get_memory(&self) -> Memory {
        self.memory.clone()
    }
}