use crate::cpu::{
    instructions::{ConditionCodes, FlagState, InstructionResult},
    registers::{Register16Bit, Register8Bit},
    CPU,
};

impl CPU {
    fn sl_u8(&mut self, value: u8) -> (ConditionCodes, u8) {
        let shift_into_carry_l: u8 = 0b1000_0000;
        let new_carry = value & shift_into_carry_l;
        let result = value << 1;
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

    pub fn sla_r8(&mut self, target: Register8Bit) -> InstructionResult {
        let value = self.get_8bit_register(target);
        let (condition_codes_result, result) = self.sl_u8(value);
        self.set_8bit_register(target, result);
        InstructionResult {
            cycles: 2,
            bytes: 2,
            condition_codes: condition_codes_result,
        }
    }

    pub fn sla_hl(&mut self) -> InstructionResult {
        let mem_addr = self.get_16bit_register(Register16Bit::HL);
        let value = self.memory.read_byte(mem_addr);
        let (condition_codes_result, result) = self.sl_u8(value);
        self.memory.write_byte(mem_addr, result);
        InstructionResult {
            cycles: 4,
            bytes: 2,
            condition_codes: condition_codes_result,
        }
    }
}

#[test]
pub fn sl_test() {
    let mut cpu = CPU::new();

    //Test sra_r8
    cpu.clear_carry_flag();
    cpu.set_8bit_register(Register8Bit::B, 64);

    let mut instruction_result = cpu.sla_r8(Register8Bit::B);
    assert_eq!(cpu.get_8bit_register(Register8Bit::B), 128);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
    cpu.set_carry_flag();

    instruction_result = cpu.sla_r8(Register8Bit::B);
    assert_eq!(cpu.get_8bit_register(Register8Bit::B), 0);
    assert!(instruction_result.condition_codes.carry == FlagState::Set);
    cpu.clear_carry_flag();

    //Test sra_hl
    let mem_addr = 0b0000_0000_1000_0000;
    cpu.set_16bit_register(Register16Bit::HL, mem_addr);
    cpu.memory.write_byte(mem_addr, 64);

    instruction_result = cpu.sla_hl();
    assert_eq!(cpu.memory.read_byte(mem_addr), 128);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
    cpu.set_carry_flag();

    instruction_result = cpu.sla_hl();
    assert_eq!(cpu.memory.read_byte(mem_addr), 0);
    assert!(instruction_result.condition_codes.carry == FlagState::Set);
    cpu.clear_carry_flag();
}
