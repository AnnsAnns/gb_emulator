## Gameboy Rust SoSE 24 Project

# [Docs](https://haw-rust-sose24.github.io/gb_emulator/)

This is a Gameboy emulator written in Rust for the Rust WP SoSE 24 project at the HAW Hamburg.

By default, the game will load the game at `./game.gb`. If there is no game at that location, the emulator will load a testrom.

## Building

```bash
cargo build
```

## Running

```bash
cargo run
```

## Using Gameboy Doctor

Gameboy Doctor is a tool that can be used to debug the emulator. It can be found [here](https://github.com/robert/gameboy-doctor), it's **extremely** useful.

### How to install

1. `git clone` the repository (Preferably *not* in the same directory as the emulator)
    2. For example `projects/gameboy-doctor` and `projects/gb_emulator`	
2. Make sure you have python installed

### How to use

1. Pick a test suite from `test_data/individual`
2. Put it at `./game.gb`
3. Run the emulator `cargo run`
    1. Make sure that `DUMP_GAMEBOY_DOCTOR_LOG` is set to `true` in `main.rs`
4. Run the gameboy doctor (from the gameboy-doctor directory)
    1. `python gameboy-doctor ../gb_emulator/gameboy_doctor_log.txt cpu_instrs NUMBER_OF_ROM`
        1. The number of the rom is the number in front of the rom, e.g. 9 for `09-op r,r.gb`
        1. This expects you to have a similar directory structure as mentioned above, otherwise you need to adjust the path
5. The gameboy doctor will output a log file with the results, e.g.:

```
Mismatch in CPU state at line 16520:

MINE:   A:00 F:-H-Z B:01 C:00 D:D0 E:00 H:CE L:46 SP:DFFB PC:CA0E PCMEM:3E,20,CD,37
YOURS:  A:00 F:-H-Z B:01 C:00 D:D0 E:00 H:CE L:46 SP:DFFB PC:CA0B PCMEM:C4,45,CA,3E

The CPU state before this (at line 16519) was:

        A:00 F:A0 B:01 C:00 D:D0 E:00 H:CE L:46 SP:DFFB PC:CA0B PCMEM:C4,45,CA,3E

The last operation executed (in between lines 16519 and 16520) was:

        0xC4 CALL NZ a16
```

## Credits and Resoures Used

- Testroms are provided by Shay Green <gblargg@gmail.com>
- [Opcode table](https://gbdev.io/gb-opcodes//optables/)
- [RGBDS](https://rgbds.gbdev.io/docs/v0.7.0/gbz80.7)
- [Simple Text Print ROM](https://github.com/ISSOtm/gb-vwf?tab=readme-ov-file)
- [Hello World ROM](https://gbdev.io/rgbds-live/)
- [Gameboy Doctor for debugging](https://github.com/robert/gameboy-doctor)
- [GBMicrotest](https://github.com/aappleby/GBMicrotest)

# Authors

### Vincent Adamczyk

- Main Focus: CPU 
    - Focus: Instructions, Decode
    - Basic Knowledge: Flags, Step

### Laurin Zacharias

- Main Focus: CPU
    - Focus: Instructions, Joypad, Step, Debugging
    - Basic Knowledge: Decode

### Michael Vogt

- Main Focus: PPU
    - Rendering Module (src/rendering)
    - CPU Rendering Operations (rendering_operations.rs)

### Tom Hert

- Main Focus: CPU, MMU, Main
    - Focus: 
        - Memory Management Unit (src/mmu)
        - Structs/Enums/Concept of the Emulator (except PPU)
            * e.g. MMU, CPU, Instructions (ENUM not implementations), Registers, Flags
        - CPU (src/cpu)
            * Decode
            * Interrupts
            * Registers
            * DMA
    - Basic Knowledge: Instructions