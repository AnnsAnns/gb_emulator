use std::io::Write;

use super::{Memory, MEMORY_SIZE};

impl Memory {
    
    pub fn is_boot_rom_enabled(&self) -> bool {
        self.boot_rom_enabled
    }

    /// This is used for testing purposes
    /// @warning This is really expensive and should only be used for testing
    pub fn return_full_memory(&self) -> [u8; MEMORY_SIZE] {
        self.memory
    }

    pub fn load_from_file(&mut self, file_path: &str, offset: usize) {
        let rom = std::fs::read(file_path).expect("Unable to read file");

        for (i, byte) in rom.iter().enumerate() {
            self.memory[i + offset] = *byte;
        }
    }

    /// Creates a new thread to dump the memory to a file (non-blocking)
    pub fn dump_to_file(&self) {
        let memory = self.clone();

        std::thread::spawn(move || {
            let mut file = std::fs::File::create("memory_dump.bin").expect("Unable to create file");

            for byte in 0..MEMORY_SIZE {
                file.write_all(memory.read_byte(byte as u16).to_le_bytes().as_ref()).expect("Unable to write to file");
            }
        });
    }
}