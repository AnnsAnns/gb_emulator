use crate::cpu::{instructions::{InstParam, Instructions}, registers::{Register16Bit, Register8Bit}, CPU};

impl CPU {
    /// Decode the unprefixed common opcodes (0x0 - 0x3)
    pub fn decode_0x0_to_0x3_commons(&self, opcode: u8) -> Result<Instructions, String> {
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
                    InstParam::Register16Bit(Register16Bit::HL) // Special case for (HL)
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
}