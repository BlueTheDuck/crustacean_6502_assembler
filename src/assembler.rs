use crate::{addressing_modes, opcodes, parser, Context};
use parser::Opcode;

struct Assembler {
    labels: HashMap<&str,usize>//Name: addr,
    undef_labels: HashMap<&str,Vec<usize>>,
    addr: usize,
}

pub fn assemble()