#![warn(rust_2018_idioms)]

use clap::Parser as CliParser;
use std::{fs, path::PathBuf};

#[derive(CliParser)]
struct Cli {
    /// Path to the program file.
    program_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let code = fs::read_to_string(args.program_path)?;
    let tokens = lexer::lex(&code)?;

    dbg!(tokens);

    Ok(())
}
