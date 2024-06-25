use crate::{cpu::{
    instructions::{ConditionCodes, FlagState, InstructionResult},
    registers::Register16Bit,
    CPU,
}, mmu::MemoryOperations};

#[cfg(test)]
use crate::test_helpers::assert_correct_instruction_step;

#[cfg(test)]
use crate::cpu::Instructions;

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
    /// loads address in target in PC, realising a jump. Currently jumps to the adress and then increases it by the bytes of the JP not sure if intended to do that after the jump
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
        } else {
            // 3 bytes for the jp instruction
            // Even if the condition is false, the instruction is still 3 bytes
            // So we need to increment the PC by 3
            self.set_16bit_register(Register16Bit::PC, self.get_16bit_register(Register16Bit::PC)+3);
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
            let value = self.get_16bit_register(Register16Bit::PC)+3;

            //push pc to stack
            self.dec_sp();
            let memory_address = self.get_16bit_register(Register16Bit::SP);
            let value1:u8 = (value >> 8) as u8;
            let value2:u8 = value as u8;

            self.mmu.write_byte(memory_address, value1);
            self.dec_sp();
            self.mmu.write_byte(memory_address-1, value2);


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
                self.call_n16(target);
            } else {
                // 3 bytes for the call_n16 instruction
                // Even if the condition is false, the instruction is still 3 bytes
                // So we need to increment the PC by 3
                self.set_16bit_register(Register16Bit::PC, self.get_16bit_register(Register16Bit::PC)+3);
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
            } else {
                // 1 bytes for the ret instruction
                // Even if the condition is false, the instruction is still 1 bytes
                // So we need to increment the PC by 1
                self.set_16bit_register(Register16Bit::PC, self.get_16bit_register(Register16Bit::PC)+1);
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
            // self.ei(); Der Ei aufruf funktioniert hier nicht, da unser superloop nicht durhclaufen wird und
            // somit die Cycels nicht aktualisiert werden. D.H. EI aktualsiert das IME flag zu spät. ==> 
            // direkt IME Flag setzen oder enable ime flag = 1 d.h. nach dieser Instruction
            self.ime_flag = true;
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
            let value = self.get_16bit_register(Register16Bit::PC)+1;

            //push pc to stack
            self.dec_sp();
            let memory_address = self.get_16bit_register(Register16Bit::SP);
            let value1:u8 = (value >> 8) as u8;
            let value2:u8 = value as u8;

            self.mmu.write_byte(memory_address, value1);
            self.dec_sp();
            self.mmu.write_byte(memory_address-1, value2);


            self.jp_n16(vec as u16);

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
            let next_address = self.get_16bit_register(Register16Bit::PC);
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

#[test]
pub fn jumps_subroutines_test() {
    let mut cpu = CPU::new(Vec::new());
    cpu.mmu.set_bootrom_enabled(false);
    let mut registers;
    // 1) CALL and JP
    cpu.set_16bit_register(Register16Bit::SP, 0xF000);
    cpu.set_16bit_register(Register16Bit::PC, 0x000A);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 6;
    expected_result.bytes = 3;
    assert_correct_instruction_step(&mut cpu, Instructions::CALL(super::InstParam::Number16Bit(0x00A0), super::InstParam::Number16Bit(0x0A00)), expected_result);

    registers = cpu.get_registry_dump();
    let register_value = Register16Bit::PC as usize;
    let high = registers[register_value] as u16;
    let low = registers[register_value + 1] as u16;
    let result = (high << 8) | low;
    assert_eq!(result, 0x00A0);

    cpu.set_16bit_register(Register16Bit::HL, 0x0A00);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 1;
    expected_result.bytes = 1;
    assert_correct_instruction_step(&mut cpu, Instructions::JP(super::InstParam::Register16Bit(Register16Bit::HL), super::InstParam::Number16Bit(0x000A)), expected_result);
    registers = cpu.get_registry_dump();
    let register_value = Register16Bit::PC as usize;
    let high = registers[register_value] as u16;
    let low = registers[register_value + 1] as u16;
    let result = (high << 8) | low;
    assert_eq!(result, 0x0A00);
    // 2) RET
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 4;
    expected_result.bytes = 1;
    assert_correct_instruction_step(&mut cpu, Instructions::RET(super::InstParam::Offset), expected_result);
    registers = cpu.get_registry_dump();
    let register_value = Register16Bit::PC as usize;
    let high = registers[register_value] as u16;
    let low = registers[register_value + 1] as u16;
    let result = (high << 8) | low;
    assert_eq!(result, 0x000D);
    // 3) JR
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 3;
    expected_result.bytes = 2;
    let next_instr = cpu.get_16bit_register(Register16Bit::PC)+2;
    assert_eq!(next_instr, 0x000F);
    assert_correct_instruction_step(&mut cpu, Instructions::JR(super::InstParam::SignedNumber8Bit(5),super::InstParam::SignedNumber8Bit(10)), expected_result);
    let result = cpu.get_16bit_register(Register16Bit::PC);
    assert_eq!(result, next_instr+5);
    // 4) RST
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 4;
    expected_result.bytes = 1;
    assert_correct_instruction_step(&mut cpu, Instructions::RST(super::InstParam::Number8Bit(0x18)), expected_result);
    registers = cpu.get_registry_dump();
    let register_value = Register16Bit::PC as usize;
    let high = registers[register_value] as u16;
    let low = registers[register_value + 1] as u16;
    let result = (high << 8) | low;
    assert_eq!(result, 0x18);
}
    