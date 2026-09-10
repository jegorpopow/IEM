use std::{fs, io::stdout};

use anyhow::Context;

use ast::convert;
use interpreter::interpret;
use lexer::Lexer;
use parser::parse_program;

use crate::cli::Cli;

mod cli;

// TODO: clap
fn main() -> anyhow::Result<()> {
    let Cli { path } = Cli::parse();

    let source = fs::read_to_string(&path).context("IO error: cannot read from input file")?;

    let tokens: Vec<_> = Lexer::from(source.as_str()).collect();
    let program = parse_program(tokens).context("Parsing error")?;
    let program = convert(&program).context("Typecheck error")?;
    interpret(stdout().lock(), &program).context("Interpreter error")?;
    Ok(())
}
