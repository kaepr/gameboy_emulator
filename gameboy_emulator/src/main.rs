use clap::Parser;
use gameboy_emulator_lib::{bus::Memory, cartridge::Cartridge, rom::Rom, utils::Opts};

mod args;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use args::Args;
    use gameboy_emulator_lib::emu::EmuContext;
    use minifb::{Key, Window, WindowOptions};

    const WIDTH: usize = 640;
    const HEIGHT: usize = 360;

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window =
        Window::new("Test", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let args = Args::parse();

    let rom = Rom::new(args.path.to_string());

    let cart = Cartridge::new(rom.data);

    let opts = Opts::new(args.debug, args.serial);

    let mut ctx = EmuContext::new(cart, opts);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0; // write something more funny here!
        }

        ctx.step();

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
