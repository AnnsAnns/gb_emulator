use crate::cpu::{instructions::{ConditionCodes, FlagState, InstructionResult}, registers::Register8Bit, CPU};

impl CPU {
    pub fn xor(&mut self, value: u8, cycles: u8, bytes: u8) -> InstructionResult {
        let a = self.get_8bit_register(Register8Bit::A);
        let result = a ^ value;

        self.set_8bit_register(Register8Bit::A, result);

        InstructionResult {
            cycles,
            bytes,
            condition_codes: ConditionCodes {
                zero: {
                    if result == 0 {
                        FlagState::Set
                    } else {
                        FlagState::Unset
                    }
                },
                subtract: FlagState::Unset,
                half_carry: FlagState::Unset,
                carry: FlagState::Unset,
            },
        }
    }
}