use num_enum::IntoPrimitive;

use super::CPU;

#[derive(Debug, IntoPrimitive, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Register8Bit {
    A = 0,
    // F = 1, // This is a special register and can only be accessed as part of AF
    B = 2,
    C = 3,
    D = 4,
    E = 5,
    H = 6,
    L = 7,
}

#[derive(Debug, IntoPrimitive, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Register16Bit {
    AF = 0,
    BC = 2,
    DE = 4,
    HL = 6,
    SP = 8,
    PC = 10,
}


impl CPU {
    /// Get an 8-bit register (e.g. A or F)
    pub fn get_8bit_register(&self, register: Register8Bit) -> u8 {
        self.registers[register as usize]
    }

    /// Get a 16-bit register (e.g. AF)
    pub fn get_16bit_register(&self, register: Register16Bit) -> u16 {
        let register_value = register as usize;
        let high = self.registers[register_value.clone()] as u16;
        let low = self.registers[register_value + 1] as u16;
        (high << 8) | low
    }

    /// Set an 8-bit register (e.g. A or F)
    pub fn set_8bit_register(&mut self, register: Register8Bit, value: u8) {
        self.registers[register as usize] = value;
    }

    /// Set a 16-bit register (e.g. AF)
    /// Note: This will set the high byte first
    pub fn set_16bit_register(&mut self, register: Register16Bit, value: u16) {
        let register_value = register as usize;
        self.registers[register_value] = (value >> 8) as u8;
        self.registers[register_value + 1] = value as u8;
    }
}