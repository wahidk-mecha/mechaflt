use clap::{Parser, Subcommand};
use rust_uuu;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List connected USB devices
    Devices,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Devices => {
            rust_uuu::print_lsusb();
        }
    }
}
