#[cfg(test)]
pub mod test_helpers;

pub mod memory;
pub mod cpu;
pub mod rendering;

use macroquad::prelude::*;
use rendering::{
    render_settings::*, tiles::*, views::*
};

use crate::cpu::registers::Register16Bit;


#[macroquad::main("GB Emulator")]
async fn main() {   
    println!("Hello, world!");

    let mut cpu = cpu::CPU::new();
    cpu.load_from_file("./test_roms/cpu_instrs.gb");
    loop {
        cpu.prepare_and_decode_next_instruction();
        println!("➡️ {:#02X}: Next Instruction: {:?}", cpu.get_16bit_register(Register16Bit::PC), cpu.get_instruction());
        cpu.step();
        println!("🏁 Last Step Result: {:?}", cpu.get_last_step_result());
    }
}

#[cfg(test)]
mod test_tile_viewer;