use gameboy_emulator_lib::cartridge::{Cartridge};

fn main() {
    println!("Gameboy emulator !");
        
    let rom_path = "../../projects/test_roms/Tetris.gb";

    let cart = Cartridge::new(rom_path.to_string());

    cart.data.iter().take(20).for_each(|x| {
        println!("{}", x);
    });
}
