



use crate::cpu::CPU;

use crate::mmu::MemoryOperations;
#[cfg(test)]
use crate::test_helpers::{assert_correct_instruction_decode, assert_correct_instruction_step};

#[cfg(test)]
use crate::cpu::Instructions;

use super::{ConditionCodes, FlagState, InstructionResult, Register8Bit};

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
    /// activates the IME flag, enabling Interrupts
    pub fn ei(&mut self) -> InstructionResult {
        self.enable_ime = 2;
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

    /// deactivates the IME flag, enabling Interrupts
    pub fn di(&mut self) -> InstructionResult {
        self.ime_flag = false;
        self.enable_ime = 0;
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

    pub fn ccf(&self) -> InstructionResult {
        // complement carry flag
        InstructionResult {
            cycles: 1,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::Unset,
                half_carry: FlagState::Unset,
                carry: if self.is_carry_flag_set() {
                    FlagState::Unset
                } else {
                    FlagState::Set
                },
            },
        }
    }

    pub fn cpl(&mut self) -> InstructionResult {
        let value = self.get_8bit_register(Register8Bit::A);
        let result = !value;
        self.set_8bit_register(Register8Bit::A, result);
        // complement Register8Bit::A
        InstructionResult {
            cycles: 1,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::Set,
                half_carry: FlagState::Set,
                carry: FlagState::NotAffected,
            },
        }
    }

    pub fn daa(&mut self) -> InstructionResult {
        let mut value = self.get_8bit_register(Register8Bit::A);
        let mut daa_correction = 0;
        let mut set_carry = false;
        //currently set flags
        let half_carry = self.is_half_carry_flag_set();
        let carry = self.is_carry_flag_set();
        let subtraction = self.is_subtraction_flag_set();
        
        if half_carry || (!subtraction && (value & 0xf) > 9) {
            daa_correction |= 0x6;
        }
        if carry || (!subtraction && value > 0x99) {
            daa_correction |= 0x60;
            set_carry = true;
        }
        if subtraction {
            (value,_) = value.overflowing_sub(daa_correction);
        }else {
            (value,_) = value.overflowing_add(daa_correction);
        }
        
        

        self.set_8bit_register(Register8Bit::A, value);
        InstructionResult {
            cycles: 1,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: if value == 0 {
                    FlagState::Set
                } else {
                    FlagState::Unset
                },
                subtract: FlagState::NotAffected,
                half_carry: FlagState::Unset,
                carry: if set_carry {
                    FlagState::Set
                } else {
                    FlagState::Unset
                },
            },
        }
    }

    pub fn scf(&mut self) -> InstructionResult {
        InstructionResult {
            cycles: 1,
            bytes: 1,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::Unset,
                half_carry: FlagState::Unset,
                carry: FlagState::Set,
            },
        }
    }

    pub fn stop(&mut self) -> InstructionResult {
        //erstmal auslassen
        self.stop_mode = true;
        log::warn!("STOP was called. The emulator isn't dead, to reduce the complexity, stop will resume after 3 seconds.");
        std::thread::sleep(std::time::Duration::from_secs(3));
        self.stop_mode = false;
        InstructionResult {
            cycles: 0,
            bytes: 2,
            condition_codes: ConditionCodes {
                zero: FlagState::NotAffected,
                subtract: FlagState::NotAffected,
                half_carry: FlagState::NotAffected,
                carry: FlagState::NotAffected,
            },
        }
    }

    pub fn interrupt_pentding(&mut self) -> bool {
        let interrupt_enable = self.mmu.read_byte(0xFFFF);
        let interrupt_flag = self.mmu.read_byte(0xFF0F);
        if interrupt_enable & interrupt_flag == 0 {
            return false;
        }
        true
    }

    pub fn halt(&mut self) -> InstructionResult {
        self.is_halted = true;

        if !self.ime_flag && self.check_interrupts(false).is_some() {
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
        } else {
            InstructionResult {
                cycles: 0,
                bytes: 0,
                condition_codes: ConditionCodes {
                    zero: FlagState::NotAffected,
                    subtract: FlagState::NotAffected,
                    half_carry: FlagState::NotAffected,
                    carry: FlagState::NotAffected,
                },
            }
        }
    }
}

#[test]
pub fn nop_test() {
    let mut cpu = CPU::new(Vec::new());
    cpu.mmu.set_bootrom_enabled(false);
    let mut expected_result = InstructionResult::default();
    expected_result.bytes = 1;
    expected_result.cycles = 1;
    assert_correct_instruction_step(&mut cpu, Instructions::NOP, expected_result);
    assert_correct_instruction_decode(&mut cpu, 0x00, Instructions::NOP);
}

