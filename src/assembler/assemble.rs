use super::{Code, LabelUse, LineType, LineType::*, Metadata, Value};
use crate::opcodes::get_code;
use crate::{addressing_modes, Error};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

macro_rules! impl_macros {
    ($type:ident,$arg:ident, $($name:literal => { $($pattern:pat => $code:expr),+ }),+  ) => {{
        match &*$type {
            $(
                $name => {
                    match $arg {
                        $($pattern => {$code}),+
                        _ => {
                            return Err(Error::Assembler{
                                cause: format!("The macro '{}' can't take {:?} as argument",$type,$arg)
                            });
                        }
                    }
                }
            ),+
            _ => {
                return Err(Error::Assembler{
                    cause: format!("The macro '{}' hasn't been implemented yet",$type)
                });
            }
        }
    }};
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
                impl_macros!(r#type,arg,
                    "org" => {
                        Value::Long(addr) => {code.pointer = addr as usize;}
                    },
                    "byte" => {
                        Value::Short(arg) => code.push_byte(arg)
                    },
                    "dw" => {
                        Value::Long(value) => {code.push_long(value)},
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
                    },
                    "incbin" => {
                        Value::Text(arg) => {let arg = String::from_utf8(arg.into_vec())
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
                        }}
                    },
                    "db" => {
                        Value::Array(array) => {
                            for value in array {
                                if let Value::Short(byte) = value {code.push_byte(byte);}
                            }
                        },
                        Value::Short(byte) => {code.push_byte(byte)}
                    }
                );
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
    Ok(*code)
}
