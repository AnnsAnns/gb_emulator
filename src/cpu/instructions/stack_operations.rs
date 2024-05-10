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
        let memory_address = self.get_16bit_register(Register16Bit::SP);
        let low_value = self.memory.read_byte(memory_address);
        self.inc_sp();
        let high_value: u16 = (self.memory.read_byte(memory_address+1) as u16) << 4;
        self.inc_sp();
        let combined_value:u16 = high_value+(low_value as u16);
        self.set_16bit_register(Register16Bit::AF, combined_value);

        InstructionResult {
            cycles: 3,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: if (low_value & 128) != 0 {
                    FlagState::Set
                }else {
                    FlagState::Unset
                },
                subtract: if (low_value & 64) != 0 {
                    FlagState::Set
                }else {
                    FlagState::Unset
                },
                half_carry: if (low_value & 32) != 0 {
                    FlagState::Set
                }else {
                    FlagState::Unset
                },
                carry: if (low_value & 16) != 0 {
                    FlagState::Set
                }else {
                    FlagState::Unset
                },
            },
        }
    }
    pub fn pop_r16 (&mut self, target:Register16Bit) -> InstructionResult {
        let memory_address = self.get_16bit_register(Register16Bit::SP);
        let low_value = self.memory.read_byte(memory_address);
        self.inc_sp();
        let high_value: u16 = (self.memory.read_byte(memory_address+1) as u16) << 4;
        self.inc_sp();
        let combined_value: u16 = high_value + (low_value as u16);
        self.set_16bit_register(target, combined_value);

        InstructionResult {
            cycles: 3,
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
        self.dec_sp();
        let value = self.get_8bit_register(Register8Bit::A);
        let memory_address = self.get_16bit_register(Register16Bit::SP);
        self.memory.write_byte(memory_address, value);
        self.dec_sp();
        let mut flags: u8 = 0;
        if self.is_zero_flag_set() {flags += 128;}
        if self.is_subtraction_flag_set() {flags += 64;}
        if self.is_half_carry_flag_set() {flags += 32;}
        if self.is_carry_flag_set() {flags += 16;}
        self.memory.write_byte(memory_address-1, flags);

        InstructionResult {
            cycles: 4,
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
        self.dec_sp();
        let memory_address = self.get_16bit_register(Register16Bit::SP);
        let value1;
        let value2;
        match target {
            Register16Bit::BC => {
                value1 = self.get_8bit_register(Register8Bit::B);
                value2 = self.get_8bit_register(Register8Bit::C);
            },
            Register16Bit::DE => {
                value1 = self.get_8bit_register(Register8Bit::D);
                value2 = self.get_8bit_register(Register8Bit::E);
            },
            Register16Bit::HL => {
                value1 = self.get_8bit_register(Register8Bit::H);
                value2 = self.get_8bit_register(Register8Bit::L);
            },
            _ => panic!("push with {:?} not intended", target),
        }
        self.memory.write_byte(memory_address, value1);
        self.dec_sp();
        self.memory.write_byte(memory_address-1, value2);


        InstructionResult {
            cycles: 4,
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

