#[macro_use]
extern crate nom;

use assembler::assemble;
use error::Error;
use parser::LineType;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Format {
    Nes2,
    Hex,
}
impl<S: std::fmt::Display> std::convert::From<S> for Format {
    fn from(v: S) -> Self {
        match &*format!("{}", v) {
            "Nes2" => Self::Nes2,
            "Hex" => Self::Hex,
            _ => panic!("Unkown format {}", v),
        }
    }
}
impl From<Format> for String {
    fn from(v: Format) -> String {
        String::from(match v {
            Format::Hex => "hex",
            Format::Nes2 => "nes",
        })
    }
}
// #region Arguments handling
#[derive(StructOpt)]
struct Args {
    #[structopt(short = "i", long, parse(from_os_str))]
    input: PathBuf,
    #[structopt(short = "o", long, parse(from_os_str))]
    output: Option<PathBuf>,
    #[structopt(parse(from_str), long, default_value = "Nes2")]
    format: Format,
}
// #endregion

mod addressing_modes;
mod assembler;
mod error;
mod nes2;
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
                    out.set_extension::<String>(args.format.into());
                    out
                }
            })
            .unwrap_or_else(|e| panic!("Could not open output file. Error: {:?}", e)),
    );
    println!("Output format: {:?}", args.format);

    let code: Vec<_> = input_buf
        .lines()
        .map(|line: Result<String, std::io::Error>| {
            let line: String = line?
                .split(';')
                .next()
                .unwrap_or_default()
                .trim()
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

    let metadata = {
        let mut search_path = PathBuf::from(args.input);
        search_path.pop();
        assembler::Metadata { search_path }
    };

    let code = assemble(code, &metadata)?;
    match args.format {
        Format::Hex => output_buf.write_all(&code)?,
        Format::Nes2 => {
            let cart: Vec<u8> = nes2::Cartridge::new(&code, vec![0xC000, 0xC000, 0x0000]).into();
            output_buf.write_all(&cart)?;
        }
    }

    Ok(())
}
