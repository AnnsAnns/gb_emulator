use crate::mmu::{MemoryBankControllerOperations, MemoryOperations};

pub struct Mbc1 {
    rom: Vec<[u8; 0x4000]>,
    ram: Vec<[u8; 0x2000]>,
    rom_bank_number: u8,
    ram_bank_number: u8,
    ram_enabled: bool,
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8,
    advanced_banking_mode: bool,
}

impl Default for Mbc1 {
    fn default() -> Self {
        let rom = [0; 0x4000];
        let ram = [0; 0x2000];

        Self {
            rom: vec![rom],
            ram: vec![ram],
            rom_bank_number: 1,
            ram_bank_number: 0,
            ram_enabled: false,
            cartridge_type: 0,
            rom_size: 0,
            ram_size: 0,
            advanced_banking_mode: false,
        }
    }
}

impl MemoryOperations for Mbc1 {
    fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x3FFF => {
                let current_rambank = match self.rom.get(0) {
                    Some(bank) => bank,
                    None => panic!("ROM bank 0 not found"),
                };
        
                current_rambank[address as usize]
            },
            0x4000..=0x7FFF => {
                let current_rambank = match self.rom.get(self.rom_bank_number as usize) {
                    Some(bank) => bank,
                    None => panic!("ROM bank {} not found", self.rom_bank_number),
                };
        
                current_rambank[self.calc_physical_rom_address(address)]
            },
            0xA000..=0xBFFF => {
                let current_rambank = match self.ram.get(self.ram_bank_number as usize) {
                    Some(bank) => bank,
                    None => panic!("RAM bank {} not found", self.rom_bank_number),
                };
        
                current_rambank[self.calc_physical_ram_address(address)]
            },
            _ => panic!("Invalid address: {:#06X}", address),
        }
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            // https://gbdev.io/pandocs/MBC1.html#00001fff--ram-enable-write-only
            0x0000..=0x1FFF => {
                self.ram_enabled = value & 0x0F == 0x0A; // Check if the lower 4 bits are 0x0A
            },
            // https://gbdev.io/pandocs/MBC1.html#20003fff--rom-bank-number-write-only
            0x2000..=0x3FFF => {
                let bank = value & 0b0001_1111; // Discard the upper 3 bits
                let bank = if bank == 0 { 1 } else { bank }; // Bank 0 is not accessible
                self.switch_rom_bank(bank);
            }, 
            // https://gbdev.io/pandocs/MBC1.html#40005fff--ram-bank-number--or--upper-bits-of-rom-bank-number-write-only
            0x4000..=0x5FFF => {
                let bank = value & 0b0000_0011; // Discard the upper 5 bits (Only 2 bits are used)
                if self.ram_size >= 3 { // If the RAM size is 32KB
                    self.switch_ram_bank(bank);
                } else if self.rom_size >= 5 { // If the ROM size is 1MB or more
                    let rom_bank_number = self.rom_bank_number & 0b0001_1111; // Discard the upper 3 bits
                    self.switch_rom_bank((bank << 5) | rom_bank_number); // Combine the upper 2 bits with the lower 5 bits
                }
            },
            0xA000..=0xBFFF => {
                log::debug!("Writing to RAM address: {:#06X}", address);
                let address = self.calc_physical_ram_address(address);

                if self.ram_enabled {
                    let current_rambank = match self.ram.get_mut(self.ram_bank_number as usize) {
                        Some(bank) => bank,
                        None => panic!("RAM bank {} not found", self.rom_bank_number),
                    };
                    current_rambank[address] = value;
                }
            },
            0x6000..=0x7FFF => {
                let bit = value & 0x01;
                log::debug!("Writing to ROM/RAM mode select register: {:#06X}", bit);
                if bit == 0 {
                    self.ram_bank_number = 0;
                }
                self.advanced_banking_mode = bit == 1;
            },
            _ => panic!("Invalid address: {:#06X}", address),
        }
    }
}

impl MemoryBankControllerOperations for Mbc1 { 
    fn init(&mut self, rom_size: u8, cartridge_type: u8, ram_size: u8) {
        self.rom_size = rom_size;
        self.cartridge_type = cartridge_type;
        self.ram_size = ram_size;
    }
    
    fn fill_rom_bank_from_slice(&mut self, bank: u8, data: &[u8; 0x4000]) {
        // Make sure all banks before the current bank are filled
        for _ in self.rom.len()..bank as usize {
            self.rom.push([0; 0x4000]);
        }
        
        // Check whether the bank already exists
        if let Some(rom) = self.rom.get_mut(bank as usize) {
            rom.copy_from_slice(data);
        } else {
            self.rom.push(*data);
        }
    }
    
    fn fill_ram_bank_from_slice(&mut self, bank: u8, data: &[u8; 0x2000]) {
        // Make sure all banks before the current bank are filled
        for _ in self.ram.len()..bank as usize {
            self.ram.push([0; 0x2000]);
        }
        
        // Check whether the bank already exists
        if let Some(ram) = self.ram.get_mut(bank as usize) {
            ram.copy_from_slice(data);
        } else {
            self.ram.push(*data);
        }
    }
    
    fn switch_rom_bank(&mut self, bank: u8) {
        self.ram_bank_number = bank;
    }
    
    fn switch_ram_bank(&mut self, bank: u8) {
        self.ram_bank_number = bank;
    }
    
    fn enable_ram(&mut self, enable: bool) {
        self.ram_enabled = enable;
    }
    
    fn is_advanced_banking_mode(&self) -> bool {
        self.advanced_banking_mode
    }
}