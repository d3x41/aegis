mod functions;
mod internal;
//use crate::internal::secure;
use crate::functions::*;
use shared::args::{BootloaderSubcommand, Command, Cli, UsersSubcommand};
use shared::clap::Parser;
use shared::exec::check_if_root;
use shared::human_panic;
use shared::logging;
use shared::partition;

fn main() -> Result<(), i32> {
    check_if_root();
    human_panic::setup_panic!();
    let cli = Cli::parse();
    println!("verbose: {}", cli.verbose);
    let log_file_path = "/tmp/aegis";
    logging::init(cli.verbose, log_file_path);
    match cli.command {
        Command::Partition(args) => {
            let mut partitions = args.partitions;
            partition::partition(
                args.device,
                args.mode,
                args.encrypt_check,
                args.efi,
                args.swap,
                args.swap_size,
                &mut partitions,
            );
        }
        Command::InstallPackages(args) => {
            let package_set: Vec<&str> = Vec::new();
            base::install_packages(args.kernel, package_set);
        }
        Command::GenFstab => {
            base::genfstab();
        }
        //Command::SetupSnapper => base::setup_snapper(),
        Command::Bootloader { subcommand } => match subcommand {
            BootloaderSubcommand::GrubEfi { efidir } => {
                base::configure_bootloader_efi(efidir,false);
            }
            BootloaderSubcommand::GrubLegacy { device } => {
                base::configure_bootloader_legacy(device,false);
            }
        }
        Command::Locale(args) => {
            locale::set_locale(args.locales.join(" ")); // locale.gen file comes grom glibc package that is in base group package
            locale::set_keyboard(&args.virtkeyboard, &args.x11keyboard).unwrap_or_else(|e| {
                eprintln!("Error setting keyboard: {}", e);
            });
            locale::set_timezone(&args.timezone);
        }
        Command::Networking(args) => {
            if args.ipv6 {
                network::create_hosts();
                network::enable_ipv6()
            } else {
                network::create_hosts();
            }
            network::set_hostname(&args.hostname);
        }
        Command::Zram => {
            base::configure_zram();
        }
        /*Command::Hardened => {
            secure::secure_password_config();
            secure::secure_ssh_config();
        }*/
        Command::Users { subcommand } => match subcommand {
            UsersSubcommand::NewUser(args) => {
                users::new_user(
                    &args.username,
                    args.hasroot,
                    &args.password,
                    false,
                    &args.shell,
                );
            }
            UsersSubcommand::RootPass { password } => {
                users::root_pass(&password);
            }
        },
        Command::Flatpak => {
            base::configure_flatpak();
        }
        Command::Config { config } => {
            let exit_code = internal::config::read_config(config);
            if exit_code != 0 {
                return Err(exit_code);
            }
        }
        Command::Desktops { desktop } => {
            desktops::install_desktop_setup(desktop);
        }
        Command::Themes { design } => {
            themes::install_theme_setup(design);
        }
        Command::DisplayManagers { displaymanager } => {
            displaymanagers::install_dm_setup(displaymanager);
        }
        Command::Shells { shell } => {
            shells::install_shell_setup(shell);
        }
        Command::Browsers { browser } => {
            browsers::install_browser_setup(browser);
        }
        Command::Terminals { terminal } => {
            terminals::install_terminal_setup(terminal);
        }
        Command::EnableServices => {
            base::enable_system_services();
        }
        _ => todo!() //Do nothing for all those Command:: specified in shared/args.rs but not specifically implemented in athena-nix (because useless)
    }
    Ok(())
}
