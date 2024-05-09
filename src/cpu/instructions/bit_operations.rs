use crate::cpu::{
    instructions::{ConditionCodes, FlagState, InstructionResult,Instructions,InstParam},
    registers::{Register16Bit, Register8Bit},
    CPU,
};


impl CPU {
    pub fn bit_u3_r8(&mut self, bit: u8, register: Register8Bit)-> InstructionResult {
        let register_to_test = self.get_8bit_register(register);
        let bit_to_test = register_to_test >> (bit-1);
        let is_set = (bit_to_test & 1) == 1;

        InstructionResult {
            cycles: 2,
            bytes: 2,
            condition_codes: ConditionCodes {
                zero: if !is_set {
                    FlagState::Set
                }else {
                    FlagState::NotAffected
                },
                subtract: FlagState::Unset,
                half_carry: FlagState::Set,
                carry: FlagState::NotAffected,
            },
        }
    }
    pub fn bit_u3_hl(&mut self, bit: u8)-> InstructionResult {
        let memory_address = self.get_16bit_register(Register16Bit::HL);
        let register_to_test = self.memory.read_byte(memory_address);
        let bit_to_test = register_to_test >> (bit-1);
        let is_set = (bit_to_test & 1) == 1;

        InstructionResult {
            cycles: 3,
            bytes: 2,
            condition_codes: ConditionCodes {
                zero: if !is_set {
                    FlagState::Set
                }else {
                    FlagState::NotAffected
                },
                subtract: FlagState::Unset,
                half_carry: FlagState::Set,
                carry: FlagState::NotAffected,
            },
        }
    }
}