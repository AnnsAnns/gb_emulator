use crate::{cpu::{
    instructions::{ConditionCodes, FlagState, InstructionResult},
    registers::{Register16Bit, Register8Bit},
    CPU,
}, memory};

impl CPU {
    /// loads(copies) the value of the source 8bit-register into the target 8bit-register
    /// https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7/#LD_r8,r8
    pub fn ld_r8_r8(&mut self, target: Register8Bit, source: Register8Bit)-> InstructionResult {
        let value = self.get_8bit_register(source);
        self.set_8bit_register(target, value);

        InstructionResult {
            cycles: 1,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::NotAffected,
                half_carry: FlagState::NotAffected,
                carry: FlagState::NotAffected,
            },
        }
    }
    /// loads(copies) the 8bit-value n8 into the target 8bit-register 
    /// https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7/#LD_r8,n8
    pub fn ld_r8_n8(&mut self, target: Register8Bit, value: u8)-> InstructionResult {
        self.set_8bit_register(target, value);

        InstructionResult {
            cycles: 2,
            bytes: 2,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::NotAffected,
                half_carry: FlagState::NotAffected,
                carry: FlagState::NotAffected,
            },
        }
    }
    /// loads(copies) the 16bit-value n16 into the target 16bit-register
    /// https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7/#LD_r16,n16
    pub fn ld_r16_n16(&mut self, target: Register16Bit, value: u16)-> InstructionResult {
        self.set_16bit_register(target, value);

        InstructionResult {
            cycles: 3,
            bytes: 3,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::NotAffected,
                half_carry: FlagState::NotAffected,
                carry: FlagState::NotAffected,
            },
        }
    }
    /// loads(copies) the value of the source 8bit-register into the memory at the byte pointed to by register HL
    /// https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7/#LD__HL_,r8
    pub fn ld_hl_r8(&mut self, source: Register8Bit)-> InstructionResult {
        let value = self.get_8bit_register(source);
        let memory_address = self.get_16bit_register(Register16Bit::HL);
        self.memory.write_byte(memory_address, value);

        InstructionResult {
            cycles: 2,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::NotAffected,
                half_carry: FlagState::NotAffected,
                carry: FlagState::NotAffected,
            },
        }
    }
    /// loads(copies) the 8bit value n8 into the memory at the byte pointed to by register HL
    /// https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7/#LD__HL_,n8
    pub fn ld_hl_n8(&mut self, value: u8)-> InstructionResult {
        let memory_address = self.get_16bit_register(Register16Bit::HL);
        self.memory.write_byte(memory_address, value);

        InstructionResult {
            cycles: 3,
            bytes: 2,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::NotAffected,
                half_carry: FlagState::NotAffected,
                carry: FlagState::NotAffected,
            },
        }
    }
    /// loads(copies) the 8bit value at the byte in memory pointed to by register HL into the target 8bit-register
    /// https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7/#LD_r8,_HL_
    pub fn ld_r8_hl(&mut self, target: Register8Bit)-> InstructionResult {
        let memory_address = self.get_16bit_register(Register16Bit::HL);
        let value = self.memory.read_byte(memory_address);

        self.set_8bit_register(target, value);
        InstructionResult {
            cycles: 2,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::NotAffected,
                half_carry: FlagState::NotAffected,
                carry: FlagState::NotAffected,
            },
        }
    }
    /// loads(copies) the value in register A into memory at the address in target 16bit-register
    /// https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7/#LD__r16_,A
    pub fn ld_r16_a (&mut self, target: Register16Bit)-> InstructionResult {
        let value = self.get_8bit_register(Register8Bit::A);
        let memory_address = self.get_16bit_register(target);
        self.memory.write_byte(memory_address, value);

        InstructionResult {
            cycles: 2,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::NotAffected,
                half_carry: FlagState::NotAffected,
                carry: FlagState::NotAffected,
            },
        }
    }
    /// loads(copies) the value in register A into memory at the 16bit-address target
    /// https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7/#LD__n16_,A
    pub fn ld_n16_a (&mut self, target: u16)-> InstructionResult {
        let value = self.get_8bit_register(Register8Bit::A);
        self.memory.write_byte(target, value);

        InstructionResult {
            cycles: 4,
            bytes: 3,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::NotAffected,
                half_carry: FlagState::NotAffected,
                carry: FlagState::NotAffected,
            },
        }
    }
    /// loads(copies) the value in register A into memory at the 16bit-address target if the address is between $FF00 and $FFFF
    /// https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7/#LDH__n16_,A
    pub fn ldh_n16_a (&mut self, target: u16)-> InstructionResult {
        if target > 0xFF00u16 && target < 0xFFFFu16 {
            let value = self.get_8bit_register(Register8Bit::A);
            self.memory.write_byte(target, value);
        }

        InstructionResult {
            cycles: 3,
            bytes: 2,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::NotAffected,
                half_carry: FlagState::NotAffected,
                carry: FlagState::NotAffected,
            },
        }
    }
    /// loads(copies) the value in register A into memory at the address $FF00+C 
    /// https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7/#LDH__C_,A
    pub fn ldh_c_a (&mut self)-> InstructionResult {
        let target = 0xFF00u16 + self.get_8bit_register(Register8Bit::C) as u16;
        let value = self.get_8bit_register(Register8Bit::A);
        
        self.memory.write_byte(target, value);

        InstructionResult {
            cycles: 2,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::NotAffected,
                half_carry: FlagState::NotAffected,
                carry: FlagState::NotAffected,
            },
        }
    }
}