use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::PathBuf;
use structopt::StructOpt;

mod addressing_modes;
mod assembler;
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

// #region Program
struct Context {
    addr: usize,
    tokens: Vec<parser::Token>,
    labels: HashMap<String, usize>,
    hex: Vec<u8>
}
impl Context {
    fn new() -> Self {
        Context {
            addr: 0,
            tokens: vec![],
            labels: HashMap::new(),
            hex: vec![]Zzz
        }
    }
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
    let mut ctx = Context::new();
    for line in input_buf.lines().map(|v| v.unwrap()) {
        ctx.tokens.push(parser::Token::new(line, ctx.addr));
        let last: &parser::Token = ctx.tokens.last().unwrap();
        ctx.addr += last.get_size();
        match &last.line_data {
            parser::LineData::Label(name) => {
                ctx.labels.insert(name.to_string(), ctx.addr);
            }
            parser::LineData::Macro(r#macro) => {
                let r#macro: &parser::Macro = r#macro;
                println!("{:#?}", r#macro);
            }
            _ => (),
        };
    }
    assembler::assemble(&mut ctx);
    // println!("{}",input_file.read);
}
