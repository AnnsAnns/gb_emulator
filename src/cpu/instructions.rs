use super::registers::{Register16Bit, Register8Bit};

pub mod arithmetic_and_logic;
pub mod bit_shift;
pub mod misc;
pub mod load;
pub mod bit_operations;
pub mod stack_operations;
pub mod jumps_subroutines;

/// The Flag States after an instruction
/// Set: The flag is set
/// Unset: The flag is unset
/// NotAffected: The flag is not affected by the instruction
#[derive(Debug, PartialEq, Clone)]
pub enum FlagState {
    Set,
    Unset,
    NotAffected,
}

/// The condition codes after an instruction
/// zero: The zero flag (Z)
/// subtract: The subtract flag (N)
/// half_carry: The half carry flag (H)
/// carry: The carry flag (C)
#[derive(Debug, PartialEq, Clone)]
pub struct ConditionCodes {
    zero: FlagState,
    subtract: FlagState,
    half_carry: FlagState,
    carry: FlagState,
}

/// The result of an instruction
/// cycles: The number of cycles the instruction took
/// bytes: The number of bytes the instruction took
/// condition_codes: The condition codes after the instruction
#[derive(Debug, PartialEq, Clone)]
pub struct InstructionResult {
    pub cycles: u8,
    pub bytes: u8,
    pub condition_codes: ConditionCodes,
}

impl InstructionResult {
    pub fn default() -> InstructionResult {
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

#[derive(Debug, PartialEq)]
pub enum InstructionCondition {
    Zero,
    NotZero,
    Carry,
    NotCarry,
}

#[derive(Debug, PartialEq)]
pub enum InstParam {
    Register8Bit(Register8Bit),
    Register16Bit(Register16Bit),
    ConditionCodes(InstructionCondition),
    Number8Bit(u8),
    SignedNumber8Bit(i8),
    Number16Bit(u16),
    Offset,
    Unsigned3Bit(u8),
}

#[derive(Debug, PartialEq)]
pub enum Instructions {
    ADD(InstParam),
    ADC(InstParam),
    SUB(InstParam),
    SBC(InstParam),
    AND(InstParam),
    XOR(InstParam),
    OR(InstParam),
    CP(InstParam),
    INC(InstParam),
    DEC(InstParam),

    LD(InstParam, InstParam),

    BIT(InstParam,InstParam),
    RES(InstParam,InstParam),
    SET(InstParam,InstParam),
    SWAP(InstParam),

    PUSH(InstParam),
    POP(InstParam),

    CALL(InstParam, InstParam),
    JP(InstParam, InstParam),
    JR(InstParam, InstParam),
    RET(InstParam),
    RETI,
    RST(InstParam),

    RLCA,
    RLA,
    RRCA,
    HALT,
    NOP,
}