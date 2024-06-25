use crate::memory::MemoryBankController;

struct RomOnly {
    rom: Vec<u8>,
}

impl MemoryBankController for RomOnly {
    fn read_byte(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn write_byte(&mut self, _address: u16, _value: u8) {
        todo!()
    }
    
    fn read_word(&self, address: u16) -> u16 {
        todo!()
    }
    
    fn new(memory: Vec<u8>) -> Self {
        todo!()
    }
}