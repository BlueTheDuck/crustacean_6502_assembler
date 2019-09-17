use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

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
    let mut tokens: Vec<parser::Token> = vec![];
    for line in input_buf.lines().map(|v| v.unwrap()) {
        tokens.push(parser::Token::new(line));
    }
    println!("{:#?}", tokens);
    // println!("{}",input_file.read);
}
