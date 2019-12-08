use super::addressing_modes::AddressingMode;
use crate::error::Error;

pub fn get_code(name: OpcodeType, addr_mode: AddressingMode) -> Result<u8, Error> {
    for (i, opcode) in OPCODES.iter().enumerate() {
        match opcode {
            None => continue,
            Some(ref opcode) => {
                if opcode.name == name && opcode.addr_mode == addr_mode {
                    return Ok((i & 0xFF) as u8);
                }
            }
        }
    }
    Err(Error::UnkownOpcode { name: name.into() })
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum OpcodeType {
    ADC,
    AND,
    ASL,
    BCC, // Branch ops
    BCS, // Branch ops
    BEQ, // Branch ops
    BIT,
    BMI, // Branch ops
    BNE, // Branch ops
    BPL, // Branch ops
    BRK,
    BVC, // Branch ops
    BVS, // Branch ops
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,
}
impl OpcodeType {
    pub fn identify<'s, S: std::ops::Deref<Target = &'s str>>(
        string: &S,
    ) -> Result<OpcodeType, ()> {
        match **string {
            "ADC" => Ok(OpcodeType::ADC),
            "AND" => Ok(OpcodeType::AND),
            "ASL" => Ok(OpcodeType::ASL),
            "BCC" => Ok(OpcodeType::BCC),
            "BCS" => Ok(OpcodeType::BCS),
            "BEQ" => Ok(OpcodeType::BEQ),
            "BIT" => Ok(OpcodeType::BIT),
            "BMI" => Ok(OpcodeType::BMI),
            "BNE" => Ok(OpcodeType::BNE),
            "BPL" => Ok(OpcodeType::BPL),
            "BRK" => Ok(OpcodeType::BRK),
            "BVC" => Ok(OpcodeType::BVC),
            "BVS" => Ok(OpcodeType::BVS),
            "CLC" => Ok(OpcodeType::CLC),
            "CLD" => Ok(OpcodeType::CLD),
            "CLI" => Ok(OpcodeType::CLI),
            "CLV" => Ok(OpcodeType::CLV),
            "CMP" => Ok(OpcodeType::CMP),
            "CPX" => Ok(OpcodeType::CPX),
            "CPY" => Ok(OpcodeType::CPY),
            "DEC" => Ok(OpcodeType::DEC),
            "DEX" => Ok(OpcodeType::DEX),
            "DEY" => Ok(OpcodeType::DEY),
            "EOR" => Ok(OpcodeType::EOR),
            "INC" => Ok(OpcodeType::INC),
            "INX" => Ok(OpcodeType::INX),
            "INY" => Ok(OpcodeType::INY),
            "JMP" => Ok(OpcodeType::JMP),
            "JSR" => Ok(OpcodeType::JSR),
            "LDA" => Ok(OpcodeType::LDA),
            "LDX" => Ok(OpcodeType::LDX),
            "LDY" => Ok(OpcodeType::LDY),
            "LSR" => Ok(OpcodeType::LSR),
            "NOP" => Ok(OpcodeType::NOP),
            "ORA" => Ok(OpcodeType::ORA),
            "PHA" => Ok(OpcodeType::PHA),
            "PHP" => Ok(OpcodeType::PHP),
            "PLA" => Ok(OpcodeType::PLA),
            "PLP" => Ok(OpcodeType::PLP),
            "ROL" => Ok(OpcodeType::ROL),
            "ROR" => Ok(OpcodeType::ROR),
            "RTI" => Ok(OpcodeType::RTI),
            "RTS" => Ok(OpcodeType::RTS),
            "SBC" => Ok(OpcodeType::SBC),
            "SEC" => Ok(OpcodeType::SEC),
            "SED" => Ok(OpcodeType::SED),
            "SEI" => Ok(OpcodeType::SEI),
            "STA" => Ok(OpcodeType::STA),
            "STX" => Ok(OpcodeType::STX),
            "STY" => Ok(OpcodeType::STY),
            "TAX" => Ok(OpcodeType::TAX),
            "TAY" => Ok(OpcodeType::TAY),
            "TSX" => Ok(OpcodeType::TSX),
            "TXA" => Ok(OpcodeType::TXA),
            "TXS" => Ok(OpcodeType::TXS),
            "TYA" => Ok(OpcodeType::TYA),
            _ => Err(()),
        }
    }
    pub fn is_branch_op(self) -> bool {
        use OpcodeType::*;
        let branch_ops = [BCC, BCS, BEQ, BMI, BNE, BPL, BVC, BVS];
        branch_ops.contains(&self)
    }
}
impl std::convert::Into<String> for OpcodeType {
    fn into(self) -> String {
        format!("{:?}", self)
    }
}
#[derive(Debug)]
pub struct OpcodeData {
    name: OpcodeType,
    addr_mode: AddressingMode,
}

pub const OPCODES: [Option<OpcodeData>; 256] = [
    Some(OpcodeData {
        name: OpcodeType::BRK,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(OpcodeData {
        name: OpcodeType::ORA,
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::ORA,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(OpcodeData {
        name: OpcodeType::ASL,
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::PHP,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(OpcodeData {
        name: OpcodeType::ORA,
        addr_mode: AddressingMode::IMM,
    }),
    Some(OpcodeData {
        name: OpcodeType::ASL,
        addr_mode: AddressingMode::A,
    }),
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::ORA,
        addr_mode: AddressingMode::ABS,
    }),
    Some(OpcodeData {
        name: OpcodeType::ASL,
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::BPL,
        addr_mode: AddressingMode::REL,
    }),
    Some(OpcodeData {
        name: OpcodeType::ORA,
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::ORA,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(OpcodeData {
        name: OpcodeType::ASL,
        addr_mode: AddressingMode::ZPGX,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::CLC,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(OpcodeData {
        name: OpcodeType::ORA,
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::ORA,
        addr_mode: AddressingMode::ABSX,
    }),
    Some(OpcodeData {
        name: OpcodeType::ASL,
        addr_mode: AddressingMode::ABSX,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::JSR,
        addr_mode: AddressingMode::ABS,
    }),
    Some(OpcodeData {
        name: OpcodeType::AND,
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::BIT,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(OpcodeData {
        name: OpcodeType::AND,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(OpcodeData {
        name: OpcodeType::ROL,
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::PLP,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(OpcodeData {
        name: OpcodeType::AND,
        addr_mode: AddressingMode::IMM,
    }),
    Some(OpcodeData {
        name: OpcodeType::ROL,
        addr_mode: AddressingMode::A,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::BIT,
        addr_mode: AddressingMode::ABS,
    }),
    Some(OpcodeData {
        name: OpcodeType::AND,
        addr_mode: AddressingMode::ABS,
    }),
    Some(OpcodeData {
        name: OpcodeType::ROL,
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::BMI,
        addr_mode: AddressingMode::REL,
    }),
    Some(OpcodeData {
        name: OpcodeType::AND,
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::AND,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(OpcodeData {
        name: OpcodeType::ROL,
        addr_mode: AddressingMode::ZPGX,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::SEC,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(OpcodeData {
        name: OpcodeType::AND,
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::AND,
        addr_mode: AddressingMode::ABSX,
    }),
    Some(OpcodeData {
        name: OpcodeType::ROL,
        addr_mode: AddressingMode::ABSX,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::RTI,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(OpcodeData {
        name: OpcodeType::EOR,
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::EOR,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(OpcodeData {
        name: OpcodeType::LSR,
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::PHA,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(OpcodeData {
        name: OpcodeType::EOR,
        addr_mode: AddressingMode::IMM,
    }),
    Some(OpcodeData {
        name: OpcodeType::LSR,
        addr_mode: AddressingMode::A,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::JMP,
        addr_mode: AddressingMode::ABS,
    }),
    Some(OpcodeData {
        name: OpcodeType::EOR,
        addr_mode: AddressingMode::ABS,
    }),
    Some(OpcodeData {
        name: OpcodeType::LSR,
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::BVC,
        addr_mode: AddressingMode::REL,
    }),
    Some(OpcodeData {
        name: OpcodeType::EOR,
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::EOR,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(OpcodeData {
        name: OpcodeType::LSR,
        addr_mode: AddressingMode::ZPGX,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::CLI,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(OpcodeData {
        name: OpcodeType::EOR,
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::EOR,
        addr_mode: AddressingMode::ABSX,
    }),
    Some(OpcodeData {
        name: OpcodeType::LSR,
        addr_mode: AddressingMode::ABSX,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::RTS,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(OpcodeData {
        name: OpcodeType::ADC,
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::ADC,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(OpcodeData {
        name: OpcodeType::ROR,
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::PLA,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(OpcodeData {
        name: OpcodeType::ADC,
        addr_mode: AddressingMode::IMM,
    }),
    Some(OpcodeData {
        name: OpcodeType::ROR,
        addr_mode: AddressingMode::A,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::JMP,
        addr_mode: AddressingMode::IND,
    }),
    Some(OpcodeData {
        name: OpcodeType::ADC,
        addr_mode: AddressingMode::ABS,
    }),
    Some(OpcodeData {
        name: OpcodeType::ROR,
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::BVS,
        addr_mode: AddressingMode::REL,
    }),
    Some(OpcodeData {
        name: OpcodeType::ADC,
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::ADC,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(OpcodeData {
        name: OpcodeType::ROR,
        addr_mode: AddressingMode::ZPGX,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::SEI,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(OpcodeData {
        name: OpcodeType::ADC,
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::ADC,
        addr_mode: AddressingMode::ABSX,
    }),
    Some(OpcodeData {
        name: OpcodeType::ROR,
        addr_mode: AddressingMode::ABSX,
    }),
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::STA,
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::STY,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(OpcodeData {
        name: OpcodeType::STA,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(OpcodeData {
        name: OpcodeType::STX,
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::DEY,
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::TXA,
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::STY,
        addr_mode: AddressingMode::ABS,
    }),
    Some(OpcodeData {
        name: OpcodeType::STA,
        addr_mode: AddressingMode::ABS,
    }),
    Some(OpcodeData {
        name: OpcodeType::STX,
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::BCC,
        addr_mode: AddressingMode::REL,
    }),
    Some(OpcodeData {
        name: OpcodeType::STA,
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::STY,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(OpcodeData {
        name: OpcodeType::STA,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(OpcodeData {
        name: OpcodeType::STX,
        addr_mode: AddressingMode::ZPGY,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::TYA,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(OpcodeData {
        name: OpcodeType::STA,
        addr_mode: AddressingMode::ABSY,
    }),
    Some(OpcodeData {
        name: OpcodeType::TXS,
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::STA,
        addr_mode: AddressingMode::ABSX,
    }),
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::LDY,
        addr_mode: AddressingMode::IMM,
    }),
    Some(OpcodeData {
        name: OpcodeType::LDA,
        addr_mode: AddressingMode::INDX,
    }),
    Some(OpcodeData {
        name: OpcodeType::LDX,
        addr_mode: AddressingMode::IMM,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::LDY,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(OpcodeData {
        name: OpcodeType::LDA,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(OpcodeData {
        name: OpcodeType::LDX,
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::TAY,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(OpcodeData {
        name: OpcodeType::LDA,
        addr_mode: AddressingMode::IMM,
    }),
    Some(OpcodeData {
        name: OpcodeType::TAX,
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::LDY,
        addr_mode: AddressingMode::ABS,
    }),
    Some(OpcodeData {
        name: OpcodeType::LDA,
        addr_mode: AddressingMode::ABS,
    }),
    Some(OpcodeData {
        name: OpcodeType::LDX,
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::BCS,
        addr_mode: AddressingMode::REL,
    }),
    Some(OpcodeData {
        name: OpcodeType::LDA,
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::LDY,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(OpcodeData {
        name: OpcodeType::LDA,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(OpcodeData {
        name: OpcodeType::LDX,
        addr_mode: AddressingMode::ZPGY,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::CLV,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(OpcodeData {
        name: OpcodeType::LDA,
        addr_mode: AddressingMode::ABSY,
    }),
    Some(OpcodeData {
        name: OpcodeType::TSX,
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::LDY,
        addr_mode: AddressingMode::ABSX,
    }),
    Some(OpcodeData {
        name: OpcodeType::LDA,
        addr_mode: AddressingMode::ABSX,
    }),
    Some(OpcodeData {
        name: OpcodeType::LDX,
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::CPY,
        addr_mode: AddressingMode::IMM,
    }),
    Some(OpcodeData {
        name: OpcodeType::CMP,
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::CPY,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(OpcodeData {
        name: OpcodeType::CMP,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(OpcodeData {
        name: OpcodeType::DEC,
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::INY,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(OpcodeData {
        name: OpcodeType::CMP,
        addr_mode: AddressingMode::IMM,
    }),
    Some(OpcodeData {
        name: OpcodeType::DEX,
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::CPY,
        addr_mode: AddressingMode::ABS,
    }),
    Some(OpcodeData {
        name: OpcodeType::CMP,
        addr_mode: AddressingMode::ABS,
    }),
    Some(OpcodeData {
        name: OpcodeType::DEC,
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::BNE,
        addr_mode: AddressingMode::REL,
    }),
    Some(OpcodeData {
        name: OpcodeType::CMP,
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::CMP,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(OpcodeData {
        name: OpcodeType::DEC,
        addr_mode: AddressingMode::ZPGX,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::CLD,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(OpcodeData {
        name: OpcodeType::CMP,
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::CMP,
        addr_mode: AddressingMode::ABSX,
    }),
    Some(OpcodeData {
        name: OpcodeType::DEC,
        addr_mode: AddressingMode::ABSX,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::CPX,
        addr_mode: AddressingMode::IMM,
    }),
    Some(OpcodeData {
        name: OpcodeType::SBC,
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::CPX,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(OpcodeData {
        name: OpcodeType::SBC,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(OpcodeData {
        name: OpcodeType::INC,
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::INX,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(OpcodeData {
        name: OpcodeType::SBC,
        addr_mode: AddressingMode::IMM,
    }),
    Some(OpcodeData {
        name: OpcodeType::NOP,
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::CPX,
        addr_mode: AddressingMode::ABS,
    }),
    Some(OpcodeData {
        name: OpcodeType::SBC,
        addr_mode: AddressingMode::ABS,
    }),
    Some(OpcodeData {
        name: OpcodeType::INC,
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::BEQ,
        addr_mode: AddressingMode::REL,
    }),
    Some(OpcodeData {
        name: OpcodeType::SBC,
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::SBC,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(OpcodeData {
        name: OpcodeType::INC,
        addr_mode: AddressingMode::ZPGX,
    }),
    None,
    Some(OpcodeData {
        name: OpcodeType::SED,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(OpcodeData {
        name: OpcodeType::SBC,
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    None,
    None,
    Some(OpcodeData {
        name: OpcodeType::SBC,
        addr_mode: AddressingMode::ABSX,
    }),
    Some(OpcodeData {
        name: OpcodeType::INC,
        addr_mode: AddressingMode::ABSX,
    }),
    None,
];

mod test {
    #[test]
    fn test_opcode_name() {
        use super::OpcodeType;
        let strings = vec![("LDA", true), ("STA", true), ("JMP", true), ("xd", false)];
        for (string, is_ok) in strings.iter() {
            let res = OpcodeType::identify(&string);
            println!("{} -> {:?}", string, res);
            assert_eq!(res.is_ok(), *is_ok);
        }
    }
}
