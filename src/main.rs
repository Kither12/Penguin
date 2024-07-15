use anyhow::{Ok, Result};
use chess_lang::run_code;
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// program read from script file
    #[arg(short, long, value_name = "FILE")]
    file: String,
}

fn main() {
    let args = Args::parse();
    let code = fs::read_to_string(args.file).expect("Failed to read file");
    run_code(&code).unwrap();
}
