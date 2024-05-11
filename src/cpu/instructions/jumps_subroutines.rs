use crate::cpu::{
    instructions::{ConditionCodes, FlagState, InstParam, InstructionResult, Instructions},
    registers::{Register16Bit, Register8Bit},
    CPU,
};

#[cfg(test)]
use crate::test_helpers::{assert_correct_instruction_step};

impl CPU {
    ///loads value in register HL in PC, realising a jump
    pub fn jp_hl(&mut self) -> InstructionResult {
        let value = self.get_16bit_register(Register16Bit::HL);
        self.set_16bit_register(Register16Bit::PC, value);

        InstructionResult {
            cycles: 1,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::NotAffected,
                half_carry: FlagState::NotAffected,
                carry: FlagState::NotAffected,
            },
        }
    }
    /// loads address in target in PC, realising a jump
    pub fn jp_n16(&mut self, target: u16) -> InstructionResult {
        
        self.set_16bit_register(Register16Bit::PC, target);

        InstructionResult {
            cycles: 4,
            bytes: 3,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::NotAffected,
                half_carry: FlagState::NotAffected,
                carry: FlagState::NotAffected,
            },
        }
    }
    /// if comdition cc is true: load address in target in PC, realising a jump
    pub fn jp_cc_n16(&mut self, cc:bool, target: u16) -> InstructionResult {
        if cc {
            self.set_16bit_register(Register16Bit::PC, target);
        }
        

        InstructionResult {
            cycles: if cc {4} else {3},
            bytes: 3,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::NotAffected,
                half_carry: FlagState::NotAffected,
                carry: FlagState::NotAffected,
            },
        }
    }
        /// pushes the next instructions address on the stack, then does a jp_n16
        pub fn call_n16(&mut self, target: u16) -> InstructionResult {
            let value = self.get_16bit_register(Register16Bit::PC);
            self.set_16bit_register(Register16Bit::PC, value+3);
            self.push_r16(Register16Bit::PC);
            self.jp_n16(target);

            InstructionResult {
                cycles: 6,
                bytes: 3,
                condition_codes: ConditionCodes {
                    zero: FlagState::NotAffected,
                    subtract: FlagState::NotAffected,
                    half_carry: FlagState::NotAffected,
                    carry: FlagState::NotAffected,
                },
            }
        }
        /// if condition cc is true: pushes the next instructions address on the stack, then does a jp_n16
        pub fn call_cc_n16(&mut self, cc: bool, target: u16) -> InstructionResult {
            if cc {
                let value = self.get_16bit_register(Register16Bit::PC);
                self.set_16bit_register(Register16Bit::PC, value+3);
                self.push_r16(Register16Bit::PC);
                self.jp_n16(target);
            }

            InstructionResult {
                cycles: if cc {4} else {3},
                bytes: 3,
                condition_codes: ConditionCodes {
                    zero: FlagState::NotAffected,
                    subtract: FlagState::NotAffected,
                    half_carry: FlagState::NotAffected,
                    carry: FlagState::NotAffected,
                },
            }
        }
        /// Pops PC from the stack, returning to the last pushed instruction
        pub fn ret(&mut self) -> InstructionResult {
            self.pop_r16(Register16Bit::PC);

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
        /// if condtition cc is true: Pops PC from the stack, returning to the last pushed instruction
        pub fn ret_cc(&mut self, cc: bool) -> InstructionResult {
            if cc {
                self.pop_r16(Register16Bit::PC);
            }

            InstructionResult {
                cycles: if cc {5} else {2},
                bytes: 1,
                condition_codes: ConditionCodes {
                    zero: FlagState::NotAffected,
                    subtract: FlagState::NotAffected,
                    half_carry: FlagState::NotAffected,
                    carry: FlagState::NotAffected,
                },
            }
        }
        /// Enables interrupts and pops PC from the stack, returning to the last pushed instruction
        pub fn reti(&mut self) -> InstructionResult {
            self.ei();
            self.pop_r16(Register16Bit::PC);

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
        /// calls address vec, for rst vectors: One of the RST vectors (0x00, 0x08, 0x10, 0x18, 0x20, 0x28, 0x30, and 0x38)
        pub fn rst_vec(&mut self,vec: u8) -> InstructionResult {
            self.call_n16(vec as u16);

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
        /// relativ jump to the address of the next instruction+signed_offset(8bit), signed_offset has to be in range -128 to 127 
        pub fn jr_n16(&mut self,signed_offset: i8) -> InstructionResult {
            let next_address = self.get_16bit_register(Register16Bit::PC) + 2;
            let jump_address = (next_address as i16 + signed_offset as i16) as u16;
            self.jp_n16(jump_address);

            InstructionResult {
                cycles: 3,
                bytes: 2,
                condition_codes: ConditionCodes {
                    zero: FlagState::NotAffected,
                    subtract: FlagState::NotAffected,
                    half_carry: FlagState::NotAffected,
                    carry: FlagState::NotAffected,
                },
            }
        }
        /// if condition cc is true: relativ jump to the address of the next instruction+signed_offset(8bit), signed_offset has to be in range -128 to 127 
        pub fn jr_cc_n16(&mut self,cc: bool, signed_offset: i8) -> InstructionResult {
            if cc {
                self.jr_n16(signed_offset);
            }

            InstructionResult {
                cycles: if cc {3} else {2},
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