use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use structopt::StructOpt;

mod addressing_modes;
mod opcodes;
mod parser;

// #region Arguments handling
#[derive(StructOpt)]
struct Args {
    #[structopt(short = "i", long, parse(from_os_str))]
    input: PathBuf,
    #[structopt(short = "o", long, parse(from_os_str))]
    output: Option<PathBuf>,
}
// #endregion

fn main() {
    let args = Args::from_args();
    let input_buf = BufReader::new(
        std::fs::OpenOptions::new()
            .read(true)
            .open(&args.input)
            .expect(&format!("Could not open file {:?}", &args.input)),
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
            .expect(&format!("Could not open file {:?}", &args.input)),
    );
    let mut addr: usize = 0;
    let mut tokens: Vec<parser::Token> = vec![];
    let mut labels: HashMap<String, usize> = HashMap::new();
    for line in input_buf.lines().map(|v| v.unwrap()) {
        tokens.push(parser::Token::new(line, addr));
        let last: &parser::Token = tokens.last().unwrap();
        addr += last.get_size();
        match &last.line_data {
            parser::LineData::Label(name) => {
                labels.insert(name.to_string(), addr);
            }
            _ => (),
        };
    }
    for token in tokens {
        let token: parser::Token = token;
        match token.line_data {
            parser::LineData::Code(code) => {
                let hex_opcode = opcodes::get_code(&code.name, code.addr_mode)
                    .expect(&format!("Unavailable addr mode for {:#?}", code));
                let mut hex = match code.arg {
                    Some(v) => {
                        addressing_modes::AddressingMode::assemble(&v, code.addr_mode, &labels)
                    }
                    None => vec![],
                };
                hex.push(hex_opcode as u8);
                hex.reverse();
                println!("{:02X?}", hex);
                output_buf.write(&hex);
            },
            parser::LineData::Macro(r#macro) => {
                let r#macro: parser::Macro = r#macro;
                output_buf.write(&r#macro.bytes);
            },
            _ => (),
        }
    }
    // println!("{}",input_file.read);
}
