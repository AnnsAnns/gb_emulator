#[cfg(test)]
pub mod test_helpers;

pub mod cpu;
pub mod mmu;
pub mod rendering;

use std::fmt::format;

use macroquad::prelude::*;
use mmu::MemoryOperations;
use rendering::{
    line_rendering::{self},
    views::*,
};

use crate::cpu::registers::Register16Bit;

#[macroquad::main("GB Emulator")]
async fn main() {
    //Set up logging

    const PALETTE: [Color; 4] = [
        Color::new(232.0 / 255.0, 252.0 / 255.0, 204.0 / 255.0, 1.00),
        Color::new(172.0 / 255.0, 212.0 / 255.0, 144.0 / 255.0, 1.00),
        Color::new(084.0 / 255.0, 140.0 / 255.0, 112.0 / 255.0, 1.00),
        Color::new(020.0 / 255.0, 044.0 / 255.0, 056.0 / 255.0, 1.00),
    ];
    const SCALING: f32 = 4.0;



    let mut final_image = Image::gen_image_color(160, 144, GREEN);
    let mut gb_display = GbDisplay {
        offset_x: 5.0,
        offset_y: 5.0,
        scaling: SCALING,
    };
    let gb_display_size = gb_display.size(&final_image);

    let on_screen_controls = OnScreenControls::new(5.0, gb_display_size.y + 10.0, 1.0);
    let osc_locs = on_screen_controls.get_on_screen_control_locations();

    request_new_screen_size(gb_display_size.x + 10.0, gb_display_size.y * 1.5);

    let rom: Vec<u8> = include_bytes!("../assets/roms/Tetris.gb").to_vec();

    let mut cpu = cpu::CPU::new(rom);
    let mut ppu = line_rendering::Ppu::new();

    cpu.skip_boot_rom();

    let mut player_input = cpu::joypad::PlayerInput {
        ..Default::default()
    };

    loop {
        // Check whether PC is at the end of the bootrom
        if cpu.get_16bit_register(Register16Bit::PC) == 0x0100 {
            log::info!("🚀 Bootrom finished");
            cpu.skip_boot_rom();
        }

        cpu.increment_div();

        let instruction = cpu.prepare_and_decode_next_instruction();
        log::debug!("🔠 Instruction: {:?}", instruction);
        let is_bootrom_enabled = cpu.is_boot_rom_enabled();
        let result = cpu.step();
        match result {
            Ok(_) => {}
            Err(e) => {
                break;
            }
        }
        log::debug!("➡️ Result: {:?} | Bootrom: {:?}", result, is_bootrom_enabled);
        let cpu_cycles_taken = result.unwrap().cycles;

        let pc_following_word = cpu
            .mmu
            .read_word(cpu.get_16bit_register(Register16Bit::PC) + 1);
        log::debug!("🔢 Following Word (PC): {:#06X}", pc_following_word);

        for _ in 0..=cpu_cycles_taken {
            ppu.step(&mut cpu, &mut final_image, &PALETTE);

            // Draw when a frame is done
            if ppu.get_frame_cycles() == 0 {
                // Poll inputs
                cpu.poll_inputs(&player_input.clone());
                cpu.blarg_print();

                gb_display.draw(&final_image);

                let touch_down = touches();

                let mut touch_input = cpu::joypad::PlayerInput {
                    ..Default::default()
                };

                for touch in touch_down {
                    touch_input.right =
                        touch.position.distance(osc_locs.cross_right) < 17.0 * SCALING;
                    touch_input.left =
                        touch.position.distance(osc_locs.cross_left) < 17.0 * SCALING;
                    touch_input.up = touch.position.distance(osc_locs.cross_up) < 17.0 * SCALING;
                    touch_input.down =
                        touch.position.distance(osc_locs.cross_down) < 17.0 * SCALING;
                    touch_input.a = touch.position.distance(osc_locs.a) < 10.0 * SCALING;
                    touch_input.b = touch.position.distance(osc_locs.b) < 10.0 * SCALING;
                    touch_input.select = touch.position.distance(osc_locs.select) < 10.0 * SCALING;
                    touch_input.start = touch.position.distance(osc_locs.start) < 10.0 * SCALING;
                }

                let keys_down = get_keys_down();
                player_input = cpu::joypad::PlayerInput {
                    up: keys_down.contains(&KeyCode::Up) || touch_input.up,
                    down: keys_down.contains(&KeyCode::Down) || touch_input.down,
                    left: keys_down.contains(&KeyCode::Left) || touch_input.left,
                    right: keys_down.contains(&KeyCode::Right) || touch_input.right,
                    a: keys_down.contains(&KeyCode::A) || touch_input.a,
                    b: keys_down.contains(&KeyCode::S) || touch_input.b,
                    start: keys_down.contains(&KeyCode::Enter) || touch_input.start,
                    select: keys_down.contains(&KeyCode::Tab) || touch_input.select,
                };

                on_screen_controls.draw(player_input.clone());

                next_frame().await;
                clear_background(GRAY);
            }
        }
    }
}
