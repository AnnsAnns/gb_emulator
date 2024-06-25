use crate::{cpu::{instructions::{InstParam, Instructions}, registers::{Register16Bit, Register8Bit}, CPU}, mmu::MemoryOperations};

impl CPU {
    /// Decode the tail of an opcode to a 8 Bit Register
    pub fn tail_to_inst_param(&self, tail: u8) -> InstParam {
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
    pub fn opcode_to_ld_target(&self, opcode: u8) -> Option<InstParam> {
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
    pub fn get_16bit_from_pc(&self) -> u16 {
        self.mmu
            .read_word(self.get_16bit_register(Register16Bit::PC) + 1)
    }

    /// Get a 8-bit value from the program counter at the next position (PC + 1)
    /// @warning: This will *not* increment the program counter
    pub fn get_8bit_from_pc(&self) -> u8 {
        self.mmu
            .read_byte(self.get_16bit_register(Register16Bit::PC) + 1)
    }

    pub fn get_8bit_from_hl(&self) -> u8 {
        self.mmu
            .read_byte(self.get_16bit_register(Register16Bit::HL))
    }

    pub fn not_implemented(&self, opcode: u8) -> Result<Instructions, String> {
        Err(format!("Opcode is not implemented (yet): {:#02X}", opcode))
    }
}