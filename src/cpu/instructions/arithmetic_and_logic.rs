mod add;
mod and;
mod inc;
mod dec;
mod or;
mod xor;
mod sub;
mod cp;

#[cfg(test)]
use crate::cpu::{
    instructions::{ConditionCodes, FlagState, InstructionResult, Instructions},
    registers::{Register16Bit, Register8Bit},
    CPU,
};
#[cfg(test)]
use crate::test_helpers::{assert_correct_instruction_step};

#[test]
pub fn arithmetics_8bit_16bit_test() {
    let mut cpu = CPU::new();
    let mut registers;
    //>>---------8bit Arithmetics--------->>
    // 1) ADD A,r8 
    cpu.set_8bit_register(Register8Bit::A, 0);
    cpu.set_8bit_register(Register8Bit::B, 15);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 1;
    expected_result.bytes = 1;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Unset,half_carry:FlagState::Unset,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::ADD(super::InstParam::Register8Bit(Register8Bit::B)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 15);
    //ADD A,n8
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 2;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Unset,half_carry:FlagState::Set,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::ADD(super::InstParam::Number8Bit(1)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 16);
    //<<---------8bit Arithmetics---------<<

    //>>---------16bit Arithmetics--------->>
    //INC r16
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 1;
    assert_correct_instruction_step(&mut cpu, Instructions::INC(super::InstParam::Register16Bit(Register16Bit::DE)), expected_result);
    assert_eq!(cpu.get_16bit_register(Register16Bit::DE), 1);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 1;
    assert_correct_instruction_step(&mut cpu, Instructions::INC(super::InstParam::Register16Bit(Register16Bit::DE)), expected_result);
    assert_eq!(cpu.get_16bit_register(Register16Bit::DE), 2);
    //DEC r16
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 1;
    assert_correct_instruction_step(&mut cpu, Instructions::DEC(super::InstParam::Register16Bit(Register16Bit::DE)), expected_result);
    assert_eq!(cpu.get_16bit_register(Register16Bit::DE), 1);
    //ADD HL,r16
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 1;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::NotAffected,subtract:FlagState::Unset,half_carry:FlagState::Unset,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::ADD(super::InstParam::Register16Bit(Register16Bit::DE)), expected_result);
    assert_eq!(cpu.get_16bit_register(Register16Bit::DE), 1);
    //<<---------16bit Arithmetics---------<<
    cpu.memory.write_byte(cpu.get_16bit_register(Register16Bit::HL), 3);
    //>>---------8bit Arithmetics--------->>
    //ADD A,[HL]
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 1;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Unset,half_carry:FlagState::Unset,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::ADD(super::InstParam::Register16Bit(Register16Bit::HL)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 19);
    //half carry and carry flags
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 2;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Unset,half_carry:FlagState::Set,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::ADD(super::InstParam::Number8Bit(13)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 32);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 2;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Set,subtract:FlagState::Unset,half_carry:FlagState::Unset,carry:FlagState::Set};
    assert_correct_instruction_step(&mut cpu, Instructions::ADD(super::InstParam::Number8Bit(224)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 0);
    //ADC (same function like for add above so 1 test should be eonugh for all ADC instructions)
    cpu.set_8bit_register(Register8Bit::C, 14);
    cpu.set_carry_flag();
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 1;
    expected_result.bytes = 1;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Unset,half_carry:FlagState::Unset,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::ADC(super::InstParam::Register8Bit(Register8Bit::C)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 15);
    //DEC r8 INC r8
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 1;
    expected_result.bytes = 1;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Unset,half_carry:FlagState::Set,carry:FlagState::NotAffected};
    assert_correct_instruction_step(&mut cpu, Instructions::INC(super::InstParam::Register8Bit(Register8Bit::A)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 16);
    //INC r8
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 1;
    expected_result.bytes = 1;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Set,half_carry:FlagState::Set,carry:FlagState::NotAffected};
    assert_correct_instruction_step(&mut cpu, Instructions::DEC(super::InstParam::Register8Bit(Register8Bit::A)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 15);
    //INC [HL]
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 3;
    expected_result.bytes = 1;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Unset,half_carry:FlagState::Unset,carry:FlagState::NotAffected};
    assert_correct_instruction_step(&mut cpu, Instructions::INC(super::InstParam::Register16Bit(Register16Bit::HL)), expected_result);
    assert_eq!(cpu.memory.read_byte(cpu.get_16bit_register(Register16Bit::HL)), 4);

    // SUB A,r8
    println!("regA: {}",cpu.get_8bit_register(Register8Bit::A));
    println!("regB: {}",cpu.get_8bit_register(Register8Bit::B));
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 1;
    expected_result.bytes = 1;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Set,subtract:FlagState::Set,half_carry:FlagState::Unset,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::SUB(super::InstParam::Register8Bit(Register8Bit::B)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 0);
    //SUB A,n8
    cpu.set_8bit_register(Register8Bit::A, 16);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 2;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Set,half_carry:FlagState::Set,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::SUB(super::InstParam::Number8Bit(2)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 14);
    //SUB A,[HL]
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 1;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Set,half_carry:FlagState::Unset,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::SUB(super::InstParam::Register16Bit(Register16Bit::HL)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 10);
    //SBC (same function like for add above so 1 test should be enough for all SBC instructions)
    cpu.set_8bit_register(Register8Bit::A, 6);
    cpu.set_8bit_register(Register8Bit::C, 2);
    cpu.set_carry_flag();
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 1;
    expected_result.bytes = 1;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Set,half_carry:FlagState::Unset,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::SBC(super::InstParam::Register8Bit(Register8Bit::C)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 3);
    //<<---------8bit Arithmetics---------<<
}

