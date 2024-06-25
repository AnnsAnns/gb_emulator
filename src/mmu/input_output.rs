use super::{MemoryOperations, NonMbcOperations};

const DIV_REGISTER: u16 = 0xFF04;
const JOYPAD_REGISTER: u16 = 0xFF00;
const OAM_DMA_REGISTER: u16 = 0xFF46;

pub struct InputOutput {
    memory: Vec<u8>,
    offset: u16,
    pub action_buttons: u8,
    pub direction_buttons: u8,
    pub dma_requested: bool,
}

impl InputOutput {
    pub fn new(size: usize, offset: u16) -> Self {
        Self {
            memory: vec![0; size],
            offset,
            action_buttons: 0xF,
            direction_buttons: 0xF,
            dma_requested: false,
        }
    }

    pub fn write_div_register(&mut self, value: u8) {
        let addr = self.calc_physical_address(DIV_REGISTER);
        self.memory[addr] = value;
    }

    pub fn write_controller_byte(&mut self, value: u8) {
        let addr = self.calc_physical_address(JOYPAD_REGISTER);
        self.memory[addr] = value;
    }

    fn calc_physical_address(&self, address: u16) -> usize {
        (address - self.offset) as usize
    }

    pub fn is_dma_requested(&self) -> bool {
        self.dma_requested
    }

    pub fn reset_dma_request(&mut self) {
        self.dma_requested = false;
    }
}

impl MemoryOperations for InputOutput {
    fn read_byte(&self, address: u16) -> u8 {
        let address = self.calc_physical_address(address);
        self.memory[address as usize]
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        let physical_address = self.calc_physical_address(address);
        match address {
            DIV_REGISTER => self.memory[physical_address as usize] = 0,
            JOYPAD_REGISTER => {
                let mut buttons: u8 = 0xF;

                let value  = value & 0x30;

                if value == 0x30 { //bit 5 = action buttons
                    buttons |= value;
                }else if value == 0x10 { //bit 5 = action buttons
                    buttons = value | self.action_buttons;
                }else if  value == 0x20 { //bit 4 = direction buttons
                    buttons = value | self.direction_buttons;
                }
                self.memory[physical_address as usize] =  buttons;
            }
            OAM_DMA_REGISTER => {
                self.dma_requested = true;
                self.memory[physical_address as usize] = value
            },
            _ => self.memory[physical_address as usize] = value,
        }
    }
}

impl NonMbcOperations for InputOutput {
    fn fill_from_slice(&mut self, data: &[u8]) {
        self.memory.copy_from_slice(data);
    }
}