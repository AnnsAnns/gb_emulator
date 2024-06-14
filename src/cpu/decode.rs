use super::{instructions::Instructions, CPU};

mod helpers;
mod unprefixed_commons;
mod unprefixed;
mod prefixed;
mod test;

impl CPU {
    /// Decode an opcode, returning the instruction
    pub fn decode(&self, opcode: u8) -> Result<Instructions, String> {
        // 0xCB is a prefixed opcode with a completely different table
        if opcode == 0xCB {
            self.decode_prefixed()
        } else {
            self.decode_unprefixed(opcode)
        }
    }
}