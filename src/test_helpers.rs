#[cfg(test)]
use crate::cpu::{instructions::InstructionResult, CPU};

#[cfg(test)]
use super::cpu::instructions::Instructions;

#[cfg(test)]
pub fn assert_correct_instruction_step(cpu: &mut CPU, instruction: Instructions, expected_result: InstructionResult) {
    cpu.set_instruction(instruction);
    let _ = cpu.step();
    assert_eq!(cpu.get_last_step_result(), expected_result);
}

#[cfg(test)]
pub fn assert_correct_instruction_decode(cpu: &mut CPU, opcode: u8, expected: Instructions) {
    assert_eq!(cpu.decode(opcode).unwrap(), expected);
}

#[cfg(test)]
pub fn assert_memory_read(cpu: &mut CPU, address: u16, expected: u8) {
    assert_eq!(cpu.get_memory().read_byte(address), expected);
}