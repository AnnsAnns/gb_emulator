use crate::{mmu::MemoryOperations};

use super::NonMbcOperations;

const BOOTROM_SIZE: usize = 0x100;

pub struct Bank00 {
    rom: [u8; 0x4000],
    boot_rom: [u8; BOOTROM_SIZE],
    pub boot_rom_enabled: bool,
}

impl Default for Bank00 {
    fn default() -> Self {
        let rom_file = include_bytes!("../../bin/DMG_ROM.bin");

        let mut boot_rom = [0; BOOTROM_SIZE];
        for (i, byte) in rom_file.iter().enumerate() {
            boot_rom[i] = *byte;
        }

        Self {
            rom: [0; 0x4000],
            boot_rom,
            boot_rom_enabled: true,
        }
    }
}

impl MemoryOperations for Bank00 {
    fn read_byte(&self, address: u16) -> u8 {
        if self.boot_rom_enabled && address < 0x100 {
            self.boot_rom[address as usize]
        } else {
            self.rom[address as usize]
        }
    }

    fn write_byte(&mut self, _address: u16, _value: u8) {
        // Do nothing as this is a ROM only cartridge
    }
}

impl NonMbcOperations for Bank00 {
    fn fill_from_slice(&mut self, data: &[u8]) {
        self.rom.copy_from_slice(data);
    }
}

impl Bank00 {
    pub fn disable_boot_rom(&mut self) {
        self.boot_rom_enabled = false;
    }
}