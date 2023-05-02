# Gameboy Emulator in Rust

Work in progress

## Run Instructions

```bash
# Running ROM
cargo run -- -p "relative path to rom" 

# List different cli options
cargo run -- --help 
```

## Controls
| Keyboard   |    Input     |
| :--------: | :----------: |
|    W       |   Dpad Up    | 
|    S       |  Dpad Down   |
|    A       |  Dpad Right  |
|    D       |  Dpad Left   |
|    J       |      A       |
|    K       |      B       |
|    U       |   Select     |
|    I       |   Start      |

## Dependencies
- [rust_minifb](https://github.com/emoon/rust_minifb) Framebuffer for displaying the lcd output and keyboard input

## Test roms
Blarrg's test roms can be found at [gb-test-roms](https://github.com/retrio/gb-test-roms.git).

## Tasks

- [x] Pass all individual CPU instruction tests
- [ ] Run using WASM
- [ ] Display on browser
- [ ] Pass display tests

