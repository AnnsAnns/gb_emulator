use macroquad::prelude::*;

use super::{interrupts::InterruptTypes, CPU};

const JOYPAD_REGISTER: u16 = 0xFF00;

impl CPU {
    /// Joypad Key I/O Call
    /// stop_mode: If true, the CPU is in a STOP state and we should not set the interrupt flag
    pub fn update_key_input(&mut self) -> bool {
        //get prev button states:
        let action = self.memory.action_buttons;
        let direction = self.memory.direction_buttons;
        let mut new_action = action;
        let mut new_direction = direction;
        //get all 8 button inputs:
        let key_map: [(KeyCode, u8); 8] =  {
            [
                (KeyCode::Right, 0),
                (KeyCode::Left, 1),
                (KeyCode::Up, 2),
                (KeyCode::Down, 3),
                (KeyCode::A, 4),
                (KeyCode::B, 5),
                (KeyCode::Tab, 6),
                (KeyCode::Enter, 7),
            ]
        };

        for (key, bit) in key_map.iter() {
            if is_key_down(*key) {
                log::info!("Key pressed: {:?}", key);
                if bit < &4 {
                    new_direction &= !(1 << bit);
                }else {
                    new_action &= !(1 << (bit%4));
                }
                 
            } else {
                if bit < &4 {
                    new_direction |= 1 << bit;
                }else {
                    new_action |= 1 << (bit%4);
                }
            }
        }
        // save current button states
        self.memory.action_buttons = new_action;
        self.memory.direction_buttons = new_direction;
        //maybe update joypadbyte in memory?
        let mut result = false;
        let selected  = self.memory.read_byte(JOYPAD_REGISTER) & 0x30;
        if selected == 0x10 { //bit 5 = action buttons
            result = action != new_action;
            self.memory.write_controller_byte(selected | new_action);

        }else if  selected == 0x20 { //bit 4 = direction buttons
            result = direction != new_direction;
            self.memory.write_controller_byte(selected | new_direction);

        }
        //joypad interrupt might not be working as intended?
        // If the joypad selects have changed, we need to set the joypad interrupt flag
        if result {
            if self.stop_mode {
                self.stop_mode = false;
            } else {               
                self.set_interrupt_flag(InterruptTypes::Joypad);
            }
        }
        result
    }

    pub fn enable_buttons_debug(&mut self) {
        let mut joypad = self.memory.read_byte(JOYPAD_REGISTER);
        // Enable button by setting the 5th bit to 0
        joypad &= 0b1101_1111;
        self.memory.write_controller_byte(joypad);
    }
}