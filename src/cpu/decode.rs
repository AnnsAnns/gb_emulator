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
            0x6 => InstParam::Register16Bit(Register16Bit::HL), //haben Befehle mit [HL] nicht meistens eine andere Byte oder Cycle Anzahl? (z.B. CP A,[HL] und CPA, n8)
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
        self.memory
            .read_word(self.get_16bit_register(Register16Bit::PC) + 1)
    }

    /// Get a 8-bit value from the program counter at the next position (PC + 1)
    /// @warning: This will *not* increment the program counter
    fn get_8bit_from_pc(&self) -> u8 {
        self.memory
            .read_byte(self.get_16bit_register(Register16Bit::PC) + 1)
    }

    fn get_8bit_from_hl(&self) -> u8 {
        self.memory
            .read_byte(self.get_16bit_register(Register16Bit::HL))
    }

    fn decode_0x0_to_0x3_commons(&self, opcode: u8) -> Result<Instructions, String> {
        let head = opcode >> 4;
        let tail = opcode & 0xF;

        let register_8bit = match head {
            0x0 => {
                if tail < 0xB {
                    Register8Bit::B
                } else {
                    Register8Bit::C
                }
            }
            0x1 => {
                if tail < 0xB {
                    Register8Bit::D
                } else {
                    Register8Bit::E
                }
            }
            0x2 => {
                if tail < 0xB {
                    Register8Bit::H
                } else {
                    Register8Bit::L
                }
            }
            0x3 => Register8Bit::A,
            _ => return Err(format!("{:#02X}", opcode)),
        };

        let register_16bit = match head {
            0x0 => Register16Bit::BC,
            0x1 => Register16Bit::DE,
            0x2 => Register16Bit::HL,
            0x3 => Register16Bit::SP,
            _ => return Err(format!("{:#02X}", opcode)),
        };

        Ok(match tail {
            0x1 => Instructions::LD(
                InstParam::Register16Bit(register_16bit),
                InstParam::Number16Bit(self.get_16bit_from_pc()),
            ),
            0x3 => Instructions::INC(InstParam::Register16Bit(register_16bit),InstParam::Boolean(false)),
            0x4 => Instructions::INC(if head == 0x3 {
                InstParam::Register16Bit(Register16Bit::HL) // Special case for (HL)
            } else {
                InstParam::Register8Bit(register_8bit)
            }, if head == 0x3 {InstParam::Boolean(true)}else{InstParam::Boolean(false)}),
            0x5 => Instructions::DEC(if head == 0x3 {
                InstParam::Register16Bit(Register16Bit::HL) // Special case for (HL)
            } else {
                InstParam::Register8Bit(register_8bit)
            }, if head == 0x3 {InstParam::Boolean(true)}else{InstParam::Boolean(false)}),
            0x6 => Instructions::LD(
                if head == 0x3 {
                    InstParam::Number16Bit(self.get_16bit_from_pc()) // Special case for (HL)
                } else {
                    InstParam::Register8Bit(register_8bit)
                },
                InstParam::Number8Bit(self.get_8bit_from_pc()),
            ),
            0x9 => Instructions::ADD_HL(InstParam::Register16Bit(register_16bit)),
            0xB => Instructions::DEC(InstParam::Register16Bit(register_16bit),InstParam::Boolean(false)),
            0xC => Instructions::INC(InstParam::Register8Bit(register_8bit),InstParam::Boolean(false)),
            0xD => Instructions::DEC(InstParam::Register8Bit(register_8bit),InstParam::Boolean(false)),
            0xE => Instructions::LD(
                InstParam::Register8Bit(register_8bit),
                InstParam::Number8Bit(self.get_8bit_from_pc()),
            ),
            _ => return Err(format!("Not covered in common {:#02X}", opcode)),
        })
    }

    fn not_implemented(&self, opcode: u8) -> Result<Instructions, String> {
        Err(format!("Opcode is not implemented (yet): {:#02X}", opcode))
    }

    pub fn decode(&self, opcode: u8) -> Result<Instructions, String> {
        // 0xCB is a prefixed opcode with a completely different table
        if opcode == 0xCB {
            return self.decode_prefixed();
        }

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
                0x7 => Instructions::RLC(InstParam::Register8Bit(Register8Bit::A)),
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
                0xF => Instructions::RRC(InstParam::Register8Bit(Register8Bit::A)),
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
                0x7 => Instructions::RL(InstParam::Register8Bit(Register8Bit::A)),
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
                0xF => Instructions::RR(InstParam::Register8Bit(Register8Bit::A)),
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
                0x3 => Instructions::INVALID,
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
                0xB => Instructions::INVALID,
                0xC => Instructions::CALL(
                    InstParam::ConditionCodes(InstructionCondition::Carry),
                    InstParam::Number16Bit(self.get_16bit_from_pc()),
                ),
                0xD => Instructions::INVALID,
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
                0x3 | 0x4 => Instructions::INVALID,
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
                0xB..=0xD => Instructions::INVALID,
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
                0x4 => Instructions::INVALID,
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
                0xC | 0xD => Instructions::INVALID,
                0xE => Instructions::CP(InstParam::Number8Bit(self.get_8bit_from_pc())),
                0xF => Instructions::RST(InstParam::Number8Bit(0x38)),
                _ => self.not_implemented(opcode)?,
            },
            _ => self.not_implemented(opcode)?,
        })
    }

    /// Decode a prefixed opcode
    fn decode_prefixed(&self) -> Result<Instructions, String> {
        let opcode = self
            .memory
            .read_byte(self.get_16bit_register(Register16Bit::PC) + 1);

        let head = opcode >> 4;
        let tail = opcode & 0xF;

        let register_tail = if tail > 0x7 { tail - 0x8 } else { tail };
        let register = match register_tail {
            0x0 => InstParam::Register8Bit(Register8Bit::B),
            0x1 => InstParam::Register8Bit(Register8Bit::C),
            0x2 => InstParam::Register8Bit(Register8Bit::D),
            0x3 => InstParam::Register8Bit(Register8Bit::E),
            0x4 => InstParam::Register8Bit(Register8Bit::H),
            0x5 => InstParam::Register8Bit(Register8Bit::L),
            0x6 => InstParam::Number16Bit(self.get_16bit_register(Register16Bit::HL)),
            0x7 => InstParam::Register8Bit(Register8Bit::A),
            _ => return Err(format!("Unknown tail: {:#02X}", tail)),
        };

        // The second half of the tail is offset by 1
        let offset: u8 = if tail >= 0x8 { 1 } else { 0 };

        Ok(match head {
            0x0 => match tail {
                0x0..=0x7 => Instructions::RLC(register),
                0x8..=0xF => Instructions::RRC(register),
                _ => return self.not_implemented(opcode),
            }
            0x1 => match tail {
                0x0..=0x7 => Instructions::RL(register),
                0x8..=0xF => Instructions::RR(register),
                _ => return self.not_implemented(opcode),
            }
            0x2 => match tail {
                0x0..=0x7 => Instructions::SLA(register),
                0x8..=0xF => Instructions::SRA(register),
                _ => return self.not_implemented(opcode),
            }
            0x3 => match tail {
                0x0..=0x7 => Instructions::SWAP(register),
                0x8..=0xF => Instructions::SRL(register),
                _ => return self.not_implemented(opcode),
            }
            0x4 => Instructions::BIT(InstParam::Unsigned3Bit(0 + offset), register),
            0x5 => Instructions::BIT(InstParam::Unsigned3Bit(2 + offset), register),
            0x6 => Instructions::BIT(InstParam::Unsigned3Bit(4 + offset), register),
            0x7 => Instructions::BIT(InstParam::Unsigned3Bit(6 + offset), register),
            0x8 => Instructions::RES(InstParam::Unsigned3Bit(0 + offset), register),
            0x9 => Instructions::RES(InstParam::Unsigned3Bit(2 + offset), register),
            0xA => Instructions::RES(InstParam::Unsigned3Bit(4 + offset), register),
            0xB => Instructions::RES(InstParam::Unsigned3Bit(6 + offset), register),
            0xC => Instructions::SET(InstParam::Unsigned3Bit(0 + offset), register),
            0xD => Instructions::SET(InstParam::Unsigned3Bit(2 + offset), register),
            0xE => Instructions::SET(InstParam::Unsigned3Bit(4 + offset), register),
            0xF => Instructions::SET(InstParam::Unsigned3Bit(6 + offset), register),
            _ => return self.not_implemented(opcode),
        })
    }
}

