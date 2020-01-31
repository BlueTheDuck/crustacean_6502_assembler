use crate::addressing_modes;
use crate::error::Error;
use crate::opcodes::get_code;
use crate::parser::{LineType, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

/// 0: Low 1: High
fn big_to_little_endian(value: u16) -> (u8, u8) {
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
    /// Take a u16, convert it to little endian then place it on self.pointer, finally increment by 2
    fn push_long(&mut self, long: u16) {
        let long = big_to_little_endian(long);
        self.push_byte(long.0);
        self.push_byte(long.1);
    }
    /// Add <amount> to self.pointer
    fn skip(&mut self, amount: usize) {
        self.pointer += amount;
    }
}
pub struct Metadata {
    /// Where to find include files
    pub search_path: std::path::PathBuf,
}
impl std::default::Default for Metadata {
    fn default() -> Self {
        Metadata {
            search_path: std::path::PathBuf::from(""),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct LabelUse {
    /// Where was this label used?
    location: usize,
    /// Was this label used for a relative (branch) instruction
    is_relative: bool,
}

pub fn assemble(parsed_code: Vec<LineType>, metadata: &Metadata) -> Result<[u8; 0x10000], Error> {
    let mut code = Code::new(); // code: holds the code
    let mut labels: HashMap<String, usize> = HashMap::default(); // labels: holds the addrs of each label
    let mut labels_used_on: HashMap<String, Vec<LabelUse>> = HashMap::default(); // labels_used_on: holds the addresses where a label was used
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
                        let label_use = LabelUse {
                            location: code.pointer,
                            is_relative: opcode.name.is_branch_op(),
                        };
                        if let Some(addresses) = labels_used_on.get_mut(name) {
                            addresses.push(label_use);
                        } else {
                            labels_used_on.insert(name.clone(), vec![label_use]);
                        }
                        // Since we don't know what to place here, just skip the argument (-1 for the opcode)
                        code.skip(size - 1);
                    }
                    Value::Array(arr) => {
                        return Err(Error::Assembler {
                            cause: format!(
                                "Arrays haven't been implemented yet (Tried to use {:?})",
                                arr
                            ),
                        })
                    }
                    Value::Text(txt) => {
                        return Err(Error::Assembler {
                            cause: format!(
                                "Text literals haven't been implemented yet (Tried to use {:?})",
                                txt
                            ),
                        })
                    }
                    Value::None => {}
                }
                println!("Assembling {:?} as {:#04X}", opcode, opcode_number);
            }
            LineType::Macro(r#type, arg) => {
                println!("Interpreting macro {:?} {:X?}", r#type, arg);
                match &*r#type {
                    "org" => {
                        if let Value::Long(addr) = arg {
                            code.pointer = addr.into();
                        } else {
                            return Err(Error::Assembler {
                                cause: ".org can only be used with long literals".to_string(),
                            });
                        }
                    }
                    "byte" => match arg {
                        Value::Short(value) => code.push_byte(value),
                        Value::Long(value) => code.push_long(value),
                        _ => {}
                    },
                    "dw" => match arg {
                        Value::Long(value) => code.push_long(value),
                        Value::Label(name) => {
                            let label_use = LabelUse {
                                location: code.pointer,
                                is_relative: false,
                            };
                            if let Some(addresses) = labels_used_on.get_mut(&name) {
                                addresses.push(label_use);
                            } else {
                                labels_used_on.insert(name.clone(), vec![label_use]);
                            }
                            code.pointer += 2;
                        }
                        _ => {
                            unimplemented!("{:?} is not implemented for .dw", arg);
                        }
                    },
                    "incbin" if arg.is_text() => {
                        if let Value::Text(arg) = arg {
                            let arg = String::from_utf8(arg.into_vec())
                                .expect("File name wasn't an UTF-8 string");
                            let mut path =
                                std::path::PathBuf::from(metadata.search_path.as_os_str());
                            path.push(arg);
                            println!("Including bytes from {}", path.display());
                            let mut file = File::open(path)?;
                            let file_size = file.metadata()?.len();
                            let mut buffer = Vec::with_capacity(file_size as usize);
                            file.read_to_end(&mut buffer)?;
                            println!("Inserting {} bytes", file_size);
                            for byte in buffer {
                                code.push_byte(byte);
                            }
                        } else {
                        }
                    }
                    "db" => match arg {
                        Value::Array(array) => {
                            for value in array {
                                match value {
                                    Value::Short(byte) => code.push_byte(byte),
                                    _ => continue,
                                };
                            }
                        }
                        Value::Short(byte) => code.push_byte(byte),
                        _ => {
                            return Err(Error::Assembler {
                                cause: format!("{:?} isn't valid for .db", arg),
                            })
                        }
                    },
                    _ => {
                        return Err(Error::Assembler {
                            cause: format!(
                                "Error in macro invocation:\n Name: {}\n Arg {:?} ",
                                r#type, arg
                            ),
                        });
                    }
                }
            }
        };
    }
    // Iterate through all the defined labels
    // removing them from the list of used_labels
    // and placing their address in the whitespaces left
    // on the assembling stage
    for (label, l_address) in labels.keys().zip(labels.values()) {
        // Skip if the label wasn't used
        let addresses_where_used: Vec<LabelUse> = match labels_used_on.remove(label) {
            None => continue,
            Some(v) => v,
        };
        for address in addresses_where_used {
            code.pointer = address.location;
            if address.is_relative {
                // Calc de diff between the 2 addresses
                // The +1 is to skip the opcode's argument
                let relative = (*l_address as isize) - (address.location as isize + 1);
                let relative = (relative & 0xFF) as u8;
                code.push_byte(relative);
            } else {
                code.push_long(labels[label] as u16);
            }
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
        let metadata = super::Metadata::default();
        let test_code: &str = include_str!("../assembly/general/basic_opcodes.asm");
        let test_code: Vec<LineType> = test_code
            .lines()
            .map(|l: &str| parse_line(l.as_bytes()).unwrap().1)
            .collect();
        let code: [u8; 0x10000] =
            assemble(test_code, &metadata).expect("This shouldn't have errored");
        super::dump(&code, Some(0x80), Some(0x80));
        assert_eq!(code[0x0000..0x0005], [0xA9, 0xFF, 0x85, 0xFF, 0x18]);
    }
    #[test]
    fn test_labels() {
        use crate::assembler::assemble;
        use crate::parser::{parse_line, LineType};
        let metadata = super::Metadata::default();
        let test_code: &str = "\tLDA main";
        let test_code: Vec<LineType> = test_code
            .lines()
            .map(|l: &str| parse_line(l.as_bytes()).unwrap().1)
            .collect();
        assert!(assemble(test_code, &metadata).is_err());
        let test_code: &str = "main:\n\tLDA main";
        let test_code: Vec<LineType> = test_code
            .lines()
            .map(|l: &str| parse_line(l.as_bytes()).unwrap().1)
            .collect();
        let code = assemble(test_code, &metadata).unwrap();
        assert_eq!(code[0x0000..0x0003], [0xAD, 0x00, 0x00]);
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
