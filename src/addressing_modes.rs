//A,abs,absX,absY,imm,impl,ind,indX,indY,rel,zpg,zpgX,zpgY
//1,  3,   3,   3,   2,  1,  3,   2,   2,  2,  2,   2,   2
pub static OP_SIZES: [usize;13] = [1,3,3,3,2,1,3,2,2,2,2,2,2];

#[derive(Debug,Copy,Clone)]
pub enum AddressingMode {
    A = 0,
    ABS,
    ABSX,
    ABSY,
    IMM,
    IMPL,
    IND,
    INDX,
    INDY,
    REL,
    ZPG,
    ZPGX,
    ZPGY,
}
impl AddressingMode {
    pub fn identify(text: &Option<String>) -> Self {
        let text = match text {
            Some(v) => v,
            None => return Self::IMPL
        };
        if text == "A" {
            Self::A
        } else if text.find("$")==Some(0) {
            match text.len()-1 {
                2 => Self::ZPG,
                4 => Self::ABS,
                _ => panic!("Wrong addr size"),
            }
        } else if text.find("#$")==Some(0) {
            Self::IMM
        } else if text == "" {
            Self::IMPL
        } else if text.to_ascii_lowercase()==*text {
            Self::ABS
        } else {
            eprintln!("Unidentifiable argument {}",text);
            unimplemented!("Unknown addressing mode");
        }
    }
}