#[test]
pub fn ccf_test() {
    let mut cpu = CPU::new(Vec::new());
    cpu.mmu.set_bootrom_enabled(false);
    let mut expected_result_1 = InstructionResult::default();
    cpu.set_carry_flag();
    expected_result_1.bytes = 1;
    expected_result_1.cycles = 1;
    expected_result_1.condition_codes = ConditionCodes {
        zero: FlagState::NotAffected,
        subtract: FlagState::Unset,
        half_carry: FlagState::Unset,
        carry: FlagState::Unset,
    };
    assert_correct_instruction_step(&mut cpu, Instructions::CCF, expected_result_1);

    cpu.clear_carry_flag();
    let mut expected_result_2 = InstructionResult::default();
    expected_result_2.bytes = 1;
    expected_result_2.cycles = 1;
    expected_result_2.condition_codes = ConditionCodes {
        zero: FlagState::NotAffected,
        subtract: FlagState::Unset,
        half_carry: FlagState::Unset,
        carry: FlagState::Set,
    };
    assert_correct_instruction_step(&mut cpu, Instructions::CCF, expected_result_2);
}

#[test]
pub fn cpl_test() {
    let mut cpu = CPU::new(Vec::new());
    cpu.mmu.set_bootrom_enabled(false);
    let mut expected_result = InstructionResult::default();
    let value_start = 0b10101010;
    let value_expected_result = 0b01010101;
    expected_result.bytes = 1;
    expected_result.cycles = 1;
    expected_result.condition_codes = ConditionCodes {
        zero: FlagState::NotAffected,
        subtract: FlagState::Set,
        half_carry: FlagState::Set,
        carry: FlagState::NotAffected,
    };
    cpu.set_8bit_register(Register8Bit::A, value_start);
    assert_correct_instruction_step(&mut cpu, Instructions::CPL, expected_result);
    assert_eq!(
        value_expected_result,
        cpu.get_8bit_register(Register8Bit::A)
    );
}

#[test]
pub fn daa_test() {
    let mut cpu = CPU::new(Vec::new());
    cpu.mmu.set_bootrom_enabled(false);
    let mut expected_result = InstructionResult::default();
    let value_start = 0x9A;
    let value_expected_result = 0x00;
    expected_result.bytes = 1;
    expected_result.cycles = 1;
    expected_result.condition_codes = ConditionCodes {
        zero: FlagState::Set,
        subtract: FlagState::NotAffected,
        half_carry: FlagState::Unset,
        carry: FlagState::Set,
    };
    cpu.set_8bit_register(Register8Bit::A, value_start);
    assert_correct_instruction_step(&mut cpu, Instructions::DAA, expected_result);
    assert_eq!(
        value_expected_result,
        cpu.get_8bit_register(Register8Bit::A)
    );
}

#[test]
pub fn di_test() {
    let mut cpu = CPU::new(Vec::new());
    cpu.mmu.set_bootrom_enabled(false);
    let mut expected_result = InstructionResult::default();
    cpu.ime_flag = true;
    expected_result.bytes = 1;
    expected_result.cycles = 1;
    expected_result.condition_codes = ConditionCodes {
        zero: FlagState::NotAffected,
        subtract: FlagState::NotAffected,
        half_carry: FlagState::NotAffected,
        carry: FlagState::NotAffected,
    };
    assert_correct_instruction_step(&mut cpu, Instructions::DI, expected_result);
    assert!(!cpu.ime_flag);
}

#[test]
pub fn ei_test() {
    let mut cpu = CPU::new(Vec::new());
    cpu.mmu.set_bootrom_enabled(false);
    let mut expected_result = InstructionResult::default();
    expected_result.bytes = 1;
    expected_result.cycles = 1;
    expected_result.condition_codes = ConditionCodes {
        zero: FlagState::NotAffected,
        subtract: FlagState::NotAffected,
        half_carry: FlagState::NotAffected,
        carry: FlagState::NotAffected,
    };
    assert_correct_instruction_step(&mut cpu, Instructions::EI, expected_result);
    assert_eq!(1, cpu.enable_ime);
    cpu.next_instruction = Instructions::NOP;
    _ = cpu.step();
    assert_eq!(0, cpu.enable_ime);
    assert!(cpu.ime_flag);
}
