#[cfg(test)]
pub mod test_helpers;

pub mod cpu;
pub mod memory;
pub mod rendering;

use std::{
    sync::{Arc, Mutex},
    thread::{self, sleep},
    time::{self, Duration},
};

use cpu::CPU;
use macroquad::{prelude::*, ui::root_ui};
use rendering::{tiles::*, views::*};

#[macro_use]
extern crate simple_log;

use crate::{
    cpu::registers::{Register16Bit, Register8Bit},
    memory::raw_memory_operations::test_helper::file_to_memory,
    rendering::utils::draw_scaled_text,
};

/// 60Hz
/// This is the refresh rate of the Gameboy
const FRAME_TIME: f32 = 30.0;

/// UI Thread
/// This thread is responsible for rendering the UI
async fn ui_thread(cpu: Arc<Mutex<CPU>>) {
    // Inititalize General Settings
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

    loop {
        // Create a copy of the memory to avoid locking the mutex for too long
        let memory = cpu.lock().unwrap().get_memory();

        //update_tile_atlas(9, &test_tile, &mut tile_atlas, &PALETTE);
        update_atlas_from_memory(&memory, 16 * 24, &mut tile_atlas, &PALETTE);
        update_background_from_memory(&memory, &tile_atlas, &mut background_image);

        emulation_controls.draw();

        background_viewer.draw(&background_image);

        gb_display.draw(&final_image);

        tile_viewer.draw(&tile_atlas);

        next_frame().await;
    }
}

#[macroquad::main("GB Emulator")]
async fn main() {
    simple_log::quick!();

    let mut cpu = Arc::new(Mutex::new(cpu::CPU::new(false)));
    
    file_to_memory(&mut cpu.get_memory(), 0x8000, "test_files/Mindy1-VRAM.bin");

    // Spawn the UI thread
    let ui_cpu_ref = cpu.clone();
    let ui_handle = async move {
        ui_thread(ui_cpu_ref).await;
    };

    loop {
        let mut cpu = cpu.lock().unwrap();
        let instruction = cpu.prepare_and_decode_next_instruction();
        let result = cpu.step();
        cpu.update_key_input();

        sleep(Duration::from_millis(50));
    }
}
