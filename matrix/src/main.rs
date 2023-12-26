#![warn(rust_2018_idioms)]

use clap::Parser as CliParser;
use miette::{Diagnostic, IntoDiagnostic, NamedSource, Report, SourceCode};
use std::{fs, path::PathBuf};

#[derive(CliParser)]
struct Cli {
    /// Path to the program file.
    program_path: PathBuf,
}

fn map_err_to_report<T, E: Diagnostic + Send + Sync + 'static>(
    r: Result<T, E>,
    (source_name, source_code): (impl AsRef<str>, impl SourceCode + 'static),
) -> miette::Result<T> {
    r.map_err(|diagnostics| {
        Report::from(diagnostics).with_source_code(NamedSource::new(source_name, source_code))
    })
}

fn main() -> miette::Result<()> {
    let args = Cli::parse();

    let code = fs::read_to_string(&args.program_path).into_diagnostic()?;
    let source_name = args.program_path.display().to_string();

    let tokens = map_err_to_report(lexer::lex(&code), (&source_name, code.clone()))?;
    dbg!(&tokens);
    let ast = map_err_to_report(parser::parse(tokens), (&source_name, code))?;
    dbg!(ast);

    Ok(())
}
