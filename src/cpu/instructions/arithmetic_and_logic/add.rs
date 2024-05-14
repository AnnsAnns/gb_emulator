use crate::cpu::{
    instructions::{ConditionCodes, FlagState, InstructionResult},
    registers::{Register16Bit, Register8Bit},
    CPU,
};

impl CPU {
    /// Abstraction for the ADD instruction used by other instructions
    fn add_8bit(
        &mut self,
        value: u8,
        target: Register8Bit,
        also_add_carry: bool,
    ) -> ConditionCodes {
        let target_value = self.get_8bit_register(target);
        let carry = if also_add_carry && self.is_carry_flag_set() {
            1
        } else {
            0
        };
        let (result, overflow) = target_value.overflowing_add(value + carry);

        self.set_8bit_register(target, result);

        ConditionCodes {
            zero: if result == 0 {
                FlagState::Set
            } else {
                FlagState::Unset
            },
            subtract: FlagState::Unset,
            half_carry: if (target_value & 0xF) + (value & 0xF) + carry > 0xF {
                FlagState::Set
            } else {
                FlagState::Unset
            },
            carry: if overflow {
                FlagState::Set
            } else {
                FlagState::Unset
            },
        }
    }

    /// Abstraction for ADC & ADD
    fn adx_a_r8(&mut self, target: Register8Bit, also_add_carry: bool) -> InstructionResult {
        let value = self.get_8bit_register(target);

        InstructionResult {
            cycles: 1,
            bytes: 1,
            condition_codes: self.add_8bit(value, Register8Bit::A, also_add_carry),
        }
    }

    fn adx_a_hl(&mut self, also_add_carry: bool) -> InstructionResult {
        let mem_addr = self.get_16bit_register(Register16Bit::HL);
        let value = self.memory.read_byte(mem_addr);

        InstructionResult {
            cycles: 2,
            bytes: 1,
            condition_codes: self.add_8bit(value, Register8Bit::A, also_add_carry),
        }
    }

    /// Add the value of an 8-bit register to the A register + carry flag
    /// https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7/#ADC_A,r8
    pub fn adc_a_r8(&mut self, target: Register8Bit) -> InstructionResult {
        self.adx_a_r8(target, true)
    }

    /// Add the value of an 8-bit register to the A register
    /// https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7/#ADD_A,r8
    pub fn add_a_r8(&mut self, target: Register8Bit) -> InstructionResult {
        self.adx_a_r8(target, false)
    }

    /// Add the value of a memory address within HL to the A register + carry flag
    /// https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7/#ADC_A,_HL_
    pub fn adc_a_hl(&mut self) -> InstructionResult {
        self.adx_a_hl(true)
    }

    /// Add the value of a memory address within HL to the A register
    /// https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7/#ADD_A,_HL_
    pub fn add_a_hl(&mut self) -> InstructionResult {
        self.adx_a_hl(false)
    }

    /// Add an 8-bit value to the A register
    /// https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7/#ADD_A,n8
    pub fn add_a_n8(&mut self, value: u8) -> InstructionResult {
        InstructionResult {
            cycles: 2,
            bytes: 2,
            condition_codes: self.add_8bit(value, Register8Bit::A, false),
        }
    }
    /// Add an 8-bit value plus the carry flag to the A register
    /// https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7/#ADC_A,n8
    pub fn adc_a_n8(&mut self, value: u8) -> InstructionResult {
        InstructionResult {
            cycles: 2,
            bytes: 2,
            condition_codes: self.add_8bit(value, Register8Bit::A, true),
        }
    }

    /// add the value of a 16-bit register to HL
    pub fn add_hl_r16(&mut self, source: Register16Bit) -> InstructionResult {
        let hl = self.get_16bit_register(Register16Bit::HL);
        let source_value = self.get_16bit_register(source);
        let (result, overflow) = hl.overflowing_add(source_value);

        self.set_16bit_register(Register16Bit::HL, result);

        InstructionResult {
            cycles: 2,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::Unset,
                half_carry: if (hl & 0xFFF) + (source_value & 0xFFF) > 0xFFF {
                    FlagState::Set
                } else {
                    FlagState::Unset
                },
                carry: if overflow {
                    FlagState::Set
                } else {
                    FlagState::Unset
                },
            },
        }
    }

    /// Add the value of SP to HL
    /// (No clue why this has its own instruction, but it does)
    pub fn add_hl_sp(&mut self) -> InstructionResult {
        self.add_hl_r16(Register16Bit::SP)
    }

    /// Add a signed 8-bit value to the SP register
    /// https://rgbds.gbdev.io/docs/v0.6.1/gbz80.7/#ADD_SP,r8
    pub fn add_sp_e8(&mut self, value: i8) -> InstructionResult {
        let sp = self.get_16bit_register(Register16Bit::SP);
        let (result, overflow) = sp.overflowing_add(value as u16);

        self.set_16bit_register(Register16Bit::SP, result);

        InstructionResult {
            cycles: 4,
            bytes: 2,
            condition_codes: ConditionCodes {
                zero: FlagState::Unset,
                subtract: FlagState::Unset,
                half_carry: if (sp & 0xF) + (value as u16 & 0xF) > 0xF {
                    FlagState::Set
                } else {
                    FlagState::Unset
                },
                carry: if overflow {
                    FlagState::Set
                } else {
                    FlagState::Unset
                },
            },
        }
    }
}

