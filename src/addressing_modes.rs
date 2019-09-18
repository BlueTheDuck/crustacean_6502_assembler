macro_rules! bytify {
    ($var: expr,$count: literal) => {{
        let mut bytes: Vec<u8> = Vec::with_capacity($count);
        let mut value = $var;
        for i in 0..$count {
            bytes.push((value & 0xFF) as u8);
            value = value / 8;
        }
        bytes
    }};
}

//A,abs,absX,absY,imm,impl,ind,indX,indY,rel,zpg,zpgX,zpgY
//1,  3,   3,   3,   2,  1,  3,   2,   2,  2,  2,   2,   2
pub static OP_SIZES: [usize; 13] = [1, 3, 3, 3, 2, 1, 3, 2, 2, 2, 2, 2, 2];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AddressingMode {
    A = 0, // LSR A
    ABS,   // LDA $1234
    ABSX,  // STA $3000,X
    ABSY,  // AND $4000,Y
    IMM,   // LDA #10
    IMPL,  // CLC
    IND,   // JMP ($FFFC)
    INDX,  // LDA ($40,X)
    INDY,  // LDA ($40),Y
    REL,   // LABEL // +4
    ZPG,   // LDA $10
    ZPGX,  // LDA $10,X
    ZPGY,  // LDA $10,Y
}
impl AddressingMode {
    pub fn identify(text: &Option<String>) -> Self {
        let text = match text {
            Some(v) => v,
            None => return Self::IMPL,
        };
        if text == "A" {
            Self::A
        } else if text.find("$") == Some(0) {
            match text.len() - 1 {
                2 => Self::ZPG,
                4 => Self::ABS,
                _ => panic!("Wrong addr size"),
            }
        } else if text.find("#") == Some(0) {
            Self::IMM
        } else if text == "" {
            Self::IMPL
        } else if text.to_ascii_lowercase() == *text {
            Self::ABS
        } else {
            eprintln!("Unidentifiable argument {}", text);
            unimplemented!("Unknown addressing mode");
        }
    }
    pub fn assemble(
        text: &String,
        addr_mode: AddressingMode,
        label_set: &std::collections::HashMap<String, usize>,
    ) -> Vec<u8> {
        match addr_mode {
            AddressingMode::A => vec![],
            AddressingMode::ABS => {
                let address = match text.parse::<u16>() {
                    Ok(v) => v,
                    Err(_) => match label_set.get(text) {
                        Some(v) => *v as u16,
                        None => panic!("ABS"),
                    },
                };
                bytify!(address, 2)
            }
            AddressingMode::ABSX|AddressingMode::ABSY => {
                let value = u16::from_str_radix(&text[1..4], 16).expect("ABS x/y");
                bytify!(value,2)
            },
            AddressingMode::IMM => {
                let value = u8::from_str_radix(&text[1..], 16).expect("IMM");
                bytify!(value, 1)
            }
            AddressingMode::IMPL => unimplemented!("Addr mode assemble"),
            AddressingMode::IND => unimplemented!("Addr mode assemble"),
            AddressingMode::INDX => unimplemented!("Addr mode assemble"),
            AddressingMode::INDY => unimplemented!("Addr mode assemble"),
            AddressingMode::REL => unimplemented!("Addr mode assemble"),
            AddressingMode::ZPG => {
                let address = u8::from_str_radix(&text[1..], 16).expect("ABS");
                bytify!(address, 1)
            }
            AddressingMode::ZPGX => unimplemented!("Addr mode assemble"),
            AddressingMode::ZPGY => unimplemented!("Addr mode assemble"),
        }
    }
}
