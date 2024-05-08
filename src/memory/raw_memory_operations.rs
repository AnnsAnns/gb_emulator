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
    use std::{fs, num::ParseIntError};
    use crate::memory::Memory;

    pub fn file_to_memory(memory: &mut Memory, offset: u16, file_path: &str) {
        let contents = fs::read_to_string(file_path).expect("Could not read file!");

        let memory_content = decode_hex(&contents).expect("Error while decoding hex file");

        for (pos, byte) in memory_content.iter().enumerate() {
            memory.write_byte(offset + pos as u16, *byte);
            //let wrote = memory.read_byte(offset + pos as u16);
            //println!("{:#X}", wrote);
        }
    }

    pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
        (0..s.len() - 1)
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect()
    }
}