use crate::cpu::{instructions::InstructionResult, CPU};

use super::cpu::instructions::Instructions;

#[cfg(test)]
pub fn assert_correct_instruction_step(instruction: Instructions, expected_result: InstructionResult) {
    let mut cpu = CPU::new();
    cpu.set_instruction(instruction);
    cpu.step();
    assert_eq!(cpu.get_last_step_result(), expected_result);
}