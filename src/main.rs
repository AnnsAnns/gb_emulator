#[cfg(test)]
pub mod test_helpers;

pub mod cpu;
pub mod mmu;
pub mod rendering;

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

    request_new_screen_size(gb_display_size.x + 10.0, gb_display_size.y + 100.0);

    let rom: Vec<u8> = include_bytes!("../assets/roms/Mindy.gb").to_vec();

    let mut cpu = cpu::CPU::new(rom);
    let mut ppu = line_rendering::Ppu::new();

    cpu.skip_boot_rom();

    let mut player_input = cpu::joypad::PlayerInput {
        up: false,
        down: false,
        left: false,
        right: false,
        a: false,
        b: false,
        start: false,
        select: false,
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
                cpu.poll_inputs(&player_input);
                cpu.blarg_print();

                gb_display.draw(&final_image);
                next_frame().await;

                let keys_down = get_keys_down();
                player_input = cpu::joypad::PlayerInput {
                    up: keys_down.contains(&KeyCode::Up),
                    down: keys_down.contains(&KeyCode::Down),
                    left: keys_down.contains(&KeyCode::Left),
                    right: keys_down.contains(&KeyCode::Right),
                    a: keys_down.contains(&KeyCode::A),
                    b: keys_down.contains(&KeyCode::S),
                    start: keys_down.contains(&KeyCode::Enter),
                    select: keys_down.contains(&KeyCode::Tab),
                };
            }
        }
    }
}