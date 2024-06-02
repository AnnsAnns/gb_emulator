use crate::cpu::{instructions::{ConditionCodes, FlagState, InstructionResult}, registers::Register8Bit, CPU};

impl CPU {
    pub fn sub_and_subc(&mut self, value: u8, cycles: u8, bytes: u8, add_carry: bool) -> InstructionResult {
        let a = self.get_8bit_register(Register8Bit::A);
        let carry = if add_carry && self.is_carry_flag_set() { 1 } else { 0 };
        let result = a.wrapping_sub(value).wrapping_sub(carry);
        
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
                subtract: FlagState::Set,
                half_carry: if ((a ^ value) & 0x10) != (result & 0x10) {FlagState::Set} else {FlagState::Unset},
                carry: if value > a {FlagState::Set} else {FlagState::Unset},
            },
        }
    } 
}