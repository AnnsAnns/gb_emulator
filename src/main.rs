pub mod memory;
pub mod cpu;
pub mod test_helpers;

fn main() {
    println!("Hello, world!");

    let mut cpu = cpu::CPU::new();
    cpu.set_zero_flag();
}
