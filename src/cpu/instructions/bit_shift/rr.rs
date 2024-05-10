use crate::cpu::{
    instructions::{ConditionCodes, FlagState, InstructionResult},
    registers::{Register16Bit, Register8Bit},
    CPU,
};

///Bitmask to controll if a 1 or 0 is shifted into carrybit
const SHIFT_INTO_CARRY_R: u8 = 0b0000_0001;

impl CPU {
    fn rr_u8(
        &mut self,
        value: u8,
        throug_carry: bool,
        set_zero: bool,
    ) -> (ConditionCodes, u8) {
        let new_carry = value & SHIFT_INTO_CARRY_R;
        let shift_into_result = if throug_carry && self.is_carry_flag_set()
            || !throug_carry && new_carry != 0
        {
            128
        } else {
            0
        };
        let result = (value >> 1) | shift_into_result;
        (
            ConditionCodes {
                zero: if result != 0 && set_zero {
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

    fn rr_x_r8(
        &mut self,
        target: Register8Bit,
        through_carry: bool,
        set_zero: bool,
    ) -> ConditionCodes {
        let value = self.get_8bit_register(target);
        let (condition_codes_result, result) = self.rr_u8(value, through_carry, set_zero);
        self.set_8bit_register(target, result);
        condition_codes_result
    }

    fn rr_x_hl(
        &mut self,
        target: Register16Bit,
        through_carry: bool,
        set_zero: bool,
    ) -> ConditionCodes {
        let mem_addr = self.get_16bit_register(target);
        let value = self.memory.read_byte(mem_addr);
        let (condition_codes_result, result) = self.rr_u8(value, through_carry, set_zero);
        self.memory.write_byte(mem_addr, result);
        condition_codes_result
    }

    pub fn rr_r8(&mut self, target: Register8Bit) -> InstructionResult {
        let through_carry = true;
        let set_zero = true;
        InstructionResult {
            cycles: 2,
            bytes: 2,
            condition_codes: self.rr_x_r8(target, through_carry, set_zero),
        }
    }

    pub fn rr_hl(&mut self, target: Register16Bit) -> InstructionResult {
        let through_carry = true;
        let set_zero = true;
        InstructionResult {
            cycles: 4,
            bytes: 2,
            condition_codes: self.rr_x_hl(target, through_carry, set_zero),
        }
    }

    pub fn rr_a(&mut self) -> InstructionResult {
        let through_carry = true;
        let set_zero = false;
        InstructionResult {
            cycles: 1,
            bytes: 1,
            condition_codes: self.rr_x_r8(Register8Bit::A, through_carry, set_zero),
        }
    }

    pub fn rr_c_r8(&mut self, target: Register8Bit) -> InstructionResult {
        let through_carry = false;
        let set_zero = true;
        InstructionResult {
            cycles: 2,
            bytes: 2,
            condition_codes: self.rr_x_r8(target, through_carry, set_zero),
        }
    }

    pub fn rr_c_hl(&mut self, target: Register16Bit) -> InstructionResult {
        let through_carry = false;
        let set_zero = true;
        InstructionResult {
            cycles: 4,
            bytes: 2,
            condition_codes: self.rr_x_hl(target, through_carry, set_zero),
        }
    }

    pub fn rr_c_a(&mut self) -> InstructionResult {
        let through_carry = false;
        let set_zero = false;
        InstructionResult {
            cycles: 1,
            bytes: 1,
            condition_codes: self.rr_x_r8(Register8Bit::A, through_carry, set_zero),
        }
    }
}

#[test]
pub fn rr_test() {
    let mut cpu = CPU::new();

    //Test rl_r8
    cpu.clear_carry_flag();
    cpu.set_8bit_register(Register8Bit::B, 1);

    let mut  instruction_result = cpu.rr_r8(Register8Bit::B);
    assert_eq!(cpu.get_8bit_register(Register8Bit::B), 0);
    assert!(instruction_result.condition_codes.carry == FlagState::Set);
    cpu.set_carry_flag();

    instruction_result = cpu.rr_r8(Register8Bit::B);
    assert_eq!(cpu.get_8bit_register(Register8Bit::B), 128);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
    cpu.clear_carry_flag();

    instruction_result = cpu.rr_r8(Register8Bit::B);
    assert_eq!(cpu.get_8bit_register(Register8Bit::B), 64);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);

    //Test rl_a
    cpu.clear_carry_flag();
    cpu.set_8bit_register(Register8Bit::A, 1);

    instruction_result = cpu.rr_a();
    assert_eq!(cpu.get_8bit_register(Register8Bit::A), 0);
    assert!(instruction_result.condition_codes.carry == FlagState::Set);
    cpu.set_carry_flag();

    instruction_result = cpu.rr_r8(Register8Bit::A);
    assert_eq!(cpu.get_8bit_register(Register8Bit::A), 128);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
    cpu.clear_carry_flag();

    instruction_result = cpu.rr_r8(Register8Bit::A);
    assert_eq!(cpu.get_8bit_register(Register8Bit::A), 64);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);

    //Test rl_hl
    cpu.clear_carry_flag();
    let mem_addr = 0b0000_0000_1000_0000;
    cpu.set_16bit_register(Register16Bit::BC, mem_addr);
    cpu.memory.write_byte(mem_addr, 1);

    instruction_result = cpu.rr_hl(Register16Bit::BC);
    assert_eq!(cpu.memory.read_byte(mem_addr), 0);
    assert!(instruction_result.condition_codes.carry == FlagState::Set);
    cpu.set_carry_flag();

    instruction_result = cpu.rr_hl(Register16Bit::BC);
    assert_eq!(cpu.memory.read_byte(mem_addr), 128);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
    cpu.clear_carry_flag();

    instruction_result = cpu.rr_hl(Register16Bit::BC);
    assert_eq!(cpu.memory.read_byte(mem_addr), 64);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);

    //Test rl_c_r8
    cpu.clear_carry_flag();
    cpu.set_8bit_register(Register8Bit::B, 1);

    instruction_result = cpu.rr_c_r8(Register8Bit::B);
    assert_eq!(cpu.get_8bit_register(Register8Bit::B), 128);
    assert!(instruction_result.condition_codes.carry == FlagState::Set);
    cpu.set_carry_flag();

    instruction_result = cpu.rr_c_r8(Register8Bit::B);
    assert_eq!(cpu.get_8bit_register(Register8Bit::B), 64);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
    cpu.clear_carry_flag();

    instruction_result = cpu.rr_c_r8(Register8Bit::B);
    assert_eq!(cpu.get_8bit_register(Register8Bit::B), 32);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);

