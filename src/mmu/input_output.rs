use super::{MemoryOperations, NonMbcOperations};

pub struct InputOutput {
    memory: Vec<u8>,
    offset: u16,
}

impl InputOutput {
    pub fn new(size: usize, offset: u16) -> Self {
        Self {
            memory: vec![0; size],
            offset,
        }
    }
}

impl MemoryOperations for InputOutput {
    fn read_byte(&self, address: u16) -> u8 {
        let address = address - self.offset;
        self.memory[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        let address = address - self.offset;
        self.memory[address as usize] = value;
    }
}

impl NonMbcOperations for InputOutput {
    fn fill_from_slice(&mut self, data: &[u8]) {
        self.memory.copy_from_slice(data);
    }
}