use memory_bank_controller::ROM;

pub mod raw_memory_operations;
mod helpers;
mod memory_bank_controller;

const MEMORY_SIZE: usize = 0xFFFF - 0xE000;
const ROM_SIZE: usize = 256;
const MBC_INFO: usize = 0x147;

pub trait MemoryBankController {
    fn new(memory: Vec<u8>) -> Self;
    fn read_byte(&self, address: u16) -> u8;
    fn read_word(&self, address: u16) -> u16;
    fn write_byte(&mut self, address: u16, value: u8);
}

/// Abstraction over the raw memory of the Gameboy
#[derive(Debug, Clone)]
pub struct Memory<T: MemoryBankController> {
    /// The raw memory of the Gameboy
    /// The Address Bus is 16-bit, thus the memory is 64KB (0xFFFF bytes)
    /// https://gbdev.io/pandocs/Memory_Map.html#memory-map
    /// 
    /// < Cartridge (Memory Bank Controlled) >
    /// 
    /// 0x0000 - 0x3FFF: 16KB ROM bank 00
    /// 0x4000 - 0x7FFF: 16KB ROM bank 01..NN
    /// 0x8000 - 0x9FFF: 8KB Video RAM (VRAM)
    /// 0xA000 - 0xBFFF: 8KB External RAM
    /// 0xC000 - 0xCFFF: 4KB Work RAM (WRAM) bank 0
    /// 0xD000 - 0xDFFF: 4KB Work RAM (WRAM) bank 1..N
    /// 
    /// < Internal / Generic for all MBCs >
    /// 
    /// 0xE000 - 0xFDFF: Mirror of C000~DDFF (ECHO RAM)
    /// 0xFE00 - 0xFE9F: Object Attribute Memory (OAM)
    /// 0xFEA0 - 0xFEFF: Not Usable
    /// 0xFF00 - 0xFF7F: I/O Registers
    /// 0xFF80 - 0xFFFE: High RAM (HRAM)
    /// 0xFFFF: Interrupt Enable Register
    cartridge_memory: T,
    internal_memory: [u8; MEMORY_SIZE],
    dma_requested: bool,
    pub direction_buttons: u8,
    pub action_buttons: u8,
}

/// Implementation of the Memory
/// For further abstractions see the respective modules
impl <T> Memory<T> where T: MemoryBankController {
    /// Create a new Memory
    pub fn new(rom: Vec<u8>) -> Memory<T> {
        let rom_file = include_bytes!("../bin/DMG_ROM.bin");

        let mut boot_rom = [0; ROM_SIZE];
        for (i, byte) in rom_file.iter().enumerate() {
            boot_rom[i] = *byte;
        }

        // Check which type of MBC the cartridge uses
        let mbc = rom.get(MBC_INFO);
        let mbc = match mbc {
            Some(mbc) => *mbc,
            None => panic!("Invalid ROM file, missing MBC info at 0x147"),
        };

        let cartridge_memory = match mbc {
            0x00 => {
                ROM::new(rom)
            },
            0x01..=0x03 => {
                todo!("Implement MBC1");
            },
            _ => panic!("Unsupported MBC: {mbc}"),
        }
        
        Memory {
            cartridge_memory,
            internal_memory: [0; MEMORY_SIZE],
            boot_rom_enabled: enable_bootrom,
            boot_rom,
            dma_requested: false,
            direction_buttons: 0b1111,
            action_buttons: 0b1111,
        }
    }
}