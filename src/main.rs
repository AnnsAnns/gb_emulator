#[cfg(test)]
pub mod test_helpers;

pub mod cpu;
pub mod memory;
pub mod rendering;

use std::{
    thread::sleep,
    time::{self, Duration},
};

use macroquad::{prelude::*, ui::root_ui};
use rendering::{tiles::*, views::*};
use simple_logger::SimpleLogger;

use crate::{
    cpu::registers::{Register16Bit, Register8Bit},
    rendering::utils::draw_scaled_text,
};

const FRAME_TIME: f32 = 5.0;

#[macroquad::main("GB Emulator")]
async fn main() {
    SimpleLogger::new().init().unwrap();

    log::info!("Hello, world!");

    const PALETTE: [Color; 4] = [
        Color::new(1.00, 1.00, 1.00, 1.00),
        Color::new(0.18, 0.83, 0.18, 1.00),
        Color::new(0.12, 0.54, 0.12, 1.00),
        Color::new(0.06, 0.15, 0.06, 1.00),
    ];
    const SCALING: f32 = 4.0;

    let mut tile_atlas = Image::gen_image_color(8 * 16, 8 * 24, WHITE);
    let combined_image = Image::gen_image_color(160, 144, GREEN);

    let mut cpu = cpu::CPU::new(true);

    // Check whether DrMario.gb exists otherwise use the test ROM
    if std::fs::metadata("./game.gb").is_err() {
        cpu.load_from_file("./test_data/cpu_instrs/individual/09-op r,r.gb");
    } else {
        cpu.load_from_file("./game.gb");
    }

    request_new_screen_size(
        160.0 * SCALING,
        144.0 * SCALING,
    );

    let mut frame_counter = 0;

    let final_image = Image::gen_image_color(160, 144, GREEN);
    let mut gb_display = GbDisplay {offset_x: 0.0, offset_y: 0.0, scaling: SCALING};

    loop {
        let pc = cpu.get_16bit_register(Register16Bit::PC);
        let sp = cpu.get_16bit_register(Register16Bit::SP);

        gb_display.draw(&final_image);

        // Get start time
        let start_time = time::Instant::now();

        let instruction = cpu.prepare_and_decode_next_instruction();
        root_ui().label(None, format!("Instruction: {:?}", instruction).as_str());
        let result = cpu.step();

        log::debug!("➡️ Result: {:?}", result);

        match result {
            Ok(result) => {
                root_ui().label(
                    None,
                    format!(
                        "Step Result: Cycles: {} | Bytes: {}",
                        result.cycles, result.bytes
                    )
                    .as_str(),
                );
            }
            Err(error) => {
                root_ui().label(None, format!("Error: {:?}", error).as_str());
            }
        }

        root_ui().label(
            None,
            format!(
                "Flags - Zero {:?} Carry {:?} Sub {:?} HalfCarry {:?}",
                cpu.is_zero_flag_set(),
                cpu.is_carry_flag_set(),
                cpu.is_subtraction_flag_set(),
                cpu.is_half_carry_flag_set()
            )
            .as_str(),
        );

        root_ui().label(
            None,
            format!(
                "A: {:#04X} B: {:#04X} C: {:#04X} D: {:#04X} E: {:#04X} H: {:#04X} L: {:#04X}",
                cpu.get_8bit_register(Register8Bit::A),
                cpu.get_8bit_register(Register8Bit::B),
                cpu.get_8bit_register(Register8Bit::C),
                cpu.get_8bit_register(Register8Bit::D),
                cpu.get_8bit_register(Register8Bit::E),
                cpu.get_8bit_register(Register8Bit::H),
                cpu.get_8bit_register(Register8Bit::L)
            )
            .as_str(),
        );

        let pc_following_word = cpu.get_memory().read_word(cpu.get_16bit_register(Register16Bit::PC) + 1);
        log::debug!("🔢 Following Word (PC): {:#06X}", pc_following_word);
        root_ui().label(
            None,
            format!(
                "AF: {:#06X} BC: {:#06X} DE: {:#06X} HL: {:#06X} SP: {:#06X} PC: {:#06X} Following Word: {:#06X}",
                cpu.get_16bit_register(Register16Bit::AF),
                cpu.get_16bit_register(Register16Bit::BC),
                cpu.get_16bit_register(Register16Bit::DE),
                cpu.get_16bit_register(Register16Bit::HL),
                sp,
                pc,
                pc_following_word
            )
            .as_str(),
        );

        // root_ui().label(
        //     None,
        //     format!("Memory: {:#?}", cpu.get_memory().return_full_memory()).as_str(),
        // );

        next_frame().await;

        // Dump memory for debugging purposes (only every 60 frames)
        if frame_counter % 60 == 0 {
            cpu.dump_memory();
        }

        let elapsed_time = start_time.elapsed();
        // We run at 60Hz so we need to calculate the time we need to sleep
        let time_to_sleep =
            match Duration::from_secs_f32(1.0 / FRAME_TIME).checked_sub(elapsed_time) {
                Some(time) => time,
                None => Duration::from_secs_f32(0.0),
            };
        log::debug!(
            "⌛ Time to sleep: {:?} | Total Duration was {:?}",
            time_to_sleep,
            elapsed_time
        );
        root_ui().label(
            None,
            format!(
                "Free Time: {:.2}ms - Render Time: {:.2?}ms",
                time_to_sleep.as_micros() as f64 / 1000.0,
                elapsed_time.as_micros() as f64 / 1000.0
            )
            .as_str(),
        );
        sleep(time_to_sleep);

        frame_counter += 1;
    }
}