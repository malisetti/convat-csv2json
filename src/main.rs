use std::io::{self, Read};
use std::process::ExitCode;

use anyhow::{Context, Result};
use clap::Parser;
use convat_csv2json::convert;

#[derive(Parser, Debug)]
#[command(
    name = "convat-csv2json",
    version,
    about = "Convert CSV (with header row) to JSON-lines on stdout"
)]
struct Cli {}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e:#}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let _cli = Cli::parse();
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .context("failed to read stdin")?;
    print!("{}", convert(&input)?);
    Ok(())
}
