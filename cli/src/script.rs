use include_dir::{Dir, include_dir};

static SCRIPTS_DIR: Dir = include_dir!("./cli/scripts/");

#[derive(Debug)]
pub struct Script {
    pub commands: Vec<String>,
}

impl Script {
    /// Create a new runnable script from a file
    pub fn new(file: &str) -> Self {
        let contents = std::fs::read_to_string(file).unwrap();
        let commands = contents
            .lines()
            .map(|s| {
                if s.starts_with('#') {
                    return String::new();
                }
                s.to_string()
            })
            .collect();
        Self { commands }
    }

    /// Default script (for iMX8MM)
    pub fn default() -> Self {
        let commands = SCRIPTS_DIR.get_file("mecha-comet-gen1-r5.auto").unwrap();
        let commands = commands.contents_utf8().unwrap();
        let commands = commands
            .lines()
            .map(|s| {
                if s.starts_with('#') {
                    return String::new();
                }
                s.to_string()
            })
            .collect();
        println!("{:?}", commands);
        Self { commands }
    }

    /// Use the image to flash the device
    pub fn with_image(self, image: &str) -> Self {
        let commands = self
            .commands
            .iter()
            .map(|s| {
                if s.contains("_image") {
                    s.replace("_image", image)
                } else {
                    s.to_string()
                }
            })
            .collect();
        Self { commands }
    }

    /// Use the bootloader to flash the device
    pub fn with_bootloader(self, bootloader: &str) -> Self {
        let commands = self
            .commands
            .iter()
            .map(|s| {
                if s.contains("_flash.bin") {
                    s.replace("_flash.bin", bootloader)
                } else {
                    s.to_string()
                }
            })
            .collect();
        Self { commands }
    }

    /// Run the script
    pub fn run(&self) -> Result<(), String> {
        for command in &self.commands {
            println!("> {}", command);
            let cmd_status = uuu_rs::run_command(command);
            match cmd_status {
                Ok(()) => {}
                Err(e) => {
                    return Err(e);
                }
            }
        }
        println!("Script execution completed.");
        Ok(())
    }
}
