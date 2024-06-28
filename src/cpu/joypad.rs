use core::fmt;

use macroquad::prelude::*;

use crate::mmu::MemoryOperations;

use super::{interrupts::InterruptTypes, CPU};

const JOYPAD_REGISTER: u16 = 0xFF00;

#[derive(Debug, Default, Clone)]
pub struct PlayerInput {
    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,
    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
}

impl CPU {
    /// Joypad Key I/O Call
    /// stop_mode: If true, the CPU is in a STOP state and we should not set the interrupt flag
    pub fn update_key_input(&mut self, player_input: &PlayerInput) -> bool {
        //get prev button states:
        let action = self.mmu.IO.action_buttons;
        let direction = self.mmu.IO.direction_buttons;

        let new_direction = (!player_input.right as u8)
            | (!player_input.left as u8) << 1
            | (!player_input.up as u8) << 2
            | (!player_input.down as u8) << 3;

        let new_action = (!player_input.a as u8)
            | (!player_input.b as u8) << 1
            | (!player_input.select as u8) << 2
            | (!player_input.start as u8) << 3;

        //maybe update joypadbyte in memory?
        let mut inputs_changed = false;
        let selected = self.mmu.read_byte(JOYPAD_REGISTER) & 0x30;
        if selected == 0x10 {
            //bit 5 = action buttons
            inputs_changed = action != new_action;
            self.mmu.IO.write_controller_byte(selected | new_action);
            self.generate_joypad_interrupt()
        } else if selected == 0x20 {
            //bit 4 = direction buttons
            inputs_changed = direction != new_direction;
            self.mmu.IO.write_controller_byte(selected | new_direction);
            self.generate_joypad_interrupt()
        }
        // save current button states
        self.mmu.IO.action_buttons = new_action;
        self.mmu.IO.direction_buttons = new_direction;

        inputs_changed
    }

    fn generate_joypad_interrupt(&mut self) {
        if self.stop_mode {
            self.stop_mode = false;
        }
        self.set_interrupt_flag(InterruptTypes::Joypad);
    }

    pub fn enable_buttons_debug(&mut self) {
        let mut joypad = self.mmu.read_byte(JOYPAD_REGISTER);
        // Enable button by setting the 5th bit to 0
        joypad &= 0b1101_1111;
        self.mmu.IO.write_controller_byte(joypad);
    }
}
