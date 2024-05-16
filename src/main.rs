#[cfg(test)]
pub mod test_helpers;

pub mod cpu;
pub mod memory;
pub mod rendering;

use std::{intrinsics::breakpoint, thread::sleep, time::Duration};

use macroquad::{prelude::*, ui::root_ui};
use rendering::{render_settings::*, tiles::*, views::*};

use crate::{
    cpu::registers::{Register16Bit, Register8Bit},
    rendering::utils::draw_scaled_text,
};

#[macroquad::main("GB Emulator")]
async fn main() {
    println!("Hello, world!");

    let gb_settings = GbSettings {
        ..Default::default()
    };

    let mut tile_atlas = Image::gen_image_color(8 * 16, 8 * 24, WHITE);
    let combined_image = Image::gen_image_color(160, 144, GREEN);

    let mut cpu = cpu::CPU::new();
    cpu.load_from_file("./test_roms/cpu_instrs.gb");

    #[rustfmt::skip]
    let test_tile: [u8; 16] = [
        0xFF, 0x00, 0x7E, 0xFF, 0x85, 0x81, 0x89, 0x83, 
        0x93, 0x85, 0xA5, 0x8B, 0xC9, 0x97, 0x7E, 0xFF
    ];

    request_new_screen_size(
        (160.0 + 8.0 * 16.0) * gb_settings.scaling + 15.0,
        (10.0 * 24.0) * gb_settings.scaling + 25.0,
    );

    loop {
        let pc = cpu.get_16bit_register(Register16Bit::PC);
        let sp = cpu.get_16bit_register(Register16Bit::SP);

        update_tile_atlas(1, 1, &test_tile, &mut tile_atlas, &gb_settings.palette);

        draw_gb_display(5.0, 5.0, &combined_image, &gb_settings);

        draw_tile_viewer(
            combined_image.width() as f32 * gb_settings.scaling + 10.0,
            5.0,
            &tile_atlas,
            &gb_settings,
        );

        root_ui().label(None, format!("PC: {:#06X}", pc).as_str());
        root_ui().label(None, format!("SP: {:#06X}", sp).as_str());
        let instruction = cpu.prepare_and_decode_next_instruction();
        root_ui().label(None, format!("Instruction: {:?}", instruction).as_str());
        root_ui().label(
            None,
            format!("Last Step Result: {:?}", cpu.step()).as_str(),
        );

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

        root_ui().label(
            None,
            format!(
                "AF: {:#06X} BC: {:#06X} DE: {:#06X} HL: {:#06X}",
                cpu.get_16bit_register(Register16Bit::AF),
                cpu.get_16bit_register(Register16Bit::BC),
                cpu.get_16bit_register(Register16Bit::DE),
                cpu.get_16bit_register(Register16Bit::HL)
            )
            .as_str(),
        );

        // root_ui().label(
        //     None,
        //     format!("Memory: {:#?}", cpu.get_memory().return_full_memory()).as_str(),
        // );

        next_frame().await;

        sleep(Duration::from_millis(3000));
    }
}

#[cfg(test)]
mod test_tile_viewer;
