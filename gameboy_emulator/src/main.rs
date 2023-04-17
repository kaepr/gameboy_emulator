use clap::Parser;
use gameboy_emulator_lib::{cartridge::Cartridge, rom::Rom, utils::Opts};

mod args;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use args::Args;
    use gameboy_emulator_lib::emu::EmuContext;

    let args = Args::parse();

    let rom = Rom::new(args.path.to_string());

    let cart = Cartridge::new(rom.data);

    let opts = Opts::new(args.debug, args.serial);

    let mut ctx = EmuContext::new(cart, opts);

    loop {
        ctx.step();
    }
}
