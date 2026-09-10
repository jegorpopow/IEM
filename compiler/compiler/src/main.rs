use std::{fs, fs::File, io::Write};

use anyhow::Context;
use ast::convert;
use bytecode::serialize;
use codegen::compile;
use lexer::Lexer;
use parser::parse_program;

use crate::cli::Cli;

mod cli;

fn main() -> anyhow::Result<()> {
    let Cli { src, out } = Cli::parse();

    let source: String =
        fs::read_to_string(&src).context("IO error: cannot read from input file")?;

    let tokens: Vec<_> = Lexer::from(source.as_str()).collect();
    let program = parse_program(tokens).context("Parsing error")?;
    let program = convert(&program).context("Typecheck error")?;
    let program = compile(&program);
    File::create(out)
        .and_then(|mut out| serialize(&program, &mut |bytes| out.write_all(bytes)))
        .context("IO error: cannot write output file")?;
    Ok(())
}
