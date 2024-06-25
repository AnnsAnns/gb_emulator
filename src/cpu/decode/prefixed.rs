use crate::{cpu::{instructions::{InstParam, Instructions}, registers::{Register16Bit, Register8Bit}, CPU}, mmu::MemoryOperations};

impl CPU {
        /// Decode a prefixed opcode (0xCB)
        pub fn decode_prefixed(&self) -> Result<Instructions, String> {
            let opcode = self
                .mmu
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
                0x6 => InstParam::Register16Bit(Register16Bit::HL),
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
                0x4 => Instructions::BIT(InstParam::Unsigned3Bit(offset), register),
                0x5 => Instructions::BIT(InstParam::Unsigned3Bit(2 + offset), register),
                0x6 => Instructions::BIT(InstParam::Unsigned3Bit(4 + offset), register),
                0x7 => Instructions::BIT(InstParam::Unsigned3Bit(6 + offset), register),
                0x8 => Instructions::RES(InstParam::Unsigned3Bit(offset), register),
                0x9 => Instructions::RES(InstParam::Unsigned3Bit(2 + offset), register),
                0xA => Instructions::RES(InstParam::Unsigned3Bit(4 + offset), register),
                0xB => Instructions::RES(InstParam::Unsigned3Bit(6 + offset), register),
                0xC => Instructions::SET(InstParam::Unsigned3Bit(offset), register),
                0xD => Instructions::SET(InstParam::Unsigned3Bit(2 + offset), register),
                0xE => Instructions::SET(InstParam::Unsigned3Bit(4 + offset), register),
                0xF => Instructions::SET(InstParam::Unsigned3Bit(6 + offset), register),
                _ => return self.not_implemented(opcode),
            })
        }
}