use crate::{cpu::{
    instructions::{ConditionCodes, FlagState, InstructionResult},
    registers::{Register16Bit, Register8Bit},
    CPU,
}, mmu::MemoryOperations};

impl CPU {
    fn rl_u8(
        &mut self,
        value: u8,
        throug_carry: bool,
        set_zero: bool,
    ) -> (ConditionCodes, u8) {
        let shift_into_carry_l: u8 = 0b1000_0000;
        let new_carry = value & shift_into_carry_l;
        let shift_into_result = if throug_carry && self.is_carry_flag_set()
            || !throug_carry && new_carry != 0
        {
            1
        } else {
            0
        };
        let result = (value << 1) | shift_into_result;
        (
            ConditionCodes {
                zero: if set_zero && result == 0  {
                    FlagState::Set
                } else {
                    FlagState::Unset
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

    fn rl_x_r8(
        &mut self,
        target: Register8Bit,
        through_carry: bool,
        set_zero: bool,
    ) -> ConditionCodes {
        let value = self.get_8bit_register(target);
        let (condition_codes_result, result) = self.rl_u8(value, through_carry, set_zero);
        self.set_8bit_register(target, result);
        condition_codes_result
    }

    fn rl_x_hl(
        &mut self,
        through_carry: bool,
        set_zero: bool,
    ) -> ConditionCodes {
        let mem_addr = self.get_16bit_register(Register16Bit::HL);
        let value = self.mmu.read_byte(mem_addr);
        let (condition_codes_result, result) = self.rl_u8(value, through_carry, set_zero);
        self.mmu.write_byte(mem_addr, result);
        condition_codes_result
    }

    pub fn rl_r8(&mut self, target: Register8Bit) -> InstructionResult {
        let through_carry = true;
        let set_zero = true;
        InstructionResult {
            cycles: 2,
            bytes: 2,
            condition_codes: self.rl_x_r8(target, through_carry, set_zero),
        }
    }

    pub fn rl_hl(&mut self) -> InstructionResult {
        let through_carry = true;
        let set_zero = true;
        InstructionResult {
            cycles: 4,
            bytes: 2,
            condition_codes: self.rl_x_hl(through_carry, set_zero),
        }
    }

    pub fn rl_a(&mut self) -> InstructionResult {
        let through_carry = true;
        let set_zero = false;
        InstructionResult {
            cycles: 1,
            bytes: 1,
            condition_codes: self.rl_x_r8(Register8Bit::A, through_carry, set_zero),
        }
    }

    pub fn rl_c_r8(&mut self, target: Register8Bit) -> InstructionResult {
        let through_carry = false;
        let set_zero = true;
        InstructionResult {
            cycles: 2,
            bytes: 2,
            condition_codes: self.rl_x_r8(target, through_carry, set_zero),
        }
    }

    pub fn rl_c_hl(&mut self) -> InstructionResult {
        let through_carry = false;
        let set_zero = true;
        InstructionResult {
            cycles: 4,
            bytes: 2,
            condition_codes: self.rl_x_hl(through_carry, set_zero),
        }
    }

    pub fn rl_c_a(&mut self) -> InstructionResult {
        let through_carry = false;
        let set_zero = false;
        InstructionResult {
            cycles: 1,
            bytes: 1,
            condition_codes: self.rl_x_r8(Register8Bit::A, through_carry, set_zero),
        }
    }
}

#[test]
pub fn rl_test() {
    let mut cpu = CPU::new(Vec::new());
    cpu.mmu.set_bootrom_enabled(false);

    //Test rl_r8
    cpu.clear_carry_flag();
    cpu.set_8bit_register(Register8Bit::B, 128);

    let mut  instruction_result = cpu.rl_r8(Register8Bit::B);
    assert_eq!(cpu.get_8bit_register(Register8Bit::B), 0);
    assert!(instruction_result.condition_codes.carry == FlagState::Set);
    cpu.set_carry_flag();

    instruction_result = cpu.rl_r8(Register8Bit::B);
    assert_eq!(cpu.get_8bit_register(Register8Bit::B), 1);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
    cpu.clear_carry_flag();

    instruction_result = cpu.rl_r8(Register8Bit::B);
    assert_eq!(cpu.get_8bit_register(Register8Bit::B), 2);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);

    //Test rl_a
    cpu.clear_carry_flag();
    cpu.set_8bit_register(Register8Bit::A, 128);

    instruction_result = cpu.rl_a();
    assert_eq!(cpu.get_8bit_register(Register8Bit::A), 0);
    assert!(instruction_result.condition_codes.carry == FlagState::Set);
    cpu.set_carry_flag();

    instruction_result = cpu.rl_r8(Register8Bit::A);
    assert_eq!(cpu.get_8bit_register(Register8Bit::A), 1);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
    cpu.clear_carry_flag();

    instruction_result = cpu.rl_r8(Register8Bit::A);
    assert_eq!(cpu.get_8bit_register(Register8Bit::A), 2);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);

    //Test rl_hl
    cpu.clear_carry_flag();
    let mem_addr = 0b0000_0000_1000_0000;
    cpu.set_16bit_register(Register16Bit::HL, mem_addr);
    cpu.mmu.write_byte(mem_addr, 128);

    instruction_result = cpu.rl_hl();
    assert_eq!(cpu.mmu.read_byte(mem_addr), 0);
    assert!(instruction_result.condition_codes.carry == FlagState::Set);
    cpu.set_carry_flag();

    instruction_result = cpu.rl_hl();
    assert_eq!(cpu.mmu.read_byte(mem_addr), 1);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
    cpu.clear_carry_flag();

    instruction_result = cpu.rl_hl();
    assert_eq!(cpu.mmu.read_byte(mem_addr), 2);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);

    //Test rl_c_r8
    cpu.clear_carry_flag();
    cpu.set_8bit_register(Register8Bit::B, 128);

    instruction_result = cpu.rl_c_r8(Register8Bit::B);
    assert_eq!(cpu.get_8bit_register(Register8Bit::B), 1);
    assert!(instruction_result.condition_codes.carry == FlagState::Set);
    cpu.set_carry_flag();

    instruction_result = cpu.rl_c_r8(Register8Bit::B);
    assert_eq!(cpu.get_8bit_register(Register8Bit::B), 2);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
    cpu.clear_carry_flag();

    instruction_result = cpu.rl_c_r8(Register8Bit::B);
    assert_eq!(cpu.get_8bit_register(Register8Bit::B), 4);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);

