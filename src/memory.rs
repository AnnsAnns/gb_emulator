/// Module for memory abstraction

// These following modules do most of the abstraction work
pub mod raw_memory_operations;
pub mod io_abstraction;

/// Abstraction over the raw memory of the Gameboy
#[derive(Debug, Clone)]
pub struct Memory {
    /// The raw memory of the Gameboy
    /// The Address Bus is 16-bit, thus the memory is 64KB (0xFFFF bytes)
    /// https://gbdev.io/pandocs/Memory_Map.html#memory-map
    /// 0x0000 - 0x3FFF: 16KB ROM bank 00
    /// 0x4000 - 0x7FFF: 16KB ROM bank 01..NN
    /// 0x8000 - 0x9FFF: 8KB Video RAM (VRAM)
    /// 0xA000 - 0xBFFF: 8KB External RAM
    /// 0xC000 - 0xCFFF: 4KB Work RAM (WRAM) bank 0
    /// 0xD000 - 0xDFFF: 4KB Work RAM (WRAM) bank 1..N
    /// 0xE000 - 0xFDFF: Mirror of C000~DDFF (ECHO RAM)
    /// 0xFE00 - 0xFE9F: Object Attribute Memory (OAM)
    /// 0xFEA0 - 0xFEFF: Not Usable
    /// 0xFF00 - 0xFF7F: I/O Registers
    /// 0xFF80 - 0xFFFE: High RAM (HRAM)
    /// 0xFFFF: Interrupt Enable Register
    memory: [u8; 0xFFFF],
}

/// Implementation of the Memory
/// For further abstractions see the respective modules
impl Memory {
    /// Create a new Memory
    pub fn new() -> Memory {
        Memory {
            memory: [0; 0xFFFF],
        }
    }
}