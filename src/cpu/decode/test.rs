#[cfg(test)]
use crate::cpu::{registers::Register16Bit, CPU};
#[cfg(test)]
use crate::mmu::MemoryOperations;

/// Test the decoding of all opcodes
/// This will print out all decoded values and failed values
/// The decoded values will be written to `decoded_values.txt` and the failed values to `failed_values.txt`
/// This is useful to check if all opcodes are correctly decoded
/// Warning: This doesn't check if the decoded values are correct, only if they are decoded
/// Please cross-reference with a Gameboy opcode table
#[test]
pub fn test_decode() {
    let mut cpu = CPU::new(Vec::new());
    let mut decoded_values = String::new();
    let mut failed_values = String::new();

    for i in 0..=0xFF {
        // Write the opcode for 0xCB to memory
        cpu.mmu
            .write_byte(cpu.get_16bit_register(Register16Bit::SP) + 1, i);

        for opcode in [i, 0xCB] {
            let decoded_value = cpu.decode(opcode);

            let opcode = if opcode == 0xCB {
                format!("0xCB | {:#02X}", i)
            } else {
                format!("{:#02X}", i)
            };

            if let Ok(val) = decoded_value {
                decoded_values.push_str(&format!("{} -> {:?}\n", opcode, val));
            } else {
                failed_values.push_str(&format!(
                    "{} -> {:?}\n",
                    opcode,
                    decoded_value.unwrap_err()
                ));
            }
        }
    }

    println!("🟢 Decoded values: {:?}", decoded_values);
    println!("🔴 Failed values: {:?}", failed_values);

    // To file

    use std::fs::File;
    use std::io::Write;

    let mut file = File::create("decoded_values.txt").unwrap();
    file.write_all(decoded_values.as_bytes()).unwrap();
    let mut file = File::create("failed_values.txt").unwrap();
    file.write_all(failed_values.as_bytes()).unwrap();
}