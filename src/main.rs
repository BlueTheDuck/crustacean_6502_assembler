#[macro_use]
extern crate nom;

use assembler::assemble;
use error::Error;
use parser::LineType;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use structopt::StructOpt;

// #region Arguments handling
#[derive(StructOpt)]
struct Args {
    #[structopt(short = "i", long, parse(from_os_str))]
    input: PathBuf,
    #[structopt(short = "o", long, parse(from_os_str))]
    output: Option<PathBuf>,
}
// #endregion

mod addressing_modes;
mod assembler;
mod error;
mod opcodes;
mod parser;

fn main() -> Result<(), error::Error> {
    let args: Args = Args::from_args();
    let input_buf = BufReader::new(
        std::fs::OpenOptions::new()
            .read(true)
            .open(&args.input)
            .unwrap_or_else(|e| panic!("Could not open input file. Error: {:?}", e)),
    );
    let mut output_buf = BufWriter::new(
        std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(match args.output {
                Some(v) => v,
                None => {
                    let mut out = args.input.clone();
                    out.set_extension("hex");
                    out
                }
            })
            .unwrap_or_else(|e| panic!("Could not open output file. Error: {:?}", e)),
    );

    let code: Vec<_> = input_buf
        .lines()
        .map(|line: Result<String, std::io::Error>| {
            let line: String = line?
                .trim()
                .split(';')
                .next()
                .unwrap_or_default()
                .to_owned();
            Ok(line)
        })
        .filter(|line: &Result<_, _>| match line {
            Ok(line) => !line.is_empty(),
            Err(_) => true,
        })
        .map(|line: Result<String, Error>| {
            line.and_then(|line| {
                parser::parse_line(line.as_bytes())
                    .map(|(_, line)| line)
                    .map_err(Into::into)
            })
        })
        .collect::<Result<_, Error>>()?;
    let code = assemble(code)?;
    output_buf.write_all(&code)?;

    Ok(())
}
