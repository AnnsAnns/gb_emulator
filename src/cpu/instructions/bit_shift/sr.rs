
use crate::cpu::{
    instructions::{ConditionCodes, FlagState, InstructionResult},
    registers::{Register16Bit, Register8Bit},
    CPU,
};

impl CPU{
    fn sr_u8(
        &mut self,
        value: u8,
        shift_arithmetic: bool,
    ) -> (ConditionCodes, u8) {
        let shift_into_carry_l: u8 = 0b0000_0001;
        let shift_into_result_l: u8 = 0b1000_0000;
        let new_carry = value & shift_into_carry_l;
        let shift_into_result = if shift_arithmetic 
            && (value &shift_into_result_l) != 0 {
            128
        }
        else{
            0
        };
        let mut result = value >> 1;
        result |= shift_into_result;
        (
            ConditionCodes {
                zero: if result != 0 {
                    FlagState::Unset
                } else {
                    FlagState::Set
                },
                subtract: FlagState::Unset,
                half_carry: FlagState::Unset,
                carry: if new_carry != 0 {
                    FlagState::Set
                } else {
                    FlagState::Unset
                },
            },
            result,
        )
    }

    pub fn sra_r8(
        &mut self,
        target: Register8Bit,
    ) -> InstructionResult {
        let value = self.get_8bit_register(target);
        let shift_arithmetic = true;
        let (condition_codes_result, result) = self.sr_u8(value, shift_arithmetic);
        self.set_8bit_register(target, result);
        InstructionResult{
            cycles: 2,
            bytes: 2,
            condition_codes: condition_codes_result,
        }
    }

    pub fn sra_hl(
        &mut self,
    ) -> InstructionResult {
        let mem_addr = self.get_16bit_register(Register16Bit::HL);
        let value = self.memory.read_byte(mem_addr);
        let shift_arithmetic = true;
        let (condition_codes_result, result) = self.sr_u8(value, shift_arithmetic);
        self.memory.write_byte(mem_addr, result);
        InstructionResult{
            cycles: 4,
            bytes: 2,
            condition_codes: condition_codes_result,
        }
    }

    pub fn srl_r8(
        &mut self,
        target: Register8Bit,
    ) -> InstructionResult {
        let value = self.get_8bit_register(target);
        let shift_arithmetic = false;
        let (condition_codes_result, result) = self.sr_u8(value, shift_arithmetic);
        self.set_8bit_register(target, result);
        InstructionResult{
            cycles: 2,
            bytes: 2,
            condition_codes: condition_codes_result,
        }
    }

    pub fn srl_hl(
        &mut self,
    ) -> InstructionResult {
        let mem_addr = self.get_16bit_register(Register16Bit::HL);
        let value = self.memory.read_byte(mem_addr);
        let shift_arithmetic = false;
        let (condition_codes_result, result) = self.sr_u8(value, shift_arithmetic);
        self.memory.write_byte(mem_addr, result);
        InstructionResult{
            cycles: 4,
            bytes: 2,
            condition_codes: condition_codes_result,
        }
    }

}

#[test]
pub fn sr_test() {
    let mut cpu = CPU::new(false);

    //Test sra_r8
    cpu.clear_carry_flag();
    cpu.set_8bit_register(Register8Bit::B, 129);

    let mut instruction_result = cpu.sra_r8(Register8Bit::B);
    assert_eq!(cpu.get_8bit_register(Register8Bit::B), 192);
    assert!(instruction_result.condition_codes.carry == FlagState::Set);
    cpu.set_carry_flag();

    instruction_result = cpu.sra_r8(Register8Bit::B);
    assert_eq!(cpu.get_8bit_register(Register8Bit::B), 224);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
    cpu.clear_carry_flag();

    //Test sra_hl
    let mem_addr = 0b0000_0000_1000_0000;
    cpu.set_16bit_register(Register16Bit::HL, mem_addr);
    cpu.memory.write_byte(mem_addr, 129);

    instruction_result = cpu.sra_hl();
    assert_eq!(cpu.memory.read_byte(mem_addr), 192);
    assert!(instruction_result.condition_codes.carry == FlagState::Set);
    cpu.set_carry_flag();

    instruction_result = cpu.sra_hl();
    assert_eq!(cpu.memory.read_byte(mem_addr), 224);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
    cpu.clear_carry_flag();

    //Test srl_r8
    cpu.set_8bit_register(Register8Bit::B, 129);

    let mut instruction_result = cpu.srl_r8(Register8Bit::B);
    assert_eq!(cpu.get_8bit_register(Register8Bit::B), 64);
    assert!(instruction_result.condition_codes.carry == FlagState::Set);
    cpu.set_carry_flag();

    instruction_result = cpu.srl_r8(Register8Bit::B);
    assert_eq!(cpu.get_8bit_register(Register8Bit::B), 32);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
    cpu.clear_carry_flag();

    //Test srl_hl
    let mem_addr = 0b0000_0000_1000_0000;
    cpu.set_16bit_register(Register16Bit::HL, mem_addr);
    cpu.memory.write_byte(mem_addr, 129);

    instruction_result = cpu.srl_hl();
    assert_eq!(cpu.memory.read_byte(mem_addr), 64);
    assert!(instruction_result.condition_codes.carry == FlagState::Set);
    cpu.set_carry_flag();

    instruction_result = cpu.srl_hl();
    assert_eq!(cpu.memory.read_byte(mem_addr), 32);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
    cpu.clear_carry_flag();
}