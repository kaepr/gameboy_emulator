use gameboy_emulator_lib::{cartridge::Cartridge, cpu::CPU, rom::Rom, utils::Opts};

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Cli {
    /// Relative path to the Gameboy Rom
    #[arg(short, long)]
    path: String,

    /// Show register values
    #[arg(short, long, required = false, default_value_t = false)]
    debug: bool,

    /// Show output in the serial register. Useful for blarrgs's test rom
    #[arg(short, long, required = false, default_value_t = false)]
    serial: bool,
}

fn main() {
    let cli = Cli::parse();

    let rom = Rom::new(cli.path.to_string());

    let cart = Cartridge::new(rom.data);

    let opts = Opts::new(cli.debug, cli.serial);

    let mut cpu = CPU::new(cart, opts);

    loop {
        cpu.step();
    }
}
