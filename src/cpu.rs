use crate::{mmu::MMU};
use self::instructions::{InstructionResult, Instructions};

pub mod decode;
/// These are the actual abstractions and implementations of the CPU
mod flags;
pub mod instructions;
pub mod registers;
pub mod render_operations;
mod step;
pub mod interrupts;
pub mod joypad;
mod timer;
mod dma;
mod helpers;

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
    pub mmu: MMU,
    next_instruction: Instructions,
    last_step_result: InstructionResult,
    /// The global interrupt master enable flag
    ime_flag: bool, 
    /// 0 if nothing to do, 2 if ime needs to be set after next instruction, 1 if ime needs to be set after this instruction
    enable_ime: i32,
    last_execution_time: std::time::Instant,
    cycles: u64,
    is_halted: bool,
    stop_mode: bool,
    pub instruction: i32,
    dma_active: bool, // Whether a DMA has been requested
    dma_current_offset: u8, // The current line offset based on the DMA register being copied
}

/// Note, please look at the relevant modules for the actual implementations
impl CPU {
    /// Create a new CPU
    pub fn new(rom: Vec<u8>) -> CPU {
        CPU {
            registers: [0; 12],
            next_instruction: Instructions::NOP,
            last_step_result: InstructionResult::default(),
            enable_ime: 0,
            ime_flag: false,
            mmu: MMU::new_from_vec(rom),
            last_execution_time: std::time::Instant::now(),
            cycles: 0,
            is_halted: false,
            stop_mode: false,
            instruction: 0,
            dma_active: false,
            dma_current_offset: 0,
        }
    }
}
