/// Used by other modules to read and write to memory
use super::Memory;

impl Memory {
    /// Read a byte from memory
    pub fn read_byte(&self, address: u16) -> u8 {
        if self.boot_rom_enabled && address < 0xFF {
            self.boot_rom[address as usize]
        } else {
            self.memory[address as usize]
        }
    }

    /// Write a byte to memory
    /// Usage: memory.write_byte(0xFF00, 0x3F);
    /// This will write the value 0x3F to the I/O register at 0xFF00 (JOYP)
    pub fn write_byte(&mut self, address: u16, value: u8) {
        // Special case for disabling the boot rom
        if address == 0xFF50 {
            self.boot_rom_enabled = false;
        }

        self.memory[address as usize] = value;
    }

    /// Read a word from memory
    /// Used to read 16-bit values from memory
    /// The GB is little-endian, so the low byte is first
    /// Usage: memory.read_word(0xFF00);
    /// This will read the 16-bit value from 0xFF00 and 0xFF01
    pub fn read_word(&self, address: u16) -> u16 {
        let low_byte = self.read_byte(address) as u16;
        let high_byte = self.read_byte(address + 1) as u16;
        (high_byte << 8) | low_byte
    }
}