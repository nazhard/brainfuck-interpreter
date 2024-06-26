use std::fs;
use std::io::{self, Write};

use anyhow::Result;
use clap::Parser;

use interpreter::interpret;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    source_path: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let source = fs::read_to_string(args.source_path)?;
    let stdin = io::stdin();

    let result = interpret(&source, Box::new(stdin))?;
    io::stdout().write_all(result.as_bytes())?;

    Ok(())
}
