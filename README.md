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

## Credits and Resoures Used

- Testroms are provided by Shay Green <gblargg@gmail.com>
- [Opcode table](https://gbdev.io/gb-opcodes//optables/)
- [RGBDS](https://rgbds.gbdev.io/docs/v0.7.0/gbz80.7)