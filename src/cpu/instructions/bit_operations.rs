use crate::cpu::{
    instructions::{ConditionCodes, FlagState, InstructionResult,Instructions,InstParam},
    registers::{Register16Bit, Register8Bit},
    CPU,
};


impl CPU {
    /// check if bit 'bit' in 8bit-register target is set and set zero flag if not
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
    ///check if bit in the byte in memory at the adress in HL is set and set zero flag if not
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
    /// set bit 'bit' in 8bit-register target to 0
    pub fn res_u3_r8(&mut self, bit: u8, target: Register8Bit)-> InstructionResult {
        let register_to_set = self.get_8bit_register(target);
        let mask = !(1 << bit);
        let value = register_to_set & mask;
        self.set_8bit_register(target, value);

        InstructionResult {
            cycles: 2,
            bytes: 2,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::NotAffected,
                half_carry: FlagState::NotAffected,
                carry: FlagState::NotAffected,
            },
        }
    }
    /// set bit 'bit' in the byte in memory at the adress in HL to 0
    pub fn res_u3_hl(&mut self, bit: u8)-> InstructionResult {
        let memory_address = self.get_16bit_register(Register16Bit::HL);
        let register_to_set = self.memory.read_byte(memory_address);
        let mask = !(1 << bit);
        let value = register_to_set & mask;
        
        self.memory.write_byte(memory_address, value);

        InstructionResult {
            cycles: 4,
            bytes: 2,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::NotAffected,
                half_carry: FlagState::NotAffected,
                carry: FlagState::NotAffected,
            },
        }
    }
}