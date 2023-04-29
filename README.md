# Gameboy Emulator in Rust

Work in progress

## Run Instructions

```bash
cargo run -- -p "relative path to rom" 
```

## Default Controls
- W A S D | Up Left Down Right
- J K | A B
- U | Select 
- I | Start

## Dependencies
- [rust_minifb](https://github.com/emoon/rust_minifb) Framebuffer for displaying the lcd output and keyboard input

## Test roms
Blarrg's test roms can be found at [gb-test-roms](https://github.com/retrio/gb-test-roms.git).

## Tasks

- [x] Pass all individual CPU instruction tests
- [ ] Run using WASM
- [ ] Display on browser
- [ ] Pass display tests

