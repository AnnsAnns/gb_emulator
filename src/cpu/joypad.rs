use macroquad::prelude::*;

use super::{interrupts::InterruptTypes, CPU};

const JOYPAD_REGISTER: u16 = 0xFF00;

impl CPU {
    fn check_bit(&self, byte: u8, bit: u8) -> bool {
        (byte & (1 << bit)) != 0
    }

    /// Joypad Key I/O Call
    /// stop_mode: If true, the CPU is in a STOP state and we should not set the interrupt flag
    pub fn update_key_input(&mut self) -> bool {
        let previous_data = self.memory.read_byte(JOYPAD_REGISTER);

        // Get the relevant bits of the joypad register (Inverted because the buttons are active low)
        let selected_buttons = self.check_bit(previous_data, 5) || self.stop_mode;
        let selected_directions = self.check_bit(previous_data, 4) || self.stop_mode;

        let mut output = previous_data;

        let key_map = if selected_directions {
            [
                (KeyCode::Right, 0),
                (KeyCode::Left, 1),
                (KeyCode::Up, 2),
                (KeyCode::Down, 3),
            ]
        } else if selected_buttons {
            [
                (KeyCode::A, 0),
                (KeyCode::B, 1),
                (KeyCode::Tab, 2),
                (KeyCode::Enter, 3),
            ]
        } else {
            return false;
        };

        for (key, bit) in key_map.iter() {
            if is_key_down(*key) {
                log::info!("Key pressed: {:?}", key);
                output &= !(1 << bit);
            } else {
                output |= 1 << bit;
            }
        }

        let result = previous_data != output;
        // If the joypad selects have changed, we need to set the joypad interrupt flag
        if result {
            if self.stop_mode {
                self.stop_mode = false;
            } else {               
                self.set_interrupt_flag(InterruptTypes::Joypad);
            }
        }

        self.memory.write_controller_byte(output);

        result
    }

    pub fn enable_buttons_debug(&mut self) {
        let mut joypad = self.memory.read_byte(JOYPAD_REGISTER);
        // Enable button by setting the 5th bit to 0
        joypad &= 0b1101_1111;
        self.memory.write_controller_byte(joypad);
    }
}