/// Test the decoding of all opcodes
/// This will print out all decoded values and failed values
/// The decoded values will be written to `decoded_values.txt` and the failed values to `failed_values.txt`
/// This is useful to check if all opcodes are correctly decoded
/// Warning: This doesn't check if the decoded values are correct, only if they are decoded
/// Please cross-reference with a Gameboy opcode table
#[test]
pub fn test_decode() {
    let mut cpu = CPU::new(false);
    let mut decoded_values = String::new();
    let mut failed_values = String::new();

    for i in 0..=0xFF {
        // Write the opcode for 0xCB to memory
        cpu.memory
            .write_byte(cpu.get_16bit_register(Register16Bit::SP) + 1, i.clone());

        for opcode in [i, 0xCB] {
            let decoded_value = cpu.decode(opcode);

            let opcode = if opcode == 0xCB {
                format!("0xCB | {:#02X}", i)
            } else {
                format!("{:#02X}", i)
            };

            if let Ok(val) = decoded_value {
                decoded_values.push_str(&format!("{} -> {:?}\n", opcode, val));
            } else {
                failed_values.push_str(&format!(
                    "{} -> {:?}\n",
                    opcode,
                    decoded_value.unwrap_err()
                ));
            }
        }
    }

    println!("🟢 Decoded values: {:?}", decoded_values);
    println!("🔴 Failed values: {:?}", failed_values);

    // To file

    use std::fs::File;
    use std::io::Write;

    let mut file = File::create("decoded_values.txt").unwrap();
    file.write_all(decoded_values.as_bytes()).unwrap();
    let mut file = File::create("failed_values.txt").unwrap();
    file.write_all(failed_values.as_bytes()).unwrap();
}
