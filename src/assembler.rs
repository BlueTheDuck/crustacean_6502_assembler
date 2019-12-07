use crate::addressing_modes;
use crate::error::Error;
use crate::parser::{LineType, Value};
use crate::{opcodes, opcodes::get_code};
use std::collections::HashMap;

/// 0: Low 1: High
fn big_to_little_endiann(value: u16) -> (u8, u8) {
    ((value & 0xFF) as u8, ((value & 0xFF00) >> 8) as u8)
}

pub fn assemble(parsed_code: Vec<LineType>) -> Result<[u8; 0x10000], Error> {
    // cart: holds the code
    // labels: label=address
    // labels_used_on: places where a label was used
    // addr: where to place the code
    let mut cart: [u8; 0x10000] = [0x00; 0x10000];
    let mut labels: HashMap<String, usize> = HashMap::default();
    let mut labels_used_on: HashMap<String, Vec<usize>> = HashMap::default();
    let mut addr: usize = 0x8000;

    for line in parsed_code {
        match line {
            LineType::LabelDef(name) => {
                labels.insert(name.clone(), addr);
            }
            LineType::Opcode(opcode) => {
                let code = get_code(opcode.name, &opcode.arg.0)?;
                let size = addressing_modes::get_size(&opcode.arg.0);
                cart[addr] = code;
                match &opcode.arg.1 {
                    Value::Long(long) => {
                        let long = big_to_little_endiann(*long);
                        cart[addr + 1] = long.0;
                        cart[addr + 2] = long.1;
                    }
                    Value::Short(short) => cart[addr + 1] = *short,
                    Value::Label(name) => {
                        if let Some(addresses) = labels_used_on.get_mut(name) {
                            addresses.push(addr);
                        } else {
                            labels_used_on.insert(name.clone(), [addr].to_vec());
                        }
                    }
                    Value::None => {}
                }
                addr += size;
                println!("Assembling {:?} as {:#04X}", opcode, code);
            }
        };
    }
    // Iterate thru all the defined labels
    // removing them from the list of used_labels
    // and placing their address in the whitespaces left
    // on the assembling stage
    for label in labels.keys() {
        // Skip if the label wasn't used
        let addresses_where_used = match labels_used_on.remove(label) {
            None => continue,
            Some(v) => v,
        };
        let addr_little = big_to_little_endiann(labels[label] as u16);
        for address in addresses_where_used {
            cart[address + 1] = addr_little.0;
            cart[address + 2] = addr_little.1;
        }
    }
    if !labels_used_on.is_empty() {
        return Err(Error::UndefLabel {
            labels: format!("{:?}", labels_used_on),
        });
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
    #[test]
    fn test_labels() {
        let test_code: &str = "\tLDA main";
        let test_code: Vec<LineType> = test_code
            .lines()
            .map(|l: &str| parse_line(l.as_bytes()).unwrap().1)
            .collect();
        assert!(assemble(test_code).is_err());
        let test_code: &str = "main:\n\tLDA main";
        let test_code: Vec<LineType> = test_code
            .lines()
            .map(|l: &str| parse_line(l.as_bytes()).unwrap().1)
            .collect();
        let code = assemble(test_code).unwrap();
        assert_eq!(code[0x8000..0x8003], [0xAD, 0x00, 0x80]);
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
