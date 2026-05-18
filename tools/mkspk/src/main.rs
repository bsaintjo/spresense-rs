use std::{fs, process};

use clap::Parser;
use mkspk::pack_spk;

/// Pack an ARM ELF binary into a Spresense SPK package.
///
/// Drop-in replacement for the C mkspk tool in the Spresense SDK.
/// Invocation: mkspk -c <number> <filename> <save name> <output file>
#[derive(Parser)]
#[command(
    name = "mkspk",
    about = "Pack an ARM ELF into a Spresense SPK",
    override_usage = "mkspk [-c <number>] <filename> <save name> [<output file>]"
)]
struct Cli {
    /// Core selector (required)
    #[arg(short = 'c', value_name = "number")]
    core: Option<String>,

    /// Input ELF file
    #[arg(value_name = "filename")]
    elffile: Option<String>,

    /// Install name stored in the package header (max 63 bytes)
    #[arg(value_name = "save name")]
    savename: Option<String>,

    /// Output SPK file
    #[arg(value_name = "output file")]
    outputfile: Option<String>,
}

fn usage() -> ! {
    eprintln!("mkspk [-c <number>] <filename> <save name> [<output file>]");
    process::exit(1);
}

fn main() {
    let cli = Cli::parse();

    // Mirror C's strict validation — exact same error messages
    let core_str = match &cli.core {
        Some(s) => s.clone(),
        None => {
            eprintln!("Core number is not set. Please use -c option.");
            process::exit(1);
        }
    };

    // Match C's strtol(optarg, &endp, 0): decimal or 0x-prefixed hex
    let parsed = if let Some(hex) = core_str.strip_prefix("0x").or_else(|| core_str.strip_prefix("0X")) {
        i64::from_str_radix(hex, 16).ok()
    } else {
        core_str.parse::<i64>().ok()
    };
    let core: u8 = match parsed {
        Some(n) if (0..=255).contains(&n) => n as u8,
        _ => {
            eprintln!("Invalid core number \"{core_str}\"");
            usage();
        }
    };

    let elffile = match &cli.elffile {
        Some(s) => s.clone(),
        None => usage(),
    };

    let savename = match &cli.savename {
        Some(s) => s.clone(),
        None => usage(),
    };

    let outputfile = match &cli.outputfile {
        Some(s) => s.clone(),
        None => usage(),
    };

    if savename.len() > 63 {
        eprintln!("savename too long.");
        process::exit(1);
    }

    let elf_bytes = match fs::read(&elffile) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Loading ELF {elffile} failure.\n{e}");
            process::exit(1);
        }
    };

    let spk = match pack_spk(&elf_bytes, &savename, core) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Loading ELF {elffile} failure.\n{e}");
            process::exit(1);
        }
    };

    if let Err(e) = fs::write(&outputfile, &spk) {
        eprintln!("Output file open error.\n{e}");
        process::exit(1);
    }

    println!("File {outputfile} is successfully created.");
}
