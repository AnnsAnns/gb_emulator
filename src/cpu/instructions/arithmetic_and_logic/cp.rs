use crate::cpu::{instructions::{ConditionCodes, FlagState, InstructionResult}, registers::Register8Bit,registers::Register16Bit, CPU};
impl CPU {
    /// subtract r8 from A without storing and set flags accordingly
    pub fn cp_a_r8(&mut self, register: Register8Bit) -> InstructionResult {
        let a_value = self.get_8bit_register(Register8Bit::A);
        let _tail = a_value & 0xF;
        let r8_value = self.get_8bit_register(register);
        let (value,overflow) = a_value.overflowing_sub(r8_value);

        InstructionResult {
            cycles: 1,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: if value == 0 {FlagState::Set} else {FlagState::Unset},
                subtract: FlagState::Set,
                half_carry: if ((a_value ^ r8_value) & 0x10) != (value & 0x10) {FlagState::Set} else {FlagState::Unset},
                carry: if overflow { FlagState::Set } else { FlagState::Unset},
            },
        }
    }
    ///subtract u8 r8_value from A without storing and set flags accordingly
    pub fn cp_a_n8(&mut self, r8_value: u8) -> InstructionResult {
        let a_value = self.get_8bit_register(Register8Bit::A);
        let (value,overflow) = a_value.overflowing_sub(r8_value);

        InstructionResult {
            cycles: 2,
            bytes: 2,
            condition_codes: ConditionCodes {
                zero: if value == 0 {FlagState::Set} else {FlagState::Unset},
                subtract: FlagState::Set,
                half_carry: if ((a_value ^ r8_value) & 0x10) != (value & 0x10) {FlagState::Set} else {FlagState::Unset},
                carry: if overflow { FlagState::Set } else { FlagState::Unset},
            },
        }
    }
    /// subtract the byte in memory at addr in hl from A without storing and set flags accordingly
    pub fn cp_a_hl(&mut self) -> InstructionResult {
        let addr = self.get_16bit_register(Register16Bit::HL);
        let a_value = self.get_8bit_register(Register8Bit::A);
        let _tail = a_value & 0xF;
        let r8_value = self.memory.read_byte(addr);
        let (value,overflow) = a_value.overflowing_sub(r8_value);

        InstructionResult {
            cycles: 2,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: if value == 0 {FlagState::Set} else {FlagState::Unset},
                subtract: FlagState::Set,
                half_carry: if ((a_value ^ r8_value) & 0x10) != (value & 0x10) {FlagState::Set} else {FlagState::Unset},
                carry: if overflow { FlagState::Set } else { FlagState::Unset},
            },
        }
    }
}