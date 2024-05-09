use crate::cpu::{
    instructions::{ConditionCodes, FlagState, InstParam, InstructionResult, Instructions},
    registers::{Register16Bit, Register8Bit},
    CPU,
};

#[cfg(test)]
use crate::test_helpers::{assert_correct_instruction_step};

impl CPU { //maybe move ld, dec and inc to their files?
    pub fn dec_sp(&mut self) -> InstructionResult {
        let sp = self.get_16bit_register(Register16Bit::SP);
        let value = sp-1;

        self.set_16bit_register(Register16Bit::SP, value);
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
    pub fn inc_sp(&mut self) -> InstructionResult {
        let sp = self.get_16bit_register(Register16Bit::SP);
        let value = sp+1;

        self.set_16bit_register(Register16Bit::SP, value);
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
    pub fn ld_sp_n16(&mut self, value:u16) -> InstructionResult {

        self.set_16bit_register(Register16Bit::SP, value);
        InstructionResult {
            cycles: 3,
            bytes: 3,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::NotAffected,
                half_carry: FlagState::NotAffected,
                carry: FlagState::NotAffected,
            },
        }
    }
    pub fn ld_n16_sp(&mut self, target:u16) -> InstructionResult {
        let sp = self.get_16bit_register(Register16Bit::SP);
        let value = (sp & 0xFF) as u8;
        let sp_shifted = (sp >> 8) as u8;
        self.memory.write_byte(target, value);
        self.memory.write_byte(target+1, sp_shifted);

        InstructionResult {
            cycles: 5,
            bytes: 3,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::NotAffected,
                half_carry: FlagState::NotAffected,
                carry: FlagState::NotAffected,
            },
        }
    }
    /// adds the signed value to sp and stores the result in HL.
    pub fn ld_hl_sp_plus_e8 (&mut self, value: i8) -> InstructionResult {
        let sp = self.get_16bit_register(Register16Bit::SP);
        let add_result = self.add_sp_e8(value);
        let added_value = self.get_16bit_register(Register16Bit::SP);

        self.set_16bit_register(Register16Bit::SP, sp);
        self.set_16bit_register(Register16Bit::HL, added_value);

        InstructionResult {
            cycles: 3,
            bytes: 2,
            condition_codes: ConditionCodes {
                zero: FlagState::Unset,
                subtract: FlagState::Unset,
                half_carry: add_result.condition_codes.half_carry,
                carry: add_result.condition_codes.carry,
            },
        }
    }
    // Load register  HL into register SP
    pub fn ld_sp_hl (&mut self) -> InstructionResult {
        let value = self.get_16bit_register(Register16Bit::HL);

        self.set_16bit_register(Register16Bit::SP, value);

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
    pub fn pop_af (&mut self) -> InstructionResult {


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
    pub fn pop_r16 (&mut self, target:Register16Bit) -> InstructionResult {


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
    pub fn push_af (&mut self) -> InstructionResult {


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
    pub fn push_r16 (&mut self, target:Register16Bit) -> InstructionResult {


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

