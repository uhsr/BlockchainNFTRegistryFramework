// src/main.rs
/*
 * Main executable for BlockchainNFTRegistryFramework
 */

use clap::Parser;
use blockchainnftregistryframework::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainNFTRegistryFramework - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
