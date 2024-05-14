use crate::cpu::{instructions::{ConditionCodes, FlagState, InstructionResult}, registers::Register8Bit,registers::Register16Bit, CPU};

impl CPU {
    pub fn inc(&mut self, register: Register8Bit) -> InstructionResult {
        let (result, overflow) = self.get_8bit_register(register).overflowing_add(1);
        self.set_8bit_register(register, result);

        InstructionResult {
            cycles: 1,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: if result == 0 { FlagState::Set } else { FlagState::Unset },
                subtract: FlagState::Unset,
                half_carry: if overflow { FlagState::Set } else { FlagState::Unset },
                carry: FlagState::NotAffected,
            },
        }
    }
    /// increments the byte pointed to by HL
    pub fn inc_hl(&mut self) -> InstructionResult {
        let addr = self.get_16bit_register(Register16Bit::HL);
        let r8_value = self.memory.read_byte(addr);
        let (value,overflow) = r8_value.overflowing_add(1);
        self.memory.write_byte(addr, value);
        let tail = r8_value & 0xF;

        InstructionResult {
            cycles: 3,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: if value == 0 {FlagState::Set} else {FlagState::NotAffected},
                subtract: FlagState::Set,
                half_carry: if tail == 0 {FlagState::Set} else {FlagState::NotAffected},
                carry: FlagState::NotAffected,
            },
        }
    }
    /// increments the 16bit_register register, wraps on overflow
    pub fn inc_r16(&mut self, register: Register16Bit) -> InstructionResult {
        let (value,overflow) = self.get_16bit_register(register).overflowing_add(1);
        self.set_16bit_register(register, value);

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