pub mod memory;
pub mod cpu;

fn main() {
    println!("Hello, world!");

    let mut cpu = cpu::CPU::new();
    cpu.set_zero_flag();
}
