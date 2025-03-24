use clap::Parser;
use inquire::{
    Confirm, Text,
    ui::{RenderConfig, Styled},
};
use mechaflt::cli::{Cli, Commands};
use mechaflt::script::Script;
use rust_uuu;

fn main() {
    let render_config = get_render_config();
    inquire::set_global_render_config(render_config);
    let cli = Cli::parse();

    match &cli.command {
        Commands::Devices => {
            rust_uuu::print_devices();
        }

        Commands::Flash { image } => {
            // Checck if the file exists
            if !std::path::Path::new(image).exists() {
                println!("{} does not exist.", image);
                return;
            }

            // Setup prompts
            let ans = Confirm::new("Is the device in SERIAL mode?")
                .with_default(true)
                .with_help_message(
                    "Make sure to connect to the device's HOST USB port. Learn more [mecha.so/...]",
                )
                .prompt()
                .unwrap();
            if !ans {
                println!("Please connect the device in SERIAL mode and try again.");
                return;
            }

            println!("Searching for device...");
            rust_uuu::print_devices();
            let devices = rust_uuu::get_devices();
            if devices.is_empty() {
                return;
            }

            let ans =
                Confirm::new(format!("Do you want to flash {} to the device?", image).as_str())
                    .with_default(true)
                    .prompt()
                    .unwrap();
            if !ans {
                println!("Aborted.");
                return;
            }

            println!("Flashing image...");

            // Flash the image
            let script = Script::default()
                .with_image(image)
                .with_bootloader("flash.bin");
            let script_status = script.run();
            match script_status {
                Ok(()) => {
                    println!("Script executed successfully");
                }
                Err(e) => {
                    println!("Error: {}", e);
                    println!("Script execution aborted.");
                }
            }
        }

        Commands::Shell => {
            println!("Enter command on prompt, or type 'exit' to quit");
            loop {
                let cmd: String = Text::new("").prompt().unwrap();
                match cmd.as_str() {
                    "exit" | "quit" => break,
                    "" => {}
                    _ => {
                        let cmd_status = rust_uuu::run_command(&cmd);
                        match cmd_status {
                            Ok(()) => {
                                println!("Command executed successfully");
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                            }
                        }
                    }
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
