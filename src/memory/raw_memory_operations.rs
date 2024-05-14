/// Used by other modules to read and write to memory
use super::Memory;

impl Memory {
    /// Read a byte from memory
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    /// Write a byte to memory
    /// Usage: memory.write_byte(0xFF00, 0x3F);
    /// This will write the value 0x3F to the I/O register at 0xFF00 (JOYP)
    pub fn write_byte(&mut self, address: u16, value: u8) {
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

#[cfg(test)]
pub mod test_helper {

    use std::fs;

    use crate::memory::Memory;

    pub fn file_to_memory(memory: &mut Memory, offset: u16, file_path: &str) {
        let data: Vec<u8> = fs::read(file_path).expect("Could not read file");

        for (pos, byte) in data.iter().enumerate() {
            memory.write_byte(offset + pos as u16, *byte);
            //let wrote = memory.read_byte(offset + pos as u16);
            //println!("{:#X}", wrote);
        }
    }
}