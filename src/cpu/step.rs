use super::{instructions::{InstParam, Instructions}, registers::Register16Bit, CPU};

impl CPU {
    // Gets a 8-bit value from the HL register
    fn get_n8_from_hl(&self) -> u8 {
        self.memory.read_byte(self.get_16bit_register(Register16Bit::HL))
    }

    /// Does a step (calls function and sets last_step_result), 
    /// ensure to first set the next instruction
    /// by decoding it (see `decode.rs`)
    pub fn step(&mut self) {
        self.last_step_result = match &self.next_instruction {
            Instructions::NOP => self.nop(),
            Instructions::ADD(param) => {
                match param {
                    InstParam::Register8Bit(register) => self.add_a_r8(register.clone()),
                    InstParam::Number8Bit => self.add_a_hl(),
                    _ => panic!("ADD with {:?} not implemented", param),
                }
            }
            _ => panic!("Handling of {:?} not implemented", self.next_instruction),
        }
    }
}