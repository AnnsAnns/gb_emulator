use crate::cpu::{instructions::{ConditionCodes, FlagState, InstructionResult}, registers::Register8Bit,registers::Register16Bit, CPU};
impl CPU {
    /// decrements the 16bit_register register, wraps on overflow
    pub fn dec_r16(&mut self, register: Register16Bit) -> InstructionResult {
        let (value,overflow) = self.get_16bit_register(register).overflowing_sub(1);
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
    /// decrements the 8bit_register
    pub fn dec_r8(&mut self, register: Register8Bit) -> InstructionResult {
        let r8_value = self.get_8bit_register(register);
        let (value,overflow) = r8_value.overflowing_sub(1);
        let tail = r8_value & 0xF;
        self.set_8bit_register(register, value);


        InstructionResult {
            cycles: 1,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: if value == 0 {FlagState::Set} else {FlagState::Unset},
                subtract: FlagState::Set,
                half_carry: if tail == 0 {FlagState::Set} else {FlagState::Unset},
                carry: FlagState::NotAffected,
            },
        }
    }
    /// decrements the byte pointed to by HL
    pub fn dec_hl(&mut self) -> InstructionResult {
        let addr = self.get_16bit_register(Register16Bit::HL);
        let r8_value = self.memory.read_byte(addr);
        let (value,overflow) = r8_value.overflowing_sub(1);
        self.memory.write_byte(addr, value);
        let tail = r8_value & 0xF;

        InstructionResult {
            cycles: 3,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: if value == 0 {FlagState::Set} else {FlagState::Unset},
                subtract: FlagState::Set,
                half_carry: if tail == 0 {FlagState::Set} else {FlagState::Unset},
                carry: FlagState::NotAffected,
            },
        }
    }
}