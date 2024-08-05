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
        let physical_address = self.calc_physical_rom_address(address);

        if self.rom.len() <= physical_address {
            panic!("ROM bank not found");
        }

        self.rom[physical_address]
    }

    fn write_byte(&mut self, _address: u16, _value: u8) {
        // Do nothing as this is a ROM only cartridge
    }
}

impl MemoryBankControllerOperations for NoMbc { 
    fn init(&mut self, _rom_size: u8, cartridge_type: u8, ram_size: u8) {
        assert_eq!(cartridge_type, 0);
    }
    
    fn fill_rom_bank_from_slice(&mut self, bank: u8, data: &[u8; 0x4000]) {
        self.rom.copy_from_slice(data);
    }
    
    fn fill_ram_bank_from_slice(&mut self, bank: u8, data: &[u8; 0x2000]) {
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
    
    fn is_advanced_banking_mode(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_mbc_initialization() {
        let mut no_mbc = NoMbc::default();
        no_mbc.init(0, 0, 0); // Ensure that the cartridge type is 0 (ROM_ONLY)
    }

    #[test]
    #[should_panic(expected = "No ROM bank switching in ROM only cartridge")]
    fn test_no_rom_bank_switching() {
        let mut no_mbc = NoMbc::default();
        no_mbc.switch_rom_bank(1); // Should panic
    }

    #[test]
    #[should_panic(expected = "No RAM bank switching in ROM only cartridge")]
    fn test_no_ram_bank_switching() {
        let mut no_mbc = NoMbc::default();
        no_mbc.switch_ram_bank(1); // Should panic
    }

    #[test]
    fn test_ram_enable_disable() {
        let mut no_mbc = NoMbc::default();
        no_mbc.enable_ram(false); // Should pass
    }

    #[test]
    #[should_panic(expected = "RAM can not be enabled in ROM only cartridge")]
    fn test_ram_enable_should_panic() {
        let mut no_mbc = NoMbc::default();
        no_mbc.enable_ram(true); // Should panic
    }
}