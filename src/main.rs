use clap::{Parser, Subcommand};
use inquire::{
    Text,
    ui::{RenderConfig, Styled},
};
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
    /// Flash an image to a device
    Flash {
        /// Path to the image to flash
        image: String,
    },
    /// Interactive shell
    Shell,
}

fn main() {
    let render_config = get_render_config();
    inquire::set_global_render_config(render_config);
    let cli = Cli::parse();

    match &cli.command {
        Commands::Devices => {
            rust_uuu::print_lsusb();
        }

        Commands::Flash { image } => {
            // TODO: Check if device is connected and image exists
            // TODO: FLash the image
            //get_usb_device_list();
            todo!("Flashing is not yet implemented");
        }

        Commands::Shell => {
            println!("Enter command on prompt, or type 'exit' to quit");
            loop {
                let cmd: String = Text::new("").prompt().unwrap();
                match cmd.as_str() {
                    "exit" | "quit" => break,
                    "" => {}
                    _ => match rust_uuu::run_command(&cmd) {
                        Ok(()) => {}
                        Err(e) => {
                            eprintln!("Error: {}", e);
                        }
                    },
                }
            }
        }
    }
}

fn get_render_config() -> RenderConfig<'static> {
    let mut render_config = RenderConfig::default();
    render_config.prompt_prefix = Styled::new(">");
    render_config
}
