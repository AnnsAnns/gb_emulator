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
}