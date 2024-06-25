use super::{MemoryOperations, NonMbcOperations};

pub struct SimpleRegion {
    memory: Vec<u8>,
    writeable: bool,
}

impl SimpleRegion {
    pub fn new(size: usize, writeable: bool) -> Self {
        Self {
            memory: vec![0; size],
            writeable,
        }
    }
}

impl MemoryOperations for SimpleRegion {
    fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        if address > self.memory.len() as u16 {
            panic!("Address out of bounds: {:#06X}", address);
        }

        if self.writeable {
            self.memory[address as usize] = value;
        } else {
            log::warn!("Attempted to write to read-only memory at address: {:#06X}", address)
        }
    }
}

impl NonMbcOperations for SimpleRegion {
    fn fill_from_slice(&mut self, data: &[u8]) {
        self.memory.copy_from_slice(data);
    }
}