    //Test rl_c_a
    cpu.clear_carry_flag();
    cpu.set_8bit_register(Register8Bit::A, 1);

    instruction_result = cpu.rr_c_a();
    assert_eq!(cpu.get_8bit_register(Register8Bit::A), 128);
    assert!(instruction_result.condition_codes.carry == FlagState::Set);
    cpu.set_carry_flag();

    instruction_result = cpu.rr_c_r8(Register8Bit::A);
    assert_eq!(cpu.get_8bit_register(Register8Bit::A), 64);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
    cpu.clear_carry_flag();

    instruction_result = cpu.rr_c_r8(Register8Bit::A);
    assert_eq!(cpu.get_8bit_register(Register8Bit::A), 32);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);

    //Test rl_c_hl
    cpu.clear_carry_flag();
    let mem_addr = 0b0000_0000_1000_0000;
    cpu.set_16bit_register(Register16Bit::BC, mem_addr);
    cpu.memory.write_byte(mem_addr, 1);

    instruction_result = cpu.rr_c_hl(Register16Bit::BC);
    assert_eq!(cpu.memory.read_byte(mem_addr), 128);
    assert!(instruction_result.condition_codes.carry == FlagState::Set);
    cpu.set_carry_flag();

    instruction_result = cpu.rr_c_hl(Register16Bit::BC);
    assert_eq!(cpu.memory.read_byte(mem_addr), 64);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
    cpu.clear_carry_flag();

    instruction_result = cpu.rr_c_hl(Register16Bit::BC);
    assert_eq!(cpu.memory.read_byte(mem_addr), 32);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
}
