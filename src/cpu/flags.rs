use super::CPU;

/// The flags register is a special register in the CPU that contains
/// the status of the last operation that was performed.
/// The flags register is 8-bits, but only 4 of those bits are used.
/// See: https://gbdev.io/pandocs/CPU_Registers_and_Flags.html
const ZERO_FLAG: u8 = 0b1000_0000; // Bit 7
const SUBTRACTION_FLAG: u8 = 0b0100_0000; // Bit 6
const HALF_CARRY_FLAG: u8 = 0b0010_0000; // Bit 5
const CARRY_FLAG: u8 = 0b0001_0000; // Bit 4

impl CPU {
    /// Set the zero flag
    /// This is used to indicate that the result of the last operation was zero
    pub fn set_zero_flag(&mut self) {
        self.registers[1] |= ZERO_FLAG;
    }

    /// Unset the zero flag
    /// Thus, the result of the last operation was not zero
    pub fn clear_zero_flag(&mut self) {
        self.registers[1] &= !ZERO_FLAG;
    }

    /// Check if the zero flag is set
    pub fn is_zero_flag_set(&self) -> bool {
        self.registers[1] & ZERO_FLAG == ZERO_FLAG
    }

    /// Set the subtraction flag
    /// This is used to indicate that the last operation was a subtraction
    pub fn set_subtraction_flag(&mut self) {
        self.registers[1] |= SUBTRACTION_FLAG;
    }

    /// Unset the subtraction flag
    /// Thus, the last operation was not a subtraction
    pub fn clear_subtraction_flag(&mut self) {
        self.registers[1] &= !SUBTRACTION_FLAG;
    }

    /// Check if the subtraction flag is set
    pub fn is_subtraction_flag_set(&self) -> bool {
        self.registers[1] & SUBTRACTION_FLAG == SUBTRACTION_FLAG
    }

    /// Set the half carry flag
    /// This is used to indicate that there was a carry from the lower nibble
    pub fn set_half_carry_flag(&mut self) {
        self.registers[1] |= HALF_CARRY_FLAG;
    }

    /// Unset the half carry flag
    /// Thus, there was no carry from the lower nibble
    pub fn clear_half_carry_flag(&mut self) {
        self.registers[1] &= !HALF_CARRY_FLAG;
    }

    /// Check if the half carry flag is set
    pub fn is_half_carry_flag_set(&self) -> bool {
        self.registers[1] & HALF_CARRY_FLAG == HALF_CARRY_FLAG
    }

    /// Set the carry flag
    /// This is used to indicate that there was a carry from the operation
    pub fn set_carry_flag(&mut self) {
        self.registers[1] |= CARRY_FLAG;
    }

    /// Unset the carry flag
    /// Thus, there was no carry from the operation
    pub fn clear_carry_flag(&mut self) {
        self.registers[1] &= !CARRY_FLAG;
    }

    /// Check if the carry flag is set
    pub fn is_carry_flag_set(&self) -> bool {
        self.registers[1] & CARRY_FLAG == CARRY_FLAG
    }

    /// Get the flags register as a u8
    pub fn flags_to_u8(&self) -> u8 {
        self.registers[1]
    }
}