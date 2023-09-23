use crate::internal::exec::*;
use crate::internal::files::append_file;
use crate::internal::*;
use log::warn;
use std::path::PathBuf;

pub fn install_base_packages(kernel: String) {
    std::fs::create_dir_all("/mnt/etc").unwrap();
    let kernel_to_install = if kernel.is_empty() {
        "linux-lts"
    } else {
        match kernel.as_str() {
            "linux" => "linux",
            "linux-lts" => "linux-lts",
            "linux-zen" => "linux-zen",
            "linux-hardened" => "linux-hardened",
            "linux-rt" => "linux-rt",
            "linux-rt-lts" => "linux-rt-lts",
            "linux-lqx" => "linux-lqx",
            "linux-xanmod" => "linux-xanmod",
            _ => {
                warn!("Unknown kernel: {}, using default instead", kernel);
                "linux-lts"
            }
        }
    };
    install::install(vec![
        // Base Arch
        "base",
        kernel_to_install,
        format!("{kernel_to_install}-headers").as_str(),
        "linux-firmware",
        "systemd-sysvcompat",
        "networkmanager",
        "man-db",
        "man-pages",
        "texinfo",
        "nano",
        "sudo",
        "curl",
        "archlinux-keyring",
        // Extra Base Arch
        "accountsservice",
        "alsa-utils",
        "arch-install-scripts",
        "broadcom-wl-dkms",
        "dhcpcd",
        "dialog",
        "dosfstools",
        "edk2-shell",
        "inetutils",
        "irqbalance",
        "lvm2",
        "memtest86+",
        "mesa",
        "mesa-utils",
        "mkinitcpio-nfs-utils",
        "mkinitcpio-openswap",
        "most",
        "mtools",
        "nbd",
        "net-tools",
        "netctl",
        "nfs-utils",
        "nohang",
        "nss-mdns",
        "ntfsprogs",
        "ntp",
        "pavucontrol",
        "profile-sync-daemon",
        "pv",
        "rsync",
        "rtl8821cu-morrownr-dkms-git",
        "sof-firmware",
        "squashfs-tools",
        "syslinux",
        "timelineproject-hg",
        "usbutils",
        "wireless_tools",
        "wpa_supplicant",
        "xfsprogs",
        // Fonts
        "noto-fonts",
        "noto-fonts-emoji",
        "noto-fonts-cjk",
        // Common packages for all desktops
        "pipewire",
        "pipewire-pulse",
        "pipewire-alsa",
        "pipewire-jack",
        "wireplumber",
        "ntfs-3g",
        "vi",
        "eza",
        "pocl", // Hashcat dependency
        "ananicy",
        "armcord-git",
        "asciinema",
        "bashtop",
        "bat",
        "bc",
        "bless",
        "chatgpt-desktop-bin",
        "cmatrix",
        "cowsay",
        "cron",
        "cyberchef-electron",
        "downgrade",
        "edex-ui-bin",
        "eog",
        "espeakup",
        "figlet",
        "figlet-fonts",
        "file-roller",
        "fortune-mod",
        "git",
        "gparted",
        "grub-customizer",
        "gtk-engine-murrine",
        "gvfs-gphoto2",
        "gvfs-mtp",
        "hexedit",
        //"hw-probe, //HW probing
        "imagemagick",
        "jdk-openjdk",
        "jq",
        "lib32-glibc",
        "lolcat",
        "lsd",
        "mtpfs",
        "nano-syntax-highlighting",
        "nautilus",
        "ncdu",
        "networkmanager-openvpn",
        "nyancat",
        "octopi",
        "onionshare",
        "openbsd-netcat",
        "openvpn",
        "orca",
        "p7zip",
        "paru",
        "pfetch",
        "polkit",
        "python-pywhat",
        "reflector",
        "sl",
        //"smartmontools", //hw-probe deps
        "superbfetch-git",
        "textart",
        "tidy",
        "tk",
        "toilet-fonts",
        "tor-browser",
        "tree",
        "ufw",
        "unzip",
        "vnstat",
        "wget",
        "which",
        "xclip",
        "xcp",
        "xmlstarlet",
        "zoxide",
        // Repositories
        "athena-keyring",
        "athena-mirrorlist",
        "blackarch-keyring",
        "blackarch-mirrorlist",
        "chaotic-keyring",
        "chaotic-mirrorlist",
        // Athena
        "athena-cyber-hub",
        "athena-neofetch-config",
        "athena-nvchad",
        "athena-powershell-config",
        "athena-system-config",
        "athena-theme-tweak",
        "athena-tmux-config",
        "athena-vim-config",
        "athena-vscodium-themes",
        "athena-welcome",
        "htb-toolkit",
        "nist-feed",
    ]);
    files::copy_file("/etc/pacman.conf", "/mnt/etc/pacman.conf");

    exec_eval(
        exec_chroot(
            "systemctl",
            vec![String::from("enable"), String::from("bluetooth")],
        ),
        "Enable bluetooth",
    );

    /*exec_eval(
        exec_chroot(
            "systemctl",
            vec![String::from("enable"), String::from("cups")],
        ),
        "Enable CUPS",
    );*/
}

