use crate::cpu::{instructions::{InstParam, InstructionCondition, Instructions}, registers::{Register16Bit, Register8Bit}, CPU};

impl CPU {
    /// Decode an unprefixed opcode (Everything that is not 0xCB)
    pub fn decode_unprefixed(&self, opcode: u8) -> Result<Instructions, String> {
        // Split the opcode into head and tail
        // The head is the first 4 bits of the opcode e.g. 0x42 -> 0x4
        // The tail is the last 4 bits of the opcode e.g. 0x42 -> 0x2
        // This makes it a bit easier to decode the opcode
        let head = opcode >> 4;
        let tail = opcode & 0xF;
        Ok(match head {
            0x0 => match tail {
                0x0 => Instructions::NOP,
                0x1 => self.decode_0x0_to_0x3_commons(opcode)?,
                0x2 => Instructions::LD(
                    InstParam::Register16Bit(Register16Bit::BC),
                    InstParam::Register8Bit(Register8Bit::A),
                ),
                0x3..=0x6 => self.decode_0x0_to_0x3_commons(opcode)?,
                0x7 => Instructions::RLCA(),
                0x8 => Instructions::LD(
                    InstParam::Number16Bit(self.get_16bit_from_pc()),
                    InstParam::Register16Bit(Register16Bit::SP),
                ),
                0x9 => self.decode_0x0_to_0x3_commons(opcode)?,
                0xA => Instructions::LD(
                    InstParam::Register8Bit(Register8Bit::A),
                    InstParam::Register16Bit(Register16Bit::BC),
                ),
                0xB..=0xE => self.decode_0x0_to_0x3_commons(opcode)?,
                0xF => Instructions::RRCA(),
                _ => self.not_implemented(opcode)?,
            },
            0x1 => match tail {
                0x0 => Instructions::STOP,
                0x1 => self.decode_0x0_to_0x3_commons(opcode)?,
                0x2 => Instructions::LD(
                    InstParam::Register16Bit(Register16Bit::DE),
                    InstParam::Register8Bit(Register8Bit::A),
                ),
                0x3..=0x6 => self.decode_0x0_to_0x3_commons(opcode)?,
                0x7 => Instructions::RLA(),
                0x8 => Instructions::JR(
                    InstParam::ConditionCodes(InstructionCondition::SkipConditionCodes),
                    InstParam::SignedNumber8Bit(self.get_8bit_from_pc() as i8),
                ),
                0x9 => self.decode_0x0_to_0x3_commons(opcode)?,
                0xA => Instructions::LD(
                    InstParam::Register8Bit(Register8Bit::A),
                    InstParam::Register16Bit(Register16Bit::DE),
                ),
                0xB..=0xE => self.decode_0x0_to_0x3_commons(opcode)?,
                0xF => Instructions::RRA(),
                _ => self.not_implemented(opcode)?,
            },
            0x2 => match tail {
                0x0 => Instructions::JR(
                    InstParam::ConditionCodes(InstructionCondition::NotZero),
                    InstParam::SignedNumber8Bit(self.get_8bit_from_pc() as i8),
                ),
                0x1 => self.decode_0x0_to_0x3_commons(opcode)?,
                0x2 => Instructions::LDHLIA,
                0x3..=0x6 => self.decode_0x0_to_0x3_commons(opcode)?,
                0x7 => Instructions::DAA,
                0x8 => Instructions::JR(
                    InstParam::ConditionCodes(InstructionCondition::Zero),
                    InstParam::SignedNumber8Bit(self.get_8bit_from_pc() as i8),
                ),
                0x9 => self.decode_0x0_to_0x3_commons(opcode)?,
                0xA => Instructions::LDAHLI,
                0xB..=0xE => self.decode_0x0_to_0x3_commons(opcode)?,
                0xF => Instructions::CPL,
                _ => self.not_implemented(opcode)?,
            },
            0x3 => match tail {
                0x0 => Instructions::JR(
                    InstParam::ConditionCodes(InstructionCondition::NotCarry),
                    InstParam::SignedNumber8Bit(self.get_8bit_from_pc() as i8),
                ),
                0x1 => self.decode_0x0_to_0x3_commons(opcode)?,
                0x2 => Instructions::LDHLDA,
                0x3..=0x6 => self.decode_0x0_to_0x3_commons(opcode)?,
                0x7 => Instructions::SCF,
                0x8 => Instructions::JR(
                    InstParam::ConditionCodes(InstructionCondition::Carry),
                    InstParam::SignedNumber8Bit(self.get_8bit_from_pc() as i8),
                ),
                0x9 => self.decode_0x0_to_0x3_commons(opcode)?,
                0xA => Instructions::LDAHLD,
                0xB..=0xE => self.decode_0x0_to_0x3_commons(opcode)?,
                0xF => Instructions::CCF,
                _ => self.not_implemented(opcode)?,
            },
            // LD instructions (& HALT)
            0x4..=0x7 => {
                let value = self.tail_to_inst_param(tail);
                let ld_target = match self.opcode_to_ld_target(opcode) {
                    Some(target) => target,
                    None => return self.not_implemented(opcode),
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
                        0xB => match tail {
                            0xE => Instructions::CP(InstParam::Register16Bit(Register16Bit::HL)),
                            _ => Instructions::CP(value),
                        }
                        _ => self.not_implemented(opcode)?,
                    }
                } else {
                    match head {
                        0x8 => Instructions::ADD(value),
                        0x9 => Instructions::SUB(value),
                        0xA => Instructions::AND(value),
                        0xB => Instructions::OR(value),
                        _ => self.not_implemented(opcode)?,
                    }
                }
            }
            0xC => match tail {
                0x0 => Instructions::RET(InstParam::ConditionCodes(InstructionCondition::NotZero)),
                0x1 => Instructions::POP(InstParam::Register16Bit(Register16Bit::BC)),
                0x2 => Instructions::JP(
                    InstParam::ConditionCodes(InstructionCondition::NotZero),
                    InstParam::Number16Bit(self.get_16bit_from_pc()),
                ),
                0x3 => Instructions::JP(
                    InstParam::ConditionCodes(InstructionCondition::SkipConditionCodes),
                    InstParam::Number16Bit(self.get_16bit_from_pc()),
                ),
                0x4 => Instructions::CALL(
                    InstParam::ConditionCodes(InstructionCondition::NotZero),
                    InstParam::Number16Bit(self.get_16bit_from_pc()),
                ),
                0x5 => Instructions::PUSH(InstParam::Register16Bit(Register16Bit::BC)),
                0x6 => Instructions::ADD(InstParam::Number8Bit(self.get_8bit_from_pc())),
                0x7 => Instructions::RST(InstParam::Number8Bit(0x00)),
                0x8 => Instructions::RET(InstParam::ConditionCodes(InstructionCondition::Zero)),
                0x9 => Instructions::RET(InstParam::ConditionCodes(
                    InstructionCondition::SkipConditionCodes,
                )),
                0xA => Instructions::JP(
                    InstParam::ConditionCodes(InstructionCondition::Zero),
                    InstParam::Number16Bit(self.get_16bit_from_pc()),
                ),
                0xB => {
                    return Err(format!(
                        "Prefixed Opcodes should have already been handled 😕 {:#02X}",
                        opcode
                    ))
                }
                0xC => Instructions::CALL(
                    InstParam::ConditionCodes(InstructionCondition::Zero),
                    InstParam::Number16Bit(self.get_16bit_from_pc()),
                ),
                0xD => Instructions::CALL(
                    InstParam::ConditionCodes(InstructionCondition::SkipConditionCodes),
                    InstParam::Number16Bit(self.get_16bit_from_pc()),
                ),
                0xE => Instructions::ADC(InstParam::Number8Bit(self.get_8bit_from_pc())),
                0xF => Instructions::RST(InstParam::Number8Bit(0x08)),
                _ => self.not_implemented(opcode)?,
            },
            0xD => match tail {
                0x0 => Instructions::RET(InstParam::ConditionCodes(InstructionCondition::NotCarry)),
                0x1 => Instructions::POP(InstParam::Register16Bit(Register16Bit::DE)),
                0x2 => Instructions::JP(
                    InstParam::ConditionCodes(InstructionCondition::NotCarry),
                    InstParam::Number16Bit(self.get_16bit_from_pc()),
                ),
                0x3 => Instructions::INVALID(0x3),
                0x4 => Instructions::CALL(
                    InstParam::ConditionCodes(InstructionCondition::NotCarry),
                    InstParam::Number16Bit(self.get_16bit_from_pc()),
                ),
                0x5 => Instructions::PUSH(InstParam::Register16Bit(Register16Bit::DE)),
                0x6 => Instructions::SUB(InstParam::Number8Bit(self.get_8bit_from_pc())),
                0x7 => Instructions::RST(InstParam::Number8Bit(0x10)),
                0x8 => Instructions::RET(InstParam::ConditionCodes(InstructionCondition::Carry)),
                0x9 => Instructions::RETI,
                0xA => Instructions::JP(
                    InstParam::ConditionCodes(InstructionCondition::Carry),
                    InstParam::Number16Bit(self.get_16bit_from_pc()),
                ),
                0xB => Instructions::INVALID(0xB),
                0xC => Instructions::CALL(
                    InstParam::ConditionCodes(InstructionCondition::Carry),
                    InstParam::Number16Bit(self.get_16bit_from_pc()),
                ),
                0xD => Instructions::INVALID(0xD),
                0xE => Instructions::SBC(InstParam::Number8Bit(self.get_8bit_from_pc())),
                0xF => Instructions::RST(InstParam::Number8Bit(0x18)),
                _ => self.not_implemented(opcode)?,
            },
            0xE => match tail {
                0x0 => Instructions::LDH(
                    InstParam::Number8Bit(self.get_8bit_from_pc()),
                    InstParam::Register8Bit(Register8Bit::A),
                ),
                0x1 => Instructions::POP(InstParam::Register16Bit(Register16Bit::HL)),
                0x2 => Instructions::LDH(
                    InstParam::Register8Bit(Register8Bit::C),
                    InstParam::Register8Bit(Register8Bit::A),
                ),
                0x3 | 0x4 => Instructions::INVALID(0x3),
                0x5 => Instructions::PUSH(InstParam::Register16Bit(Register16Bit::HL)),
                0x6 => Instructions::AND(InstParam::Number8Bit(self.get_8bit_from_pc())),
                0x7 => Instructions::RST(InstParam::Number8Bit(0x20)),
                0x8 => {
                    Instructions::ADD(InstParam::SignedNumber8Bit(self.get_8bit_from_pc() as i8))
                }
                0x9 => Instructions::JP(
                    InstParam::ConditionCodes(InstructionCondition::SkipConditionCodes),
                    InstParam::Register16Bit(Register16Bit::HL),
                ),
                0xA => Instructions::LD(
                    InstParam::Number16Bit(self.get_16bit_from_pc()),
                    InstParam::Register8Bit(Register8Bit::A),
                ),
                0xB..=0xD => Instructions::INVALID(0xB),
                0xE => Instructions::XOR(InstParam::Number8Bit(self.get_8bit_from_pc())),
                0xF => Instructions::RST(InstParam::Number8Bit(0x28)),
                _ => self.not_implemented(opcode)?,
            },
            0xF => match tail {
                0x0 => Instructions::LDH(
                    InstParam::Register8Bit(Register8Bit::A),
                    InstParam::Number8Bit(self.get_8bit_from_pc()),
                ),
                0x1 => Instructions::POP(InstParam::Register16Bit(Register16Bit::AF)),
                0x2 => Instructions::LDH(
                    InstParam::Register8Bit(Register8Bit::A),
                    InstParam::Register8Bit(Register8Bit::C),
                ),
                0x3 => Instructions::DI,
                0x4 => Instructions::INVALID(0x4),
                0x5 => Instructions::PUSH(InstParam::Register16Bit(Register16Bit::AF)),
                0x6 => Instructions::OR(InstParam::Number8Bit(self.get_8bit_from_pc())),
                0x7 => Instructions::RST(InstParam::Number8Bit(0x30)),
                0x8 => Instructions::LD(InstParam::Register16Bit(Register16Bit::HL),InstParam::SignedNumber8Bit(
                    self.get_8bit_from_pc() as i8,
                )),
                0x9 => Instructions::LD(
                    InstParam::Register16Bit(Register16Bit::SP),
                    InstParam::Register16Bit(Register16Bit::HL),
                ),
                0xA => Instructions::LD(
                    InstParam::Register8Bit(Register8Bit::A),
                    InstParam::Number16Bit(self.get_16bit_from_pc()),
                ),
                0xB => Instructions::EI,
                0xC | 0xD => Instructions::INVALID(0xC),
                0xE => Instructions::CP(InstParam::Number8Bit(self.get_8bit_from_pc())),
                0xF => Instructions::RST(InstParam::Number8Bit(0x38)),
                _ => self.not_implemented(opcode)?,
            },
            _ => self.not_implemented(opcode)?,
        })
    }
}