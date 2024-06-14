use crate::cpu::{
    instructions::{ConditionCodes, FlagState, InstructionResult},
    registers::{Register16Bit, Register8Bit},
    CPU,
};

#[cfg(test)]
use crate::test_helpers::{assert_correct_instruction_step};

impl CPU {
    /// check if bit 'bit' in 8bit-register target is set and set zero flag if not
    pub fn bit_u3_r8(&mut self, bit: u8, register: Register8Bit)-> InstructionResult {
        let register_to_test = self.get_8bit_register(register);
        let bit_to_test = register_to_test >> (bit);
        let is_set = (bit_to_test & 1) == 1;

        InstructionResult {
            cycles: 2,
            bytes: 2,
            condition_codes: ConditionCodes {
                zero: if !is_set {
                    FlagState::Set
                }else {
                    FlagState::Unset
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
        let bit_to_test = register_to_test >> (bit);
        let is_set = (bit_to_test & 1) == 1;

        InstructionResult {
            cycles: 3,
            bytes: 2,
            condition_codes: ConditionCodes {
                zero: if !is_set {
                    FlagState::Set
                }else {
                    FlagState::Unset
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


    /// set bit 'bit' in 8bit-register target to 1
    pub fn set_u3_r8(&mut self, bit: u8, target: Register8Bit)-> InstructionResult {
        let register_to_set = self.get_8bit_register(target);
        let mask = 1 << bit;
        let value = register_to_set | mask;
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
    /// set bit 'bit' in the byte in memory at the adress in HL to 1
    pub fn set_u3_hl(&mut self, bit: u8)-> InstructionResult {
        let memory_address = self.get_16bit_register(Register16Bit::HL);
        let register_to_set = self.memory.read_byte(memory_address);
        let mask = 1 << bit;
        let value = register_to_set | mask;
        
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

    pub fn swap_r8(&mut self, target: Register8Bit)-> InstructionResult {
        let register = self.get_8bit_register(target);
        let value = register.rotate_left(4);

        self.set_8bit_register(target, value);

        InstructionResult {
            cycles: 2,
            bytes: 2,
            condition_codes: ConditionCodes {
                zero: if value == 0 {
                    FlagState::Set
                }else {
                    FlagState::Unset
                },
                subtract: FlagState::Unset,
                half_carry: FlagState::Unset,
                carry: FlagState::Unset,
            },
        }
    }
    pub fn swap_hl(&mut self)-> InstructionResult {
        let memory_address = self.get_16bit_register(Register16Bit::HL);
        let byte = self.memory.read_byte(memory_address);
        let value = byte.rotate_left(4);

        self.memory.write_byte(memory_address, value);

        InstructionResult {
            cycles: 4,
            bytes: 2,
            condition_codes: ConditionCodes {
                zero: if value == 0 {
                    FlagState::Set
                }else {
                    FlagState::Unset
                },
                subtract: FlagState::Unset,
                half_carry: FlagState::Unset,
                carry: FlagState::Unset,
            },
        }
    }
}

#[test]
pub fn bit_op_test() {
    let mut cpu = CPU::new(false);
    let mut expected_result = InstructionResult::default();
    let mut registers;

    // 1) BIT
    cpu.ld_r8_n8(Register8Bit::A, 0b11111101u8);
    cpu.ld_hl_r8(Register8Bit::A);
    // bit is 0
    expected_result.cycles = 2;
    expected_result.bytes = 2;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Set,subtract:FlagState::Unset,half_carry:FlagState::Set,carry:FlagState::NotAffected};
    assert_correct_instruction_step(&mut cpu, Instructions::BIT(super::InstParam::Unsigned3Bit(1), super::InstParam::Register8Bit(Register8Bit::A)), expected_result);
    //bit is not 0
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 2;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::NotAffected,subtract:FlagState::Unset,half_carry:FlagState::Set,carry:FlagState::NotAffected};
    assert_correct_instruction_step(&mut cpu, Instructions::BIT(super::InstParam::Unsigned3Bit(7), super::InstParam::Register8Bit(Register8Bit::A)), expected_result);
    //bit is 0 with HL
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 3;
    expected_result.bytes = 2;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Set,subtract:FlagState::Unset,half_carry:FlagState::Set,carry:FlagState::NotAffected};
    assert_correct_instruction_step(&mut cpu, Instructions::BIT(super::InstParam::Unsigned3Bit(1), super::InstParam::Register16Bit(Register16Bit::HL)), expected_result);

    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 0b11111101u8);
    // 2) RES
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 2;
    assert_correct_instruction_step(&mut cpu, Instructions::RES(super::InstParam::Unsigned3Bit(0), super::InstParam::Register8Bit(Register8Bit::A)), expected_result);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 2;
    assert_correct_instruction_step(&mut cpu, Instructions::RES(super::InstParam::Unsigned3Bit(7), super::InstParam::Register8Bit(Register8Bit::A)), expected_result);
    
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 4;
    expected_result.bytes = 2;
    assert_correct_instruction_step(&mut cpu, Instructions::RES(super::InstParam::Unsigned3Bit(0), super::InstParam::Register16Bit(Register16Bit::HL)), expected_result);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 4;
    expected_result.bytes = 2;
    assert_correct_instruction_step(&mut cpu, Instructions::RES(super::InstParam::Unsigned3Bit(7), super::InstParam::Register16Bit(Register16Bit::HL)), expected_result);
    
    cpu.ld_r8_hl(Register8Bit::B);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 0b01111100u8);
    assert_eq!(registers[Register8Bit::B as usize], 0b01111100u8);

    //3) SET
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 2;
    assert_correct_instruction_step(&mut cpu, Instructions::SET(super::InstParam::Unsigned3Bit(0), super::InstParam::Register8Bit(Register8Bit::A)), expected_result);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 2;
    assert_correct_instruction_step(&mut cpu, Instructions::SET(super::InstParam::Unsigned3Bit(7), super::InstParam::Register8Bit(Register8Bit::A)), expected_result);
    
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 4;
    expected_result.bytes = 2;
    assert_correct_instruction_step(&mut cpu, Instructions::SET(super::InstParam::Unsigned3Bit(0), super::InstParam::Register16Bit(Register16Bit::HL)), expected_result);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 4;
    expected_result.bytes = 2;
    assert_correct_instruction_step(&mut cpu, Instructions::SET(super::InstParam::Unsigned3Bit(7), super::InstParam::Register16Bit(Register16Bit::HL)), expected_result);
    
    cpu.ld_r8_hl(Register8Bit::B);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 0b11111101u8);
    assert_eq!(registers[Register8Bit::B as usize], 0b11111101u8);
    // 4) SWAP
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 2;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::NotAffected,subtract:FlagState::Unset,half_carry:FlagState::Unset,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::SWAP( super::InstParam::Register8Bit(Register8Bit::A)), expected_result);

    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 4;
    expected_result.bytes = 2;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::NotAffected,subtract:FlagState::Unset,half_carry:FlagState::Unset,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::SWAP( super::InstParam::Register16Bit(Register16Bit::HL)), expected_result);

    cpu.ld_r8_hl(Register8Bit::B);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 0b11011111u8);
    assert_eq!(registers[Register8Bit::B as usize], 0b11011111u8);

    //0 as result
    cpu.ld_hl_n8(0);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 4;
    expected_result.bytes = 2;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Unset,half_carry:FlagState::Unset,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::SWAP( super::InstParam::Register16Bit(Register16Bit::HL)), expected_result);
    
    cpu.ld_r8_hl(Register8Bit::B);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::B as usize], 0b00000000u8);
}

