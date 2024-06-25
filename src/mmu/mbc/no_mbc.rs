use crate::mmu::{MemoryBankControllerOperations, MemoryOperations};

pub struct NoMbc {
    rom: [u8; 0x4000],
    ram: [u8; 0x2000],
}

impl Default for NoMbc {
    fn default() -> Self {
        Self {
            rom: [0; 0x4000],
            ram: [0; 0x2000],
        }
    }
}

impl MemoryOperations for NoMbc {
    fn read_byte(&self, address: u16) -> u8 {
        self.rom[self.calc_physical_rom_address(address)]
    }

    fn write_byte(&mut self, _address: u16, _value: u8) {
        // Do nothing as this is a ROM only cartridge
    }
}

impl MemoryBankControllerOperations for NoMbc { 
    fn init(&mut self, _rom_size: u8, cartridge_type: u8, ram_size: u8) {
        assert_eq!(ram_size, 0);
        assert_eq!(cartridge_type, 0);
    }
    
    fn fill_rom_bank_from_slice(&mut self, bank: u8, data: &[u8; 0x4000]) {
        assert_eq!(bank, 1, "Only bank 0 & 1 is available in ROM only cartridge");
        self.rom.copy_from_slice(data);
    }
    
    fn fill_ram_bank_from_slice(&mut self, bank: u8, data: &[u8; 0x2000]) {
        assert_eq!(bank, 0, "Only bank 0 is available in ROM only cartridge");
        self.ram.copy_from_slice(data);
    }
    
    fn switch_rom_bank(&mut self, _bank: u8) {
        panic!("No ROM bank switching in ROM only cartridge")
    }
    
    fn switch_ram_bank(&mut self, _bank: u8) {
        panic!("No RAM bank switching in ROM only cartridge")
    }
    
    fn enable_ram(&mut self, enable: bool) {
        assert!(!enable, "RAM can not be enabled in ROM only cartridge");
    }
}