use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Args {
    /// Relative path to the Gameboy Rom
    #[arg(short, long)]
    pub path: String,

    /// Show register values
    #[arg(short, long, required = false, default_value_t = false)]
    pub debug: bool,

    /// Show output in the serial register. Useful for blarrgs's test rom
    #[arg(short, long, required = false, default_value_t = false)]
    pub serial: bool,
}
