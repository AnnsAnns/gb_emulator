/// Used to abstract I/O register reads and writes for the Frontend

use num_enum::IntoPrimitive;

use super::Memory;

/// Enum for the I/O registers
/// See: https://gbdev.io/pandocs/Hardware_Reg_List.html
#[derive(IntoPrimitive, Debug, Clone, Copy)]
#[repr(u16)]
pub enum HardwareRegisters {
    JOYP = 0xFF00, // Joypad
    SB = 0xFF01, // Serial Transfer Data
    SC = 0xFF02, // Serial Transfer Control
    DIV = 0xFF04, // Divider Register
    TIMA = 0xFF05, // Timer Counter
    TMA = 0xFF06, // Timer Modulo
    TAC = 0xFF07, // Timer Control
    IF = 0xFF0F, // Interrupt Flag
    NR10 = 0xFF10, // Sound Mode 1 Sweep
    NR11 = 0xFF11, // Sound Mode 1 Sound Length/Wave Pattern Duty
    NR12 = 0xFF12, // Sound Mode 1 Envelope
    NR13 = 0xFF13, // Sound Mode 1 Frequency Lo
    NR14 = 0xFF14, // Sound Mode 1 Frequency Hi
    NR21 = 0xFF16, // Sound Mode 2 Sound Length/Wave Pattern Duty
    NR22 = 0xFF17, // Sound Mode 2 Envelope
    NR23 = 0xFF18, // Sound Mode 2 Frequency Lo
    NR24 = 0xFF19, // Sound Mode 2 Frequency Hi
    NR30 = 0xFF1A, // Sound Mode 3 Sound On/Off
    NR31 = 0xFF1B, // Sound Mode 3 Sound Length
    NR32 = 0xFF1C, // Sound Mode 3 Select Output Level
    NR33 = 0xFF1D, // Sound Mode 3 Frequency Lo
    NR34 = 0xFF1E, // Sound Mode 3 Frequency Hi
    NR41 = 0xFF20, // Sound Mode 4 Sound Length
    NR42 = 0xFF21, // Sound Mode 4 Envelope
    NR43 = 0xFF22, // Sound Mode 4 Polynomial Counter
    NR44 = 0xFF23, // Sound Mode 4 Counter/Consecutive; Initial
    NR50 = 0xFF24, // Channel Control / On-Off / Volume
    NR51 = 0xFF25, // Selection of Sound Output Terminal
    NR52 = 0xFF26, // Sound on/off
    LCDC = 0xFF40, // LCD Control
    STAT = 0xFF41, // LCDC Status
    SCY = 0xFF42, // Viewport Y
    SCX = 0xFF43, // Viewport X
    LY = 0xFF44, // LCD Y-Coordinate
    LYC = 0xFF45, // LY Compare
    DMA = 0xFF46, // DMA Transfer and Start Address
    BGP = 0xFF47, // BG Palette Data
    OBP0 = 0xFF48, // Object Palette 0 Data
    OBP1 = 0xFF49, // Object Palette 1 Data
    WY = 0xFF4A, // Window Y Position
    WX = 0xFF4B, // Window X Position + 7
    // If we get to the Gameboy Color, implement 0xFF4D - 0xFF77
    IE = 0xFFFF, // Interrupt Enable
}

impl Memory {
    /// Abstraction for Frontend to read from I/O registers
    /// This is a convenience method to read from the I/O registers
    /// but with easy to read enum values :)
    /// Usage: memory.read_io_register(HardwareRegisters::JOYP);
    /// This will read the value from the I/O register at 0xFF00 (JOYP)
    pub fn read_io_register(&self, register: HardwareRegisters) -> u8 {
        self.read_byte(register as u16)
    }

    /// Abstraction for Frontend to write from I/O registers
    /// This is a convenience method to write from the I/O registers
    /// but with easy to read enum values :)
    /// Usage: memory.write_io_register(HardwareRegisters::JOYP, 0x3F);
    /// This will write the value 0x3F to the I/O register at 0xFF00 (JOYP)
    pub fn write_io_register(&mut self, register: HardwareRegisters, value: u8) {
        self.write_byte(register as u16, value);
    }
}