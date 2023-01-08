use gameboy_emulator_lib::{cartridge::Cartridge, cpu::CPU, rom::Rom};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// Relative path to the Gameboy Rom
    #[arg(long)]
    path: String,
}

fn main() {
    let cli = Cli::parse();

    println!("Gameboy Emulator !");

    let rom = Rom::new(cli.path.to_string());

    rom.stat();

    let cart = Cartridge::new(rom.data.clone());
    cart.header.print();

    let mut cpu = CPU::new();

    cpu.bus.load_cart(&rom);

    loop {
        if !(true) {
            break;
        }

        cpu.step();
    }
}
