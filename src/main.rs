#[cfg(test)]
pub mod test_helpers;

pub mod cpu;
pub mod memory;
pub mod rendering;

use std::{
    f32::consts::E,
    io::Write,
    thread::sleep,
    time::{self, Duration},
};

use macroquad::{prelude::*, ui::root_ui};
use rendering::{tiles::*, views::*};
use simple_log::LogConfigBuilder;

#[macro_use]
extern crate simple_log;

use crate::{
    cpu::registers::{Register16Bit, Register8Bit},
    rendering::utils::draw_scaled_text,
};

/// 60Hz
/// This is the refresh rate of the Gameboy
const TIME_PER_FRAME: f32 = 1.0 / 60.0 * 1000.0;
const DUMP_GAMEBOY_DOCTOR_LOG: bool = true;

#[macroquad::main("GB Emulator")]
async fn main() {
    // Set up logging
    let config = LogConfigBuilder::builder()
        .size(1 * 100)
        .roll_count(10)
        .level("info")
        .output_console()
        .build();
    simple_log::new(config).unwrap();

    const PALETTE: [Color; 4] = [
        Color::new(1.00, 1.00, 1.00, 1.00),
        Color::new(0.18, 0.83, 0.18, 1.00),
        Color::new(0.12, 0.54, 0.12, 1.00),
        Color::new(0.06, 0.15, 0.06, 1.00),
    ];
    const SCALING: f32 = 4.0;

    let final_image = Image::gen_image_color(160, 144, GREEN);
    let mut gb_display = GbDisplay {
        offset_x: 5.0,
        offset_y: 5.0,
        scaling: SCALING,
    };
    let gb_display_size = gb_display.size(&final_image);

    let mut background_viewer = BackgroundViewer {
        offset_x: gb_display_size.x + 10.0,
        offset_y: 5.0,
        scaling: SCALING / 2.0,
    };
    let mut background_image = Image::gen_image_color(32 * 8, 32 * 8, PINK);
    let background_viewer_size = background_viewer.size();

    let mut tile_atlas = Image::gen_image_color(8 * 16, 8 * 24, WHITE);
    let mut tile_viewer = TileViewer {
        offset_x: gb_display_size.x + background_viewer_size.x + 15.0,
        offset_y: 5.0,
        scaling: SCALING,
    };
    let tile_viewer_size = tile_viewer.size();

    request_new_screen_size(
        background_viewer_size.x + tile_viewer_size.x + gb_display_size.x + 20.0,
        tile_viewer_size.y + 10.0,
    );

    let mut cpu = cpu::CPU::new(true);

    cpu.load_from_file("./game.gb");

    // Get start time
    let mut ppu_time = time::Instant::now();
    let mut ppu_h_time = time::Instant::now(); // Horizontal time
    let mut dump_time = time::Instant::now();
    let mut frame = 0;
    let mut h_timeslots = TIME_PER_FRAME / 153.0;
    let mut y_coordinate: u8 = 0;

    // Open "registers.txt" file for Gameboy Doctor
    let mut gb_doctor_file = std::fs::File::create("gameboy_doctor_log.txt").unwrap();

    if DUMP_GAMEBOY_DOCTOR_LOG {
        cpu.skip_boot_rom();
    }

    loop {
        if DUMP_GAMEBOY_DOCTOR_LOG {
            // Dump registers to file for Gameboy Doctor like this
            // A:00 F:11 B:22 C:33 D:44 E:55 H:66 L:77 SP:8888 PC:9999 PCMEM:AA,BB,CC,DD
            let _ = gb_doctor_file.write_all(
                format!(
                    "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}\n",
                    cpu.get_8bit_register(Register8Bit::A),
                    cpu.flags_to_u8(),
                    cpu.get_8bit_register(Register8Bit::B),
                    cpu.get_8bit_register(Register8Bit::C),
                    cpu.get_8bit_register(Register8Bit::D),
                    cpu.get_8bit_register(Register8Bit::E),
                    cpu.get_8bit_register(Register8Bit::H),
                    cpu.get_8bit_register(Register8Bit::L),
                    cpu.get_16bit_register(Register16Bit::SP),
                    cpu.get_16bit_register(Register16Bit::PC),
                    cpu.get_memory().read_byte(cpu.get_16bit_register(Register16Bit::PC)),
                    cpu.get_memory().read_byte(cpu.get_16bit_register(Register16Bit::PC) + 1),
                    cpu.get_memory().read_byte(cpu.get_16bit_register(Register16Bit::PC) + 2),
                    cpu.get_memory().read_byte(cpu.get_16bit_register(Register16Bit::PC) + 3),
                )
                .as_bytes(),
            );
        }

        let instruction = cpu.prepare_and_decode_next_instruction();
        log::debug!("🔠 Instruction: {:?}", instruction);
        let is_bootrom_enabled = cpu.is_boot_rom_enabled();
        let result = cpu.step();
        log::debug!("➡️ Result: {:?} | Bootrom: {:?}", result, is_bootrom_enabled);

        let pc_following_word = cpu
            .get_memory()
            .read_word(cpu.get_16bit_register(Register16Bit::PC) + 1);
        log::debug!("🔢 Following Word (PC): {:#06X}", pc_following_word);

        cpu.update_key_input();

        // Set the LCD Y coordinate
        // This is a hack to get the LCD interrupts to work
        // Without a proper PPU implementation
        if (ppu_h_time.elapsed().as_millis() as f32) >= h_timeslots {
            y_coordinate = if y_coordinate == 153 {
                0
            } else {
                y_coordinate + 1
            };

            cpu.set_lcd_y_coordinate(y_coordinate);
            ppu_h_time = time::Instant::now();

            // Set the PPU mode
            let mode = if y_coordinate >= 144 {
                1
            } else {
                // Random mode either 2, 3 or 0
                // Because we don't have a proper PPU implementation
                (y_coordinate % 4) + 2
            };

            cpu.set_ppu_mode(mode);
        }

        // Draw at 60Hz so 60 frames per second
        if (ppu_time.elapsed().as_millis() as f32) >= TIME_PER_FRAME {
            // Inform about the time it took to render the frame
            root_ui().label(
                None,
                format!(
                    "Frame time: {:?} | Target: {:?} | Frame: {:?}",
                    ppu_time.elapsed(),
                    TIME_PER_FRAME,
                    frame
                )
                .as_str(),
            );
            ppu_time = time::Instant::now();
            update_atlas_from_memory(&cpu.get_memory(), 16 * 24, &mut tile_atlas, &PALETTE);
            update_background_from_memory(&cpu.get_memory(), &tile_atlas, &mut background_image);

            background_viewer.draw(&background_image);

            gb_display.draw(&final_image);

            tile_viewer.draw(&tile_atlas);
            next_frame().await;
            // Set the VBlank interrupt since we are done with the frame
            cpu.set_vblank_interrupt();
            frame += 1;
        }

        // Dump memory every 3 seconds
        if dump_time.elapsed().as_secs() >= 3 {
            dump_time = time::Instant::now();
            cpu.dump_memory();
        }
    }
}
