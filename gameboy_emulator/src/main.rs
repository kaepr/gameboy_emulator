use clap::Parser;
use gameboy_emulator_lib::{
    cartridge::Cartridge,
    emu::EmuContext,
    io::ppu::registers::Color,
    rom::Rom,
    utils::{BitPosCheck, Opts},
};

mod args;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use args::Args;
    use gameboy_emulator_lib::{emu::EmuContext, utils::CYCLES_1_FRAME};
    use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

    const WIDTH: usize = 640;
    const HEIGHT: usize = 360;

    let mut main_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut debug_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let custom_window = WindowOptions {
        borderless: false,
        transparency: false,
        title: true,
        resize: false,
        scale: Scale::X2,
        scale_mode: ScaleMode::Stretch,
        topmost: false,
        none: false,
    };

    let mut window =
        Window::new("gbemu", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut debug_window =
        Window::new("debug gbemu", WIDTH, HEIGHT, custom_window).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let args = Args::parse();

    let rom = Rom::new(args.path.to_string());

    let cart = Cartridge::new(rom.data);

    let opts = Opts::new(args.debug, args.serial);

    let mut ctx = EmuContext::new(cart, opts);

    while window.is_open() && !window.is_key_down(Key::Escape) && debug_window.is_open() {
        let mut cycles_elapsed = 0;
        loop {
            cycles_elapsed += ctx.step();

            if cycles_elapsed > CYCLES_1_FRAME {
                break;
            }
        }

        for i in main_buffer.iter_mut() {
            *i = 123124;
        }

        update_debug_buffer(&mut debug_buffer, &mut ctx);

        window
            .update_with_buffer(&main_buffer, WIDTH, HEIGHT)
            .unwrap();

        debug_window
            .update_with_buffer(&debug_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn color_to_rgb(color: Color) -> u32 {
    match color {
        Color::C0 => from_u8_rgb(255, 255, 255),
        Color::C1 => from_u8_rgb(169, 169, 169),
        Color::C2 => from_u8_rgb(84, 84, 84),
        Color::C3 => from_u8_rgb(0, 0, 0),
    }
}

fn get_col(fst: bool, snd: bool) -> Color {
    match (fst, snd) {
        (true, true) => Color::C3,
        (true, false) => Color::C2,
        (false, true) => Color::C1,
        (false, false) => Color::C0,
    }
}

/// Draws tile with 1 pixel margin
fn draw_tile(buffer: &mut Vec<u32>, tile_data: Vec<u8>, pos: (usize, usize)) {
    let (x_pos, _) = pos;
    let mut row = pos.1;

    let mut cnt = 0;
    for bytes in tile_data.chunks(2) {
        let mut idx: i32 = 7;
        let mut cur_x: usize = x_pos.into();
        cnt += 1;

        while idx >= 0 {
            let color = get_col(
                bytes[0].is_bit_set(idx as usize),
                bytes[1].is_bit_set(idx as usize),
            );
            let pixel_color = color_to_rgb(color);

            // place pixel at correct position in linear array
            buffer[cur_x + 640 * row] = pixel_color;
            cur_x += 1;
            idx -= 1;
        }

        row += 1;
    }

    println!("draw tile runs for {}", cnt);
}

fn reset_buffer(buffer: &mut Vec<u32>) {
    for pixel in buffer {
        *pixel = from_u8_rgb(0, 0, 0);
    }
}

fn update_debug_buffer(buffer: &mut Vec<u32>, ctx: &mut EmuContext) {
    // Render VRAM Tile Data
    // Renders 16 tiles in each row
    let mut x_start = 0;
    let mut row_no = 0;
    let mut count = 0;

    reset_buffer(buffer);

    let vram_ref = &ctx.bus.borrow().ppu.vram;

    for tile_no in 0..384 {
        println!("tile no: {}", tile_no);
        let tile_data = vram_ref
            .iter()
            .skip(tile_no * 16)
            .take(16)
            .map(|x| *x)
            .collect::<Vec<_>>();

        draw_tile(buffer, tile_data, (x_start, row_no));

        x_start += 9;
        count += 1;

        if count == 16 {
            x_start = 0;
            count = 0;
            row_no += 9;
        }

        println!("row no: {}", row_no);
    }
}

fn update_main_buffer(buffer: &mut Vec<u32>, ctx: &mut EmuContext) {}
