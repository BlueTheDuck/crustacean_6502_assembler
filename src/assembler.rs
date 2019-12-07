use crate::addressing_modes;
use crate::parser::{LineType, Value};
use crate::{opcodes, opcodes::get_code};
use std::collections::HashMap;

fn big_to_little_endiann(value: &u16) -> (u8, u8) {
    ((value & 0xFF) as u8, ((value & 0xFF00) >> 8) as u8)
}

pub fn assemble(parsed_code: Vec<LineType>) -> Result<[u8; 0x10000], ()> {
    let mut cart: [u8; 0x10000] = [0x00; 0x10000];
    let mut labels: HashMap<String, usize> = HashMap::default();
    let mut undef_labels: HashMap<String, Vec<usize>> = HashMap::default();
    let mut last_addr: usize = 0x8000;
    for line in parsed_code {
        let line: LineType = line;
        match line {
            LineType::LabelDef(name) => {
                labels.insert(name.clone(), last_addr);
            }
            LineType::Opcode(opcode) => {
                let code = get_code(&opcode.name, &opcode.arg.0).ok_or(())?;
                let size = addressing_modes::get_size(&opcode.arg.0);
                cart[last_addr] = code;
                match &opcode.arg.1 {
                    Value::Long(long) => {
                        let long = big_to_little_endiann(long);
                        cart[last_addr + 1] = long.0;
                        cart[last_addr + 2] = long.1;
                    }
                    Value::Short(short) => cart[last_addr + 1] = *short,
                    Value::Label(_) => {}
                    Value::None => {}
                }
                last_addr += size;
                println!("Assembling {:?} as {:#04X}", opcode, code);
            }
        };
    }
    Ok(cart)
}

mod tests {
    use super::assemble;
    use crate::parser::{parse_line, LineType};
    #[test]
    fn test_assemble() {
        let test_code: &str = include_str!("../assembly/basic_opcodes.asm");
        let test_code: Vec<LineType> = test_code
            .lines()
            .map(|l: &str| parse_line(l.as_bytes()).unwrap().1)
            .collect();
        let code: [u8; 0x10000] = assemble(test_code).expect("This shouldn't have errored");
        super::dump(&code, Some(0x80), Some(0x80));
        assert_eq!(code[0x8000..0x8005], [0xA9, 0xFF, 0x85, 0xFF, 0x18]);
    }
}

pub fn dump(code: &[u8], page_start: Option<usize>, page_end: Option<usize>) {
    for (page_addr, page) in code.chunks(256).enumerate() {
        if page_start.is_some() && page_addr < page_start.unwrap() {
            continue;
        }
        if page_end.is_some() && page_addr > page_end.unwrap() {
            break;
        }
        let page: &[u8] = page;
        for (line_addr, line) in page.chunks(16).enumerate() {
            let line: &[u8] = line;
            println!(
                "{:#04X?}: {:?}",
                page_addr * 256 + line_addr * 16,
                line.iter()
                    .map(|v| format!("{:02X}", v))
                    .collect::<Vec<String>>()
                    .join(",")
            );
        }
    }
}
