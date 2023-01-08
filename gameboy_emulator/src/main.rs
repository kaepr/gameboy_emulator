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

    let rom = Rom::new(cli.path.to_string());

    let cart = Cartridge::new(rom.data.clone());

    let mut cpu = CPU::new(cart);

    loop {
        if !(true) {
            break;
        }

        cpu.step();
    }
}
