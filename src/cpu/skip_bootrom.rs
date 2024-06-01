use super::{registers::{Register16Bit, Register8Bit}, CPU};

impl CPU {
    /// Tries to recreate the state of the CPU after the bootrom
    /// See: https://gbdev.io/pandocs/Power_Up_Sequence.html#cpu-registers
    pub fn skip_bootrom(&mut self) {
        self.set_8bit_register(Register8Bit::A, 0x01);
        self.clear_carry_flag();
        self.clear_half_carry_flag();
        self.clear_subtraction_flag();
        self.clear_zero_flag();
        self.set_8bit_register(Register8Bit::B, 0x00);
        self.set_8bit_register(Register8Bit::C, 0x13);
        self.set_8bit_register(Register8Bit::D, 0x00);
        self.set_8bit_register(Register8Bit::E, 0xC1);
        self.set_8bit_register(Register8Bit::H, 0x84);
        self.set_8bit_register(Register8Bit::L, 0x03);
        self.set_16bit_register(Register16Bit::SP, 0xFFFE);
        self.set_16bit_register(Register16Bit::PC, 0x0100);


        // Set the initial values of the memory
        self.memory.write_byte(0xFF00, 0xCF);
        self.memory.write_byte(0xFF01, 0x00);
        self.memory.write_byte(0xFF02, 0x7E);
        self.memory.write_byte(0xFF04, 0x18);
        self.memory.write_byte(0xFF05, 0x00);
        self.memory.write_byte(0xFF06, 0x00);
        self.memory.write_byte(0xFF07, 0xF8);
        self.memory.write_byte(0xFF0F, 0xE1);
        self.memory.write_byte(0xFF10, 0x80);
        self.memory.write_byte(0xFF11, 0xBF);
        self.memory.write_byte(0xFF12, 0xF3);
        self.memory.write_byte(0xFF14, 0xBF);
        self.memory.write_byte(0xFF16, 0x3F);
        self.memory.write_byte(0xFF17, 0x00);
        self.memory.write_byte(0xFF18, 0xFF);
        self.memory.write_byte(0xFF19, 0xBF);
        self.memory.write_byte(0xFF1A, 0x7F);
        self.memory.write_byte(0xFF1B, 0xFF);
        self.memory.write_byte(0xFF1C, 0x9F);
        self.memory.write_byte(0xFF1E, 0xBF);
        self.memory.write_byte(0xFF20, 0xFF);
        self.memory.write_byte(0xFF21, 0x00);
        self.memory.write_byte(0xFF22, 0x00);
        self.memory.write_byte(0xFF23, 0xBF);
        self.memory.write_byte(0xFF24, 0x77);
        self.memory.write_byte(0xFF25, 0xF3);
        self.memory.write_byte(0xFF26, 0xF1);
        self.memory.write_byte(0xFF40, 0x91);
        self.memory.write_byte(0xFF42, 0x00);
        self.memory.write_byte(0xFF43, 0x00);
        self.memory.write_byte(0xFF45, 0x00);
        self.memory.write_byte(0xFF47, 0xFC);
        self.memory.write_byte(0xFF48, 0xFF);
        self.memory.write_byte(0xFF49, 0xFF);
        self.memory.write_byte(0xFF4A, 0x00);
        self.memory.write_byte(0xFF4B, 0x00);
        self.memory.write_byte(0xFFFF, 0x00);
    }
}