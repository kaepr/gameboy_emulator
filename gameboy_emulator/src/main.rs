use gameboy_emulator_lib::{cartridge::Cartridge, cpu::CPU};

fn main() {
    println!("Gameboy Emulator !");

    let rom_path = "../../projects/test_roms/07-jr,jp,call,ret,rst.gb";

    let cart = Cartridge::new(rom_path.to_string());

    cart.stat();

    let mut cpu = CPU::new();

    cpu.bus.load_cart(&cart);

    loop {
        if !(true) {
            break;
        }

        cpu.step();
    }
}
