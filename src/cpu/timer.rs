use super::{interrupts::InterruptTypes, CPU};

const TIMER_COUNTER_ADDRESS: u16 = 0xFF05;
const TIMER_MODULO_ADDRESS: u16 = 0xFF06;

impl CPU {
    pub fn increment_timer(&mut self) {
        // Check whether FF07 is enabled [Pos 2]
        if self.memory.read_byte(0xFF07) & 0b100 == 0 {
            return;
        }
        let previous_val = self.memory.read_byte(TIMER_COUNTER_ADDRESS);
        let (new_val, overflow) = previous_val.overflowing_add(1);

        if overflow {
            log::debug!("Timer overflow - resetting to modulo & setting interrupt flag");
            self.memory.write_byte(TIMER_COUNTER_ADDRESS, self.memory.read_byte(TIMER_MODULO_ADDRESS));
            self.set_interrupt_flag(InterruptTypes::Timer);
        } else {
            self.memory.write_byte(TIMER_COUNTER_ADDRESS, new_val);
        }
    }

    /// Get the timer modulo based on the timer speed
    pub fn get_timer_modulo(&mut self) -> u64 {
        let timer_speed = self.memory.read_byte(0xFF07) & 0b11;

        match timer_speed {
            0b00 => 256,
            0b01 => 4,
            0b10 => 16,
            0b11 => 64,
            _ => panic!("Invalid timer speed"),
        }
    }
}