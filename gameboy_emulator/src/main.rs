use clap::Parser;
use eframe;
use gameboy_emulator_lib::{cartridge::Cartridge, rom::Rom, utils::Opts};
use tracing_subscriber;

mod args;

struct Emulator {
    label: String,
}

impl Default for Emulator {
    fn default() -> Self {
        Self {
            label: "gb emulator".to_string(),
        }
    }
}

impl Emulator {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for Emulator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { label } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
            ui.label(label.to_string());
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use args::Args;
    use gameboy_emulator_lib::emu::EmuContext;

    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(350.0, 380.0))
    };


    eframe::run_native(
        "Gameboy Emulator",
        native_options,
        Box::new(|cc| Box::new(Emulator::new(cc))),
    );

    let args = Args::parse();

    let rom = Rom::new(args.path.to_string());

    let cart = Cartridge::new(rom.data);

    let opts = Opts::new(args.debug, args.serial);

    let mut ctx = EmuContext::new(cart, opts);

    loop {
        ctx.step();
    }
}
