use gameboy_emulator_lib::{cartridge::Cartridge, cpu::CPU};

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

    let cart = Cartridge::new(cli.path.to_string());

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
