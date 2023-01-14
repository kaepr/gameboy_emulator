use clap::Parser;
use eframe;
use gameboy_emulator_lib::{cartridge::Cartridge, cpu::CPU, rom::Rom, utils::Opts};
use tracing_subscriber;

mod args;

#[derive(Default)]
struct MyEguiApp {}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use args::Args;

    tracing_subscriber::fmt::init();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Gameboy Emulator",
        native_options,
        Box::new(|cc| Box::new(MyEguiApp::new(cc))),
    );

    let args = Args::parse();

    let rom = Rom::new(args.path.to_string());

    let cart = Cartridge::new(rom.data);

    let opts = Opts::new(args.debug, args.serial);

    let mut cpu = CPU::new(cart, opts);

    loop {
        cpu.step();
    }
}
