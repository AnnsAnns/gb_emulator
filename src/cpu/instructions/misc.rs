use crate::cpu::CPU;

use super::{ConditionCodes, FlagState, InstructionResult};

impl CPU {
    /// NOP instruction
    pub fn nop(&self) -> InstructionResult {
        // Do nothing
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
}