use macroquad::prelude::*;

use super::CPU;

impl CPU {
    pub fn update_key_input(&mut self) {
        let keys_down = get_keys_down();

        let mut output = 0xFF;

        let joypad_selects = self.memory.read_byte(0xFF00);

        //is button flag on
        if (joypad_selects & 0x20) == 0 {
            output &= !(1 << 5);
            if keys_down.contains(&KeyCode::Enter) {
                //start key
                output &= !(1 << 3);
            } else if keys_down.contains(&KeyCode::Tab) {
                //select key
                output &= !(1 << 2);
            } else if keys_down.contains(&KeyCode::A) {
                output &= !(1 << 0);
            } else if keys_down.contains(&KeyCode::B) {
                output &= !(1 << 1);
            }
        }

        //is direction flag on
        if (joypad_selects & 0x10) == 0 {
            output &= !(1 << 4);
            if keys_down.contains(&KeyCode::Left) {
                output &= !(1 << 1);
            } else if keys_down.contains(&KeyCode::Right) {
                output &= !(1 << 0);
            } else if keys_down.contains(&KeyCode::Up) {
                output &= !(1 << 2);
            } else if keys_down.contains(&KeyCode::Down) {
                output &= !(1 << 3);
            }
        }
        
        self.memory.write_controller_byte(output);
    }

    pub fn get_mem_reg(& self, address: u16) -> u8 {
        self.memory.read_byte(address)
    }
    
}