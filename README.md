# Gameboy Emulator in Rust

Work in progress

## Run Instructions

```bash
cargo run -- -p "relative path to rom" 
```

## Depdencies
- [rust_minifb](https://github.com/emoon/rust_minifb) Framebuffer for displaying the lcd output

## Test roms
Blarrg's test roms can be found at [gb-test-roms](https://github.com/retrio/gb-test-roms.git).

## Tasks

- [x] Pass all individual CPU instruction tests
- [ ] Run using WASM
- [ ] Display on browser
- [ ] Pass display tests

