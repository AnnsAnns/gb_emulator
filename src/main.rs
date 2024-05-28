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

#[macro_use]
extern crate simple_log;

use crate::{
    cpu::registers::{Register16Bit, Register8Bit},
    rendering::utils::draw_scaled_text,
};

/// 60Hz
/// This is the refresh rate of the Gameboy
const TIME_PER_FRAME: f32 = 1.0/60.0*1000.0;

#[macroquad::main("GB Emulator")]
async fn main() {
    simple_log::quick!();

    log::info!("Hello, world!");

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

    let mut emulation_controls = EmulationControls::new(5.0, gb_display_size.y + 10.0, 1.0);

    request_new_screen_size(
        background_viewer_size.x + tile_viewer_size.x + gb_display_size.x + 20.0,
        tile_viewer_size.y + 10.0,
    );

    let mut cpu = cpu::CPU::new(true);

    cpu.load_from_file("./game.gb");

    // Get start time
    let mut ppu_time = time::Instant::now();
    let mut dump_time = time::Instant::now();
    let mut frame = 0;

    loop {
        let instruction = cpu.prepare_and_decode_next_instruction();
        log::debug!("🔠 Instruction: {:?}", instruction);
        let is_bootrom_enabled = cpu.is_boot_rom_enabled();
        let result = cpu.step();
        log::debug!("➡️ Result: {:?} | Bootrom: {:?}", result, is_bootrom_enabled);

        let pc_following_word = cpu.get_memory().read_word(cpu.get_16bit_register(Register16Bit::PC) + 1);
        log::debug!("🔢 Following Word (PC): {:#06X}", pc_following_word);

        cpu.update_key_input();

        // Draw at 60Hz so 60 frames per second
        if (ppu_time.elapsed().as_millis() as f32) >= TIME_PER_FRAME {
            // Inform about the time it took to render the frame
            root_ui().label(None, format!("⏱️ Frame time: {:?} | Target: {:?} | Frame: {:?}", ppu_time.elapsed(), TIME_PER_FRAME, frame).as_str());
            ppu_time = time::Instant::now();
            update_atlas_from_memory(&cpu.get_memory(), 16 * 24, &mut tile_atlas, &PALETTE);
            update_background_from_memory(&cpu.get_memory(), &tile_atlas, &mut background_image);

            emulation_controls.draw();

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