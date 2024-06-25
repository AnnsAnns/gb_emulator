use crate::memory::MemoryBankController;

struct MBC1 {
    memory: Vec<u8>,
    mode: u8,
}

impl MemoryBankController for MBC1 {
    fn read_byte(&self, address: u16) -> u8 {
        todo!()
    }

    fn read_word(&self, address: u16) -> u16 {
        todo!()
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        todo!()
    }
    
    fn new(memory: Vec<u8>) -> Self {
        todo!()
    }
}