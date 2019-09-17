use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;
use std::collections::HashMap;

mod addressing_modes;
mod parser;

// #region Arguments handling
#[derive(StructOpt)]
struct Args {
    #[structopt(short = "i", long, parse(from_os_str))]
    input: PathBuf,
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
    let mut addr: usize = 0;
    let mut tokens: Vec<parser::Token> = vec![];
    let mut labels: HashMap<String,usize> = HashMap::new();
    for line in input_buf.lines().map(|v| v.unwrap()) {
        tokens.push(parser::Token::new(line));
        let last: &parser::Token = tokens.last().unwrap();
        addr += last.get_size();
        match &last.line_data {
            parser::LineData::Label(name) => {labels.insert(name.to_string(), addr);},
            _ => (),
        };
    }
    println!("{:#?}", tokens);
    println!("{:04X}",addr);
    // println!("{}",input_file.read);
}
