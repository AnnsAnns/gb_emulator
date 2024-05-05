pub mod arithmetic_and_logic;

/// The Flag States after an instruction
/// Set: The flag is set
/// Unset: The flag is unset
/// NotAffected: The flag is not affected by the instruction
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
pub struct InstructionResult {
    cycles: u8,
    bytes: u8,
    condition_codes: ConditionCodes,
}