use crate::addressing_modes;
use crate::error::Error;
use crate::opcodes::get_code;
use crate::parser::{LineType, LineType::*, Value};

mod assemble;
mod code;
mod types;
pub use assemble::assemble;
pub use code::Code;
use types::LabelUse;
pub use types::Metadata;

mod tests {
    #[test]
    fn test_assemble() {
        use crate::assembler::assemble;
        use crate::parser::{parse_line, LineType};
        let metadata = super::Metadata::default();
        let test_code: &str = include_str!("../../assembly/general/basic_opcodes.asm");
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
