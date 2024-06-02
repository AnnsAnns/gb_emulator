use macroquad::prelude::*;

use super::{interrupts::InterruptTypes, CPU};

const JOYPAD_REGISTER: u16 = 0xFF00;

impl CPU {
    pub fn update_key_input(&mut self) {
        let keys_down = get_keys_down();

        let previous_data = self.memory.read_byte(JOYPAD_REGISTER);

        // Get the relevant bits of the joypad register (Inverted because the buttons are active low)
        let selected_buttons = (!previous_data & 0x20) != 0;
        let selected_directions = (!previous_data & 0x10) != 0;

        let mut output = previous_data;

        let key_map = if selected_buttons {
            [
                (KeyCode::Right, 0),
                (KeyCode::Left, 1),
                (KeyCode::Up, 2),
                (KeyCode::Down, 3),
            ]
        } else if selected_directions {
            [
                (KeyCode::A, 0),
                (KeyCode::B, 1),
                (KeyCode::Tab, 2),
                (KeyCode::Enter, 3),
            ]
        } else {
            return;
        };

        for (key, bit) in key_map.iter() {
            if keys_down.contains(key) {
                output &= !(1 << bit);
            } else {
                output |= 1 << bit;
            }
        }

        // If the joypad selects have changed, we need to set the joypad interrupt flag
        if previous_data != output {
            self.set_interrupt_flag(InterruptTypes::Joypad);
        }

        self.memory.write_controller_byte(output);
    }

    pub fn enable_buttons_debug(&mut self) {
        let mut joypad = self.memory.read_byte(JOYPAD_REGISTER);
        // Enable button by setting the 5th bit to 0
        joypad &= 0b1101_1111;
        self.memory.write_controller_byte(joypad);
    }
}