    //Test rl_c_a
    cpu.clear_carry_flag();
    cpu.set_8bit_register(Register8Bit::A, 128);

    instruction_result = cpu.rl_c_a();
    assert_eq!(cpu.get_8bit_register(Register8Bit::A), 1);
    assert!(instruction_result.condition_codes.carry == FlagState::Set);
    cpu.set_carry_flag();

    instruction_result = cpu.rl_c_r8(Register8Bit::A);
    assert_eq!(cpu.get_8bit_register(Register8Bit::A), 2);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
    cpu.clear_carry_flag();

    instruction_result = cpu.rl_c_r8(Register8Bit::A);
    assert_eq!(cpu.get_8bit_register(Register8Bit::A), 4);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);

    //Test rl_c_hl
    cpu.clear_carry_flag();
    let mem_addr = 0b0000_0000_1000_0000;
    cpu.set_16bit_register(Register16Bit::HL, mem_addr);
    cpu.mmu.write_byte(mem_addr, 128);

    instruction_result = cpu.rl_c_hl();
    assert_eq!(cpu.mmu.read_byte(mem_addr), 1);
    assert!(instruction_result.condition_codes.carry == FlagState::Set);
    cpu.set_carry_flag();

    instruction_result = cpu.rl_c_hl();
    assert_eq!(cpu.mmu.read_byte(mem_addr), 2);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
    cpu.clear_carry_flag();

    instruction_result = cpu.rl_c_hl();
    assert_eq!(cpu.mmu.read_byte(mem_addr), 4);
    assert!(instruction_result.condition_codes.carry == FlagState::Unset);
}
