use crate::memory::Memory;

/// These are the actual abstractions and implementations of the CPU
mod flags;
mod registers;

/// The CPU of the Gameboy
pub struct CPU {
    /// The registers of the CPU
    /// The CPU has 8 8-bit registers (A, F, B, C, D, E, H, L)
    /// that can be combined to form 4 16-bit registers (AF, BC, DE, HL)
    /// and two purely 16-bit registers (SP, PC)
    registers: [u8; 8],
    memory: Memory,
}

/// Note, please look at the relevant modules for the actual implementations
impl CPU {
    /// Create a new CPU
    pub fn new() -> CPU {
        CPU {
            registers: [0; 8],
            memory: Memory::new(),
        }
    }
}