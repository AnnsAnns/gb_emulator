use macroquad::prelude::*;
use rendering::{
    render_settings::*, tiles::*, views::*
};

pub mod memory;
pub mod cpu;
pub mod rendering;


#[macroquad::main("GB Emulator")]
async fn main() {
    println!("Hello, world!");

    let mut cpu = cpu::CPU::new();
    cpu.set_zero_flag();
}

#[cfg(test)]
mod test_tile_viewer;