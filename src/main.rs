#[cfg(test)]
pub mod test_helpers;

pub mod cpu;
pub mod rendering;
pub mod mmu;

use std::{fs::File, io::Write, ops::Sub, thread, time};

use cpu::CPU;
use macroquad::{prelude::*, ui::root_ui};
use mmu::MemoryOperations;
use rendering::{
    line_rendering::{self},
    tiles::{self, *},
    views::*,
};
use rfd::FileDialog;
use simple_log::LogConfigBuilder;

extern crate simple_log;

use crate::cpu::registers::{Register16Bit, Register8Bit};

const TIME_PER_FRAME: f32 = 1000.0 / 59.73;

const DUMP_GAMEBOY_DOCTOR_LOG: bool = false;
#[cfg(target_os = "linux")]
const WINDOWS: bool = false;
#[cfg(target_os = "windows")]
const WINDOWS: bool = true;

#[macroquad::main("GB Emulator")]
async fn main() {
    //Set up logging
    let config = LogConfigBuilder::builder()
        .size(1000)
        .roll_count(10)
        .level("info")
        .output_console()
        .build();
    simple_log::new(config).unwrap();

    const PALETTE: [Color; 4] = [
        Color::new(232.0/255.0, 252.0/255.0, 204.0/255.0, 1.00),
        Color::new(172.0/255.0, 212.0/255.0, 144.0/255.0, 1.00),
        Color::new(084.0/255.0, 140.0/255.0, 112.0/255.0, 1.00),
        Color::new(020.0/255.0, 044.0/255.0, 056.0/255.0, 1.00),
    ];
    const SCALING: f32 = 4.0;

    let mut gb_display = GbDisplay::new(5.0, 5.0, SCALING);
    let mut background_viewer = BackgroundViewer::new(gb_display.size().x + 10.0, 5.0, SCALING / 2.0);
    let mut tile_viewer = TileViewer::new(gb_display.size().x + background_viewer.size().x + 15.0, 5.0, SCALING);

    request_new_screen_size(
        background_viewer.size().x + tile_viewer.size().x + gb_display.size().x + 20.0,
        tile_viewer.size().y + 10.0,
    );

    let filedialog = FileDialog::new()
        .add_filter("gb", &["gb"])
        .set_title("Select a Gameboy ROM")
        // Set directory to the current directory
        .set_directory(std::env::current_dir().unwrap())
        .pick_file()
        .unwrap();

    let filepath = filedialog.as_path().to_str();

    let rom = std::fs::read(filepath.expect("No file was found")).expect("Unable to read file");

    let mut cpu = cpu::CPU::new(rom);
    let mut ppu = line_rendering::Ppu::new();

    // Get start time
    let mut last_frame_time = time::Instant::now();
    let mut ppu_time = time::Instant::now();
    let mut fps_time = time::Instant::now();
    let mut fps = 0;
    let mut dump_time = time::Instant::now();
    let mut frame = 0;

    // Open "registers.txt" file for Gameboy Doctor
    let mut gb_doctor_file = std::fs::File::create("gameboy_doctor_log.txt").unwrap();
    if DUMP_GAMEBOY_DOCTOR_LOG {
        cpu.skip_boot_rom();
    }

    cpu.set_ppu_mode(cpu::interrupts::PpuMode::OamScan);

    loop {
        // Check whether PC is at the end of the bootrom
        if cpu.get_16bit_register(Register16Bit::PC) == 0x0100 {
            log::info!("🚀 Bootrom finished");
            cpu.skip_boot_rom();
        }

        cpu.increment_div();

        if DUMP_GAMEBOY_DOCTOR_LOG {
            dump_cpu_info(&cpu, &mut gb_doctor_file);
        }

        let instruction = cpu.prepare_and_decode_next_instruction();
        log::debug!("🔠 Instruction: {:?}", instruction);
        let is_bootrom_enabled = cpu.is_boot_rom_enabled();
        let result = cpu.step();
        match result {
            Ok(_) => {}
            Err(e) => {
                log::error!("❌ Error: {:?} | Info: {}", e, info_to_string(&cpu));
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
            ppu.step(&mut cpu, &mut gb_display.get_gb_image(), &PALETTE);

            // Alternatively Redraw UI at 30 frames per second: (ppu_time.elapsed().as_millis() as f32) >= TIME_PER_FRAME
            // Draw when a frame is done
            if ppu.get_frame_cycles() == 0 {
                // Check whether 1 second has passed to update the FPS
                if fps_time.elapsed().as_secs() >= 1 {
                    fps_time = time::Instant::now();
                    fps = frame;
                    frame = 0;
                }

                // Poll inputs
                cpu.poll_inputs();
                cpu.blarg_print();
                ppu_time = time::Instant::now();

                // Inform about the time it took to render the frame
                root_ui().label(
                None,
                format!(
                    "FPS: {:?} | Dots: {:?} | CPU Cycle: {:?} | Frame: {:?}",
                    fps,
                    ppu.get_dot(),
                    cpu.get_cycles(),
                    frame,
                )
                .as_str(),
                );

                // Update Debugging Views
                update_atlas_from_memory(&cpu, 16 * 24, &mut tile_viewer.get_atlas(), &PALETTE);
                update_background_from_memory(&cpu, &mut background_viewer.get_image(), &PALETTE, false, true);
                background_viewer.draw();
                tile_viewer.draw();

                gb_display.draw();
                next_frame().await;
                frame += 1;

                thread::sleep(time::Duration::from_millis(
                    (TIME_PER_FRAME - last_frame_time.elapsed().as_millis() as f32) as u64,
                ));
                last_frame_time = time::Instant::now();
            }
        }
    }
}

fn info_to_string(cpu: &CPU) -> String {
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
        cpu.mmu.read_byte(cpu.get_16bit_register(Register16Bit::PC)),
        cpu.mmu.read_byte(cpu.get_16bit_register(Register16Bit::PC) + 1),
        cpu.mmu.read_byte(cpu.get_16bit_register(Register16Bit::PC) + 2),
        cpu.mmu.read_byte(cpu.get_16bit_register(Register16Bit::PC) + 3),
    )
}

fn dump_cpu_info(cpu: &CPU, destination: &mut File) {
    // Dump registers to file for Gameboy Doctor like this
    // A:00 F:11 B:22 C:33 D:44 E:55 H:66 L:77 SP:8888 PC:9999 PCMEM:AA,BB,CC,DD
    let _ = destination.write_all(
                    info_to_string(cpu)
                    .as_bytes(),
                );
}
