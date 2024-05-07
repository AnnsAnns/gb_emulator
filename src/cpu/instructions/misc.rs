use crate::{cpu::CPU, test_helpers::{assert_correct_instruction_decode, assert_correct_instruction_step}};

use super::{ConditionCodes, FlagState, InstructionResult, Instructions};

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

#[test]
pub fn nop_test() {
    let mut cpu = CPU::new();
    let mut expected_result = InstructionResult::default();
    expected_result.bytes = 1;
    expected_result.cycles = 1;
    assert_correct_instruction_step(&mut cpu, Instructions::NOP, expected_result);
    assert_correct_instruction_decode(&mut cpu, 0x00, Instructions::NOP);
}