#[test]
pub fn logic_8bit_16bit_test() {
    let mut cpu = CPU::new();
    let mut registers;
    //CP
    cpu.set_8bit_register(Register8Bit::A, 15);
    cpu.set_8bit_register(Register8Bit::B, 15);
    cpu.set_16bit_register(Register16Bit::HL, 10);
    cpu.memory.write_byte(cpu.get_16bit_register(Register16Bit::HL), 16);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 1;
    expected_result.bytes = 1;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Set,subtract:FlagState::Set,half_carry:FlagState::Unset,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::CP(super::InstParam::Register8Bit(Register8Bit::B)), expected_result);

    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 2;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Set,half_carry:FlagState::Unset,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::CP(super::InstParam::Number8Bit(5)), expected_result);

    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 1;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Set,half_carry:FlagState::Set,carry:FlagState::Set};
    assert_correct_instruction_step(&mut cpu, Instructions::CP(super::InstParam::Register16Bit(Register16Bit::HL)), expected_result);
    //check A for no changes
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 15);
    //AND
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 2;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Unset,half_carry:FlagState::Set,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::AND(super::InstParam::Number8Bit(0b00000011)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 3);

    cpu.set_8bit_register(Register8Bit::A, 15);
    cpu.set_8bit_register(Register8Bit::B, 0);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 1;
    expected_result.bytes = 1;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Set,subtract:FlagState::Unset,half_carry:FlagState::Set,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::AND(super::InstParam::Register8Bit(Register8Bit::B)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 0);
    
    cpu.set_8bit_register(Register8Bit::A, 9);
    cpu.memory.write_byte(cpu.get_16bit_register(Register16Bit::HL), 10);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 1;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Unset,half_carry:FlagState::Set,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::AND(super::InstParam::Register16Bit(Register16Bit::HL)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 8);
    //OR
    cpu.set_8bit_register(Register8Bit::A, 9);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 2;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Unset,half_carry:FlagState::Unset,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::OR(super::InstParam::Number8Bit(0b00000010)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 11);

    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 1;
    expected_result.bytes = 1;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Unset,half_carry:FlagState::Unset,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::OR(super::InstParam::Register8Bit(Register8Bit::B)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 11);

    cpu.memory.write_byte(cpu.get_16bit_register(Register16Bit::HL), 15);
    cpu.set_8bit_register(Register8Bit::A, 0b00110000);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 1;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Unset,half_carry:FlagState::Unset,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::OR(super::InstParam::Register16Bit(Register16Bit::HL)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 0b00111111);
    
    //XOR
    cpu.set_8bit_register(Register8Bit::A, 9);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 2;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Unset,half_carry:FlagState::Unset,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::XOR(super::InstParam::Number8Bit(0b00000010)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 11);

    cpu.set_8bit_register(Register8Bit::B, 0b0111);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 1;
    expected_result.bytes = 1;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Unset,half_carry:FlagState::Unset,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::XOR(super::InstParam::Register8Bit(Register8Bit::B)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 12);

    cpu.memory.write_byte(cpu.get_16bit_register(Register16Bit::HL), 31);
    cpu.set_8bit_register(Register8Bit::A, 0b00110000);
    let mut expected_result = InstructionResult::default();
    expected_result.cycles = 2;
    expected_result.bytes = 1;
    expected_result.condition_codes = ConditionCodes{zero:FlagState::Unset,subtract:FlagState::Unset,half_carry:FlagState::Unset,carry:FlagState::Unset};
    assert_correct_instruction_step(&mut cpu, Instructions::XOR(super::InstParam::Register16Bit(Register16Bit::HL)), expected_result);
    registers = cpu.get_registry_dump();
    assert_eq!(registers[Register8Bit::A as usize], 0b00101111);
}