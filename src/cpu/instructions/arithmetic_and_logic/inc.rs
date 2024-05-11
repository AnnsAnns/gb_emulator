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
    /// incements the 16bit_register register
    pub fn inc_r16(&mut self, register: Register16Bit) -> InstructionResult {
        let value = self.get_16bit_register(register)+1;
        self.set_16bit_register(register, value);

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
}