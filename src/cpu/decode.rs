use super::{
    instructions::{InstParam, InstructionCondition, Instructions},
    registers::{Register16Bit, Register8Bit},
    CPU,
};

impl CPU {
    /// Decode the tail of an opcode to a 8 Bit Register
    fn tail_to_inst_param(&self, tail: u8) -> InstParam {
        // The tail repeats every 8 values, e.g. 0x0 & 0x8 are the same (B)
        let tail = if tail > 0x7 { tail - 0x8 } else { tail };

        match tail {
            0x0 => InstParam::Register8Bit(Register8Bit::B),
            0x1 => InstParam::Register8Bit(Register8Bit::C),
            0x2 => InstParam::Register8Bit(Register8Bit::D),
            0x3 => InstParam::Register8Bit(Register8Bit::E),
            0x4 => InstParam::Register8Bit(Register8Bit::H),
            0x5 => InstParam::Register8Bit(Register8Bit::L),
            0x6 => InstParam::Number8Bit(self.get_8bit_from_hl()),
            0x7 => InstParam::Register8Bit(Register8Bit::A),
            _ => panic!("Unknown tail: {:X}", tail),
        }
    }

    /// Calculate the target of a LD instruction based on the opcode
    /// Returns None if the opcode is not a LD instruction
    fn opcode_to_ld_target(&self, opcode: u8) -> Option<InstParam> {
        Some(match opcode {
            0x40..=0x47 => InstParam::Register8Bit(Register8Bit::B),
            0x48..=0x4F => InstParam::Register8Bit(Register8Bit::C),
            0x50..=0x57 => InstParam::Register8Bit(Register8Bit::D),
            0x58..=0x5F => InstParam::Register8Bit(Register8Bit::E),
            0x60..=0x67 => InstParam::Register8Bit(Register8Bit::H),
            0x68..=0x6F => InstParam::Register8Bit(Register8Bit::L),
            0x70..=0x77 => InstParam::Register16Bit(Register16Bit::HL),
            0x78..=0x7F => InstParam::Register8Bit(Register8Bit::A),
            _ => return None,
        })
    }

    /// Get a 16-bit value from the program counter at the next two positions (PC + 1, PC + 2)
    /// @warning: This will *not* increment the program counter
    fn get_16bit_from_pc(&self) -> u16 {
        self.memory.read_word(self.get_16bit_register(Register16Bit::PC) + 1)
    }

    /// Get a 8-bit value from the program counter at the next position (PC + 1)
    /// @warning: This will *not* increment the program counter
    fn get_8bit_from_pc(&self) -> u8 {
        self.memory.read_byte(self.get_16bit_register(Register16Bit::PC) + 1)
    }

    fn get_8bit_from_hl(&self) -> u8 {
        self.memory.read_byte(self.get_16bit_register(Register16Bit::HL))
    }

    pub fn decode(&self, opcode: u8) -> Result<Instructions, String> {
        // Split the opcode into head and tail
        // The head is the first 4 bits of the opcode e.g. 0x42 -> 0x4
        // The tail is the last 4 bits of the opcode e.g. 0x42 -> 0x2
        // This makes it a bit easier to decode the opcode
        let head = opcode >> 4;
        let tail = opcode & 0xF;
        Ok(match head {
            0x0 => match tail {
                0x0 => Instructions::NOP,
                _ => return Err(format!("Unknown opcode {:#02X}", opcode)),
            },
            0x3 => {
                match tail {
                    0x0 => Instructions::JR(InstParam::ConditionCodes(InstructionCondition::NotCarry), InstParam::Number8Bit(self.get_8bit_from_pc())),
                    0xC => Instructions::INC(InstParam::Register8Bit(Register8Bit::A)),
                    _ => return Err(format!("Unknown opcode {:#02X}", opcode)),
                }
            }
            // LD instructions (& HALT)
            0x4..=0x7 => {
                let value = self.tail_to_inst_param(tail);
                let ld_target = match self.opcode_to_ld_target(opcode) {
                    Some(target) => target,
                    None => return Err(format!("Unknown opcode {:#02X}", opcode)),
                };

                // There is a single opcode within this range that is not a LD instruction
                if opcode == 0x76 {
                    Instructions::HALT
                } else {
                    Instructions::LD(ld_target, value)
                }
            }
            // ADD, ADC, SUB, SBC, AND, XOR, OR, CP
            0x8..=0xB => {
                let value = self.tail_to_inst_param(tail);
                let is_second_half = tail > 0x7;

                if is_second_half {
                    match head {
                        0x8 => Instructions::ADC(value),
                        0x9 => Instructions::SBC(value),
                        0xA => Instructions::XOR(value),
                        0xB => Instructions::CP(value),
                        _ => return Err(format!("Unknown opcode {:#02X}", opcode)),
                    }
                } else {
                    match head {
                        0x8 => Instructions::ADD(value),
                        0x9 => Instructions::SUB(value),
                        0xA => Instructions::AND(value),
                        0xB => Instructions::OR(value),
                        _ => return Err(format!("Unknown opcode {:#02X}", opcode)),
                    }
                
                }
            }
            0xC => {
                match tail {
                    0x0 => Instructions::RET(InstParam::ConditionCodes(InstructionCondition::NotZero)),
                    0x1 => Instructions::POP(InstParam::Register16Bit(Register16Bit::BC)),
                    0x2 => Instructions::JP(InstParam::ConditionCodes(InstructionCondition::NotZero), InstParam::Number16Bit(self.get_16bit_from_pc())),
                    0x3 => Instructions::JP(InstParam::ConditionCodes(InstructionCondition::SkipConditionCodes), InstParam::Number16Bit(self.get_16bit_from_pc())),
                    0x4 => Instructions::CALL(InstParam::ConditionCodes(InstructionCondition::NotZero), InstParam::Number16Bit(self.get_16bit_from_pc())),
                    0x5 => Instructions::PUSH(InstParam::Register16Bit(Register16Bit::BC)),
                    0x6 => Instructions::ADD(InstParam::Number8Bit(self.get_8bit_from_pc())),
                    0x7 => Instructions::RST(InstParam::Number8Bit(0x00)),
                    0x8 => Instructions::RET(InstParam::ConditionCodes(InstructionCondition::Zero)),
                    0x9 => Instructions::RET(InstParam::ConditionCodes(InstructionCondition::SkipConditionCodes)),
                    0xA => Instructions::JP(InstParam::ConditionCodes(InstructionCondition::Zero), InstParam::Number16Bit(self.get_16bit_from_pc())),
                    0xB => return Err(format!("Prefixed Opcodes should have already been handled 😕 {:#02X}", opcode)),
                    0xC => Instructions::CALL(InstParam::ConditionCodes(InstructionCondition::Zero), InstParam::Number16Bit(self.get_16bit_from_pc())),
                    0xD => Instructions::CALL(InstParam::ConditionCodes(InstructionCondition::SkipConditionCodes), InstParam::Number16Bit(self.get_16bit_from_pc())),
                    0xE => Instructions::ADC(InstParam::Number8Bit(self.get_8bit_from_pc())),
                    0xF => Instructions::RST(InstParam::Number8Bit(0x08)),
                    _ => return Err(format!("Unknown opcode {:#02X}", opcode)),
                }
            }
            _ => return Err(format!("Unknown opcode {:#02X}", opcode)),
        })
    }
}
