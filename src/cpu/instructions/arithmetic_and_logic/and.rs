use crate::{cpu::{
    instructions::{ConditionCodes, FlagState, InstructionResult},
    registers::{Register16Bit, Register8Bit},
    CPU,
}, mmu::MemoryOperations};

impl CPU {
    /// Bitwise AND between the value in r8 and A.
    pub fn and_a_r8(&mut self, target: Register8Bit) -> InstructionResult {
        let value = self.get_8bit_register(target);
        let a = self.get_8bit_register(Register8Bit::A);
        let result = a & value;

        self.set_8bit_register(Register8Bit::A, result);

        InstructionResult {
            cycles: 1,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: if result == 0 {
                    FlagState::Set
                } else {
                    FlagState::Unset
                },
                subtract: FlagState::Unset,
                half_carry: FlagState::Set,
                carry: FlagState::Unset,
            },
        }
    }

    /// Bitwise AND between the value in memory address HL and A.
    pub fn and_a_hl(&mut self) -> InstructionResult {
        let mem_addr = self.get_16bit_register(Register16Bit::HL);
        let value = self.mmu.read_byte(mem_addr);
        let a = self.get_8bit_register(Register8Bit::A);
        let result = a & value;

        self.set_8bit_register(Register8Bit::A, result);

        InstructionResult {
            cycles: 2,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: if result == 0 {
                    FlagState::Set
                } else {
                    FlagState::Unset
                },
                subtract: FlagState::Unset,
                half_carry: FlagState::Set,
                carry: FlagState::Unset,
            },
        }
    }

    /// Bitwise AND between the value n8 and A.
    pub fn and_a_n8(&mut self, value: u8) -> InstructionResult {
        let a = self.get_8bit_register(Register8Bit::A);
        let result = a & value;

        self.set_8bit_register(Register8Bit::A, result);

        InstructionResult {
            cycles: 2,
            bytes: 2,
            condition_codes: ConditionCodes {
                zero: if result == 0 {
                    FlagState::Set
                } else {
                    FlagState::Unset
                },
                subtract: FlagState::Unset,
                half_carry: FlagState::Set,
                carry: FlagState::Unset,
            },
        }
    }
}