pub fn genfstab() {
    exec_eval(
        exec(
            "bash",
            vec![
                String::from("-c"),
                String::from("genfstab -U /mnt >> /mnt/etc/fstab"),
            ],
        ),
        "Generate fstab",
    );
}

pub fn install_bootloader_efi(efidir: PathBuf) {
    install::install(vec![
        "grub",
        "efibootmgr",
        "os-prober",
    ]);
    let efidir = std::path::Path::new("/mnt").join(efidir);
    let efi_str = efidir.to_str().unwrap();
    if !std::path::Path::new(&format!("/mnt{efi_str}")).exists() {
        crash(format!("The efidir {efidir:?} doesn't exist"), 1);
    }
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![
                String::from("--target=x86_64-efi"),
                format!("--efi-directory={}", efi_str),
                String::from("--bootloader-id=tofill"),
                String::from("--removable"),
            ],
        ),
        "install grub as efi with --removable",
    );
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![
                String::from("--target=x86_64-efi"),
                format!("--efi-directory={}", efi_str),
                String::from("--bootloader-id=tofill"),
            ],
        ),
        "install grub as efi without --removable",
    );
    files_eval(
        append_file(
            "/mnt/etc/default/grub",
            "GRUB_THEME=\"/boot/grub/themes/athena/theme.txt\"",
        ),
        "enable athena grub theme",
    );
    exec_eval(
        exec_chroot(
            "grub-mkconfig",
            vec![String::from("-o"), String::from("/boot/grub/grub.cfg")],
        ),
        "create grub.cfg",
    );
}

pub fn install_bootloader_legacy(device: PathBuf) {
    install::install(vec![
        "grub",
        "athena-grub-theme",
        "os-prober",
    ]);
    if !device.exists() {
        crash(format!("The device {device:?} does not exist"), 1);
    }
    let device = device.to_string_lossy().to_string();
    exec_eval(
        exec_chroot(
            "grub-install",
            vec![String::from("--target=i386-pc"), device],
        ),
        "install grub as legacy",
    );
    files_eval(
        append_file(
            "/mnt/etc/default/grub",
            "GRUB_THEME=\"/boot/grub/themes/athena/theme.txt\"",
        ),
        "enable athena grub theme",
    );
    exec_eval(
        exec_chroot(
            "grub-mkconfig",
            vec![String::from("-o"), String::from("/boot/grub/grub.cfg")],
        ),
        "create grub.cfg",
    );
}

pub fn setup_timeshift() {
    install(vec!["timeshift", "timeshift-autosnap", "grub-btrfs"]);
    exec_eval(
        exec_chroot("timeshift", vec![String::from("--btrfs")]),
        "setup timeshift",
    )
}

pub fn setup_snapper() {
    install(vec!["snap-pac", "snap-pac-grub", "snapper-support"]);
}

pub fn install_homemgr() {
    install(vec!["nix"]);
}

pub fn install_flatpak() {
    install(vec!["flatpak"]);
    exec_eval(
        exec_chroot(
            "flatpak",
            vec![
                String::from("remote-add"),
                String::from("--if-not-exists"),
                String::from("flathub"),
                String::from("https://flathub.org/repo/flathub.flatpakrepo"),
            ],
        ),
        "add flathub remote",
    )
}

pub fn install_cuda() {
    install(vec!["cuda"]);
}

pub fn install_spotify() {
    install(vec!["spotify"]);
}

pub fn install_cherrytree() {
    install(vec!["cherrytree"]);
}

pub fn install_flameshot() {
    install(vec!["flameshot"]);
}

pub fn install_busybox() {
    install(vec!["busybox"]);
}

pub fn install_toybox() {
    install(vec!["toybox"]);
}

pub fn install_zram() {
    install(vec!["zram-generator"]);
    files::create_file("/mnt/etc/systemd/zram-generator.conf");
    files_eval(
        files::append_file("/mnt/etc/systemd/zram-generator.conf", "[zram0]"),
        "Write zram-generator config",
    );
}
