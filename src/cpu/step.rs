use super::{
    instructions::{InstParam, Instructions},
    registers::{self, Register16Bit, Register8Bit},
    CPU,
};

impl CPU {
    // Gets a 8-bit value from the HL register
    fn get_n8_from_hl(&self) -> u8 {
        self.memory
            .read_byte(self.get_16bit_register(Register16Bit::HL))
    }

    pub fn prepare_and_decode_next_instruction(&mut self) {
        let opcode = self.get_next_opcode();
        self.next_instruction = self.decode(opcode).unwrap();
    }

    /// Does a step (calls function and sets last_step_result),
    /// ensure to first set the next instruction
    /// by decoding it (see `decode.rs`)
    pub fn step(&mut self) {
        self.last_step_result = match &self.next_instruction {
            Instructions::NOP => self.nop(),
            Instructions::ADD(param) => match param {
                InstParam::Register8Bit(register) => self.add_a_r8(register.clone()),
                InstParam::Register16Bit(register) =>  self.add_hl_r16(*register),
                InstParam::SignedNumber8Bit(value) =>  self.add_sp_e8(*value),
                _ => panic!("ADD with {:?} not implemented", param),
            },
            Instructions::INC(param) => match param {
                InstParam::Register8Bit(register) => self.inc(register.clone()),
                InstParam::Register16Bit(register) => match register {
                    Register16Bit::SP => self.inc_sp(),
                    _ => panic!("INC with {:?} not implemented", register),
                    
                }
                _ => panic!("INC with {:?} not implemented", param),
            },
            Instructions::DEC(param) => match param {
                InstParam::Register16Bit(register) => match register {
                    Register16Bit::SP => self.dec_sp(),
                    _ => panic!("INC with {:?} not implemented", register),
                    
                }
                _ => panic!("INC with {:?} not implemented", param),
            },
            Instructions::PUSH(target) => match target {
                InstParam::Register16Bit(register) => {
                    if *register == Register16Bit::AF {
                        self.push_af()}
                    else { self.push_r16(*register)}
                }
                _ => panic!("PUSH with {:?} not implemented", target),
            }
            Instructions::POP(target) => match target {
                InstParam::Register16Bit(register) => {
                    if *register == Register16Bit::AF {
                        self.pop_af()}
                    else { self.pop_r16(*register)}
                }
                _ => panic!("PUSH with {:?} not implemented", target),
            }
            Instructions::BIT(bit, target) => match target {
                InstParam::Register8Bit(register) => match bit {
                    InstParam::Unsigned3Bit(targeted_bit) => {
                        self.bit_u3_r8(*targeted_bit, *register)
                    }
                    _ => panic!("BIT with {:?} not implemented", bit),
                },
                InstParam::Register16Bit(register) => {
                    if *register == Register16Bit::HL {
                        match bit {
                            InstParam::Unsigned3Bit(targeted_bit) => self.bit_u3_hl(*targeted_bit),
                            _ => panic!("BIT with {:?} not implemented", bit),
                        }
                    } else {
                        panic!("BIT with {:?} not implemented", target);
                    }
                }
                _ => panic!("BIT with {:?} not implemented", target),
            },
            Instructions::RES(bit, target) => match bit {
                InstParam::Unsigned3Bit(targeted_bit) => match target {
                    InstParam::Register8Bit(register) => self.res_u3_r8(*targeted_bit, *register),
                    InstParam::Register16Bit(register) => {
                        if *register == Register16Bit::HL {
                            self.res_u3_hl(*targeted_bit)
                        } else {
                            panic!("RES with {:?} not implemented", target);
                        }
                    }
                    _ => panic!("RES with {:?} not implemented", target),
                },
                _ => panic!("RES with {:?} not implemented", target),
            },
            Instructions::SET(bit, target) => match bit {
                InstParam::Unsigned3Bit(targeted_bit) => match target {
                    InstParam::Register8Bit(register) => self.set_u3_r8(*targeted_bit, *register),
                    InstParam::Register16Bit(register) => {
                        if *register == Register16Bit::HL {
                            self.set_u3_hl(*targeted_bit)
                        } else {
                            panic!("SET with {:?} not implemented", target);
                        }
                    }
                    _ => panic!("SET with {:?} not implemented", target),
                },
                _ => panic!("SET with {:?} not implemented", target),
            },
            Instructions::SWAP(target) => match target {
                InstParam::Register8Bit(register) => self.swap_r8(*register),
                InstParam::Register16Bit(register) => {
                    if *register == Register16Bit::HL {
                        self.swap_hl()
                    } else {
                        panic!("SWAP with {:?} not implemented", target);
                    }
                }
                _ => panic!("SWAP with {:?} not implemented", target),
            },
            Instructions::LD(target, source) => {
                match target {
                    InstParam::Register8Bit(target_register) => {
                        if *target_register == Register8Bit::A {
                            match source {
                                InstParam::Register16Bit(source_register) => {
                                    self.ld_a_r16(*source_register)
                                }
                                InstParam::Number16Bit(source_number) => {
                                    self.ld_a_n16(*source_number)
                                }
                                InstParam::Register8Bit(source_register) => {
                                    self.ld_r8_r8(*target_register, *source_register)
                                }
                                _ => panic!("Handling of {:?} not implemented", source),
                            }
                        } else {
                            match source {
                                InstParam::Register8Bit(source_register) => {
                                    self.ld_r8_r8(*target_register, *source_register)
                                }
                                InstParam::Number8Bit(source_number) => {
                                    self.ld_r8_n8(*target_register, *source_number)
                                }
                                InstParam::Register16Bit(source_register) => {
                                    self.ld_r8_hl(*target_register)
                                }
                                _ => panic!("Handling of {:?} not implemented", source),
                            }
                        }
                    }
                    InstParam::Register16Bit(target_register) => {
                        if *target_register == Register16Bit::SP {
                            match source {
                                InstParam::Register16Bit(source_register) => self.ld_sp_hl(),
                                InstParam::Number16Bit(source_address) => self.ld_sp_n16(*source_address),
                                _ => panic!("LD with {:?} not implemented", source),
                            }
                        }
                        else if *target_register == Register16Bit::HL {
                            //TODO what about HLI und HLD?
                            match source {
                                InstParam::Register8Bit(source_register) => {
                                    self.ld_hl_r8(*source_register)
                                }
                                InstParam::Number8Bit(source_number) => {
                                    self.ld_hl_n8(*source_number)
                                }
                                InstParam::SignedNumber8Bit(source_number) => self.ld_hl_sp_plus_e8(*source_number),
                                _ => panic!("Handling of {:?} not implemented", source),
                            }
                        } else {
                            match source {
                                InstParam::Number16Bit(source_number) => {
                                    self.ld_r16_n16(*target_register, *source_number)
                                }
                                InstParam::Register8Bit(source_register) => {
                                    self.ld_r16_a(*target_register)
                                }
                                _ => panic!("Handling of {:?} not implemented", source),
                            }
                        }
                    }
                    InstParam::Number16Bit(number) => match source {
                        InstParam::Register8Bit(source_register) => self.ld_n16_a(*number),
                        InstParam::Register16Bit(source_register) => self.ld_n16_sp(*number),
                        _ => panic!("LD with n16 address of {:?} not implemented", source),
                    }
                    _ => panic!("Handling of {:?} not implemented", target),
                }
            }
            _ => panic!("Handling of {:?} not implemented", self.next_instruction),
        };

        // Move the program counter to the next instruction
        // Depending on the bytes of the last instruction
        self.set_16bit_register(
            Register16Bit::PC,
            self.get_16bit_register(Register16Bit::PC) + self.last_step_result.bytes as u16,
        );
    }
}
