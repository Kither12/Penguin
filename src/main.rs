use anyhow::{Context, Ok, Result};
use clap::Parser;
use penguin::run_code;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// program read from script file
    #[arg(short, long, value_name = "FILE")]
    file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let code = fs::read_to_string(args.file).context("Failed to read file")?;
    run_code(&code)?;
    Ok(())
}
