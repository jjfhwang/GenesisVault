// src/main.rs
/*
 * Main executable for GenesisVault
 */

use clap::Parser;
use genesisvault::{Result, run};

#[derive(Parser)]
#[command(version, about = "GenesisVault - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
