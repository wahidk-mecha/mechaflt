#[derive(Debug)]
pub struct Script {
    pub commands: Vec<String>,
}

impl Script {
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

    pub fn default() -> Self {
        let default_script = r#" SDP: boot -f _flash.bin -scanlimited 0x800000
            SDPV: delay 1000
            SDPV: write -f _flash.bin -skipspl -scanterm -scanlimited 0x800000
            SDPV: jump -scanlimited 0x800000
            FB: ucmd setenv fastboot_dev mmc
            FB: ucmd setenv mmcdev ${emmc_dev}
            FB: ucmd mmc dev ${emmc_dev}
            FB: flash -scanterm -scanlimited 0x800000 bootloader _flash.bin
            FB: ucmd if env exists emmc_ack; then ; else setenv emmc_ack 0; fi;
            FB: ucmd mmc partconf ${emmc_dev} ${emmc_ack} 1 0
            FB: ucmd setenv fastboot_buffer ${loadaddr}
            FB: download -f Image 
            FB: ucmd setenv fastboot_buffer ${fdt_addr}
            FB: download -f imx8mm-mecha-comet-m-gen1.dtb
            FB: ucmd setenv fdtfile imx8mm-mecha-comet-m-gen1.dtb
            FB: ucmd printenv fdtfile 
            FB: ucmd setenv fastboot_buffer ${initrd_addr}
            FB: download -f mecha-image-mfgtool-initramfs-mecha-comet-m-gen1-20240702102938.rootfs.cpio.gz.u-boot
            FB: ucmd setenv mfgtool_args ${mfgtool_args} mfg_mmcdev=${emmc_dev}
            FB: ucmd run mfgtool_args
            FB: ucmd setenv bootargs root=/dev/ram0 rdinit=/linuxrc rw
            FB: ucmd printenv fdtfile
            FB: acmd booti ${loadaddr} ${initrd_addr} ${fdt_addr}
            FBK: ucmd sleep 3
            FBK: ucmd cmdline=`cat /proc/cmdline`;cmdline=${cmdline#*mfg_mmcdev=};cmds=($cmdline);echo ${cmds[0]}>/tmp/mmcdev
            FBK: ucmd sleep 1
            FBK: ucmd mmc=`cat /tmp/mmcdev`; while [ ! -e /dev/mmcblk2 ]; do sleep 1; echo "wait for /dev/mmcblk2 appear"; done;
            FBK: ucmd sleep 1
            FBK: ucmd parted /dev/mmcblk2 --script mklabel msdos
            FBK: ucmd lsblk
            FBK: ucmd parted /dev/mmcblk2 -- mkpart primary FAT16 10MB 195MB
            FBK: ucmd mkfs.vfat /dev/mmcblk2p1
            FBK: ucmd parted /dev/mmcblk2 -- mkpart primary ext4 205MB 10000MB
            FBK: ucmd mkfs.ext4 /dev/mmcblk2p2
            FBK: ucmd parted /dev/mmcblk2 -- mkpart primary ext4 10005MB 29000MB
            FBK: ucmd mkfs.ext4 /dev/mmcblk2p3
            FBK: ucmd mmc=`cat /tmp/mmcdev`; mkdir -p /mnt
            FBK: ucmd mmc=`cat /tmp/mmcdev`; mount -t ext4 /dev/mmcblk2p2 /mnt
            FBK: ucmd mmc=`cat /tmp/mmcdev`; mkdir -p /mnt/boot
            FBK: ucmd mmc=`cat /tmp/mmcdev`; mount -t vfat /dev/mmcblk2p1 /mnt/boot
            FBK: ucmd mmc=`cat /tmp/mmcdev`; mkdir -p /mnt/home
            FBK: ucmd mmc=`cat /tmp/mmcdev`; mount -t ext4 /dev/mmcblk2p3 /mnt/home
            FBK: acmd EXTRACT_UNSAFE_SYMLINKS=1 tar -z -x -C /mnt
            FBK: ucp _image t:- /mnt
            FBK: ucmd sleep 45
            FBK: ucmd sync
            FBK: DONE"#;

        let commands = default_script
            .lines()
            .map(|s| s.trim().to_string())
            .collect();
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
            let cmd_status = rust_uuu::run_command(command);
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
