use crate::cpu::{instructions::{ConditionCodes, FlagState, InstructionResult}, registers::Register8Bit, CPU};

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
}