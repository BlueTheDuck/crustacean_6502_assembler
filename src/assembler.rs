use crate::addressing_modes;
use crate::error::Error;
use crate::opcodes::get_code;
use crate::parser::{LineType, Value};
use std::collections::HashMap;

/// 0: Low 1: High
fn big_to_little_endiann(value: u16) -> (u8, u8) {
    ((value & 0xFF) as u8, ((value & 0xFF00) >> 8) as u8)
}

struct Code {
    cart: [u8; 0x10000],
    pointer: usize,
}
impl Code {
    fn new() -> Self {
        Self {
            cart: [0x00u8; 0x10000],
            pointer: 0,
        }
    }
    /// Place a u8 on self.pointer, then increment by 1
    fn push_byte(&mut self, byte: u8) {
        self.cart[self.pointer] = byte;
        self.pointer += 1;
    }
    /// Take a u16, convert it to little endiand then place it on self.pointer, finally increment by 2
    fn push_long(&mut self, long: u16) {
        let long = big_to_little_endiann(long);
        self.push_byte(long.0);
        self.push_byte(long.1);
    }
    /// Add <amount> to self.pointer
    fn skip(&mut self, amount: usize) {
        self.pointer += amount;
    }
}

pub fn assemble(parsed_code: Vec<LineType>) -> Result<[u8; 0x10000], Error> {
    // cart: holds the code
    // labels: label=address
    // labels_used_on: places where a label was used
    // addr: where to place the code
    let mut code = Code::new();
    let mut labels: HashMap<String, usize> = HashMap::default();
    let mut labels_used_on: HashMap<String, Vec<usize>> = HashMap::default();

    for line in parsed_code {
        match line {
            LineType::LabelDef(name) => {
                labels.insert(name.clone(), code.pointer);
            }
            LineType::Opcode(opcode) => {
                let opcode_number = get_code(opcode.name, opcode.arg.0)?;
                let size = addressing_modes::get_size(opcode.arg.0);
                code.push_byte(opcode_number);
                match &opcode.arg.1 {
                    Value::Long(long) => code.push_long(*long),
                    Value::Short(short) => code.push_byte(*short),
                    Value::Label(name) => {
                        if let Some(addresses) = labels_used_on.get_mut(name) {
                            addresses.push(code.pointer);
                        } else {
                            labels_used_on.insert(name.clone(), [code.pointer].to_vec());
                        }
                        // Since we don't know what to place here, just skip the argument (-1 for the opcode)
                        code.skip(size - 1);
                    }
                    Value::None => {}
                    Value::Array(_) => panic!("Array?"),
                }
                println!("Assembling {:?} as {:#04X}", opcode, opcode_number);
            }
            _ => {}
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
        for address in addresses_where_used {
            code.pointer = address + 1;
            code.push_long(labels[label] as u16);
        }
    }
    if !labels_used_on.is_empty() {
        return Err(Error::UndefLabel {
            labels: format!("{:?}", labels_used_on),
        });
    }
    Ok(code.cart)
}

mod tests {
    #[test]
    fn test_assemble() {
        use crate::assembler::assemble;
        use crate::parser::{parse_line, LineType};
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
        use crate::assembler::assemble;
        use crate::parser::{parse_line, LineType};
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

#[allow(dead_code)]
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
