use std::{path::PathBuf, process};

use clap::Parser;
use flash_writer::{flash, FlashOptions};

#[derive(Parser)]
#[command(
    name = "flash_writer",
    bin_name = "flash_writer",
    about = "Install Spresense SPK packages over UART",
    override_usage = "flash_writer [-s] -c <port> [-D] [-b <baud>] [-n] [--set-bootable] <spk> [<spk>...]"
)]
struct Cli {
    /// Use serial transport (accepted for compatibility; always active)
    #[arg(short = 's', action = clap::ArgAction::SetTrue)]
    serial: bool,

    /// Serial port device (e.g. /dev/ttyUSB0)
    #[arg(short = 'c', long = "serial-port", value_name = "PORT", required = true)]
    port: String,

    /// Skip the DTR-pulse / nash handshake (only for boards where DTR is not wired)
    #[arg(short = 'D', long = "no-dtr-reset", action = clap::ArgAction::SetTrue)]
    no_dtr_reset: bool,

    /// Deprecated: DTR reset is on by default. Accepted for backward compatibility.
    #[arg(short = 'd', long = "dtr-reset", action = clap::ArgAction::SetTrue, hide = true)]
    _dtr_reset_compat: bool,

    /// Switch to this baud rate for the XMODEM transfer phase
    #[arg(short = 'b', long = "xmodem-baudrate", value_name = "BAUD")]
    xmodem_baud: Option<u32>,

    /// Run `set bootable M0P` after install (most boards reject this; off by default)
    #[arg(long = "set-bootable", action = clap::ArgAction::SetTrue)]
    set_bootable: bool,

    /// Accepted for SDK compatibility; equivalent to omitting --set-bootable
    #[arg(short = 'n', long = "no-set-bootable", action = clap::ArgAction::SetTrue, hide = true)]
    _no_set_bootable_compat: bool,

    /// SPK file(s) to install
    #[arg(value_name = "spk", required = true)]
    packages: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    let pkg_refs: Vec<&std::path::Path> = cli.packages.iter().map(|p| p.as_path()).collect();

    let opts = FlashOptions {
        port: &cli.port,
        packages: &pkg_refs,
        dtr_reset: !cli.no_dtr_reset,
        xmodem_baud: cli.xmodem_baud,
        set_bootable: cli.set_bootable,
        reboot: true,
    };

    if let Err(e) = flash(&opts) {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
