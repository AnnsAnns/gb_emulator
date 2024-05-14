#[cfg(test)]
pub mod test_helpers;

pub mod memory;
pub mod cpu;
pub mod rendering;

#[macroquad::main("GB Emulator")]
async fn main() {
    println!("Hello, world!");

    let mut cpu = cpu::CPU::new();
    cpu.set_zero_flag();
}