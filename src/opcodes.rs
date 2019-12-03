use super::addressing_modes::AddressingMode;

/* pub fn get_code(name: &str, addr_mode: AddressingMode) -> Option<usize> {
    for (i, opcode) in OPCODES.iter().enumerate() {
        match opcode {
            None => continue,
            Some(opcode) => {
                if opcode.name == name && opcode.addr_mode == addr_mode {
                    return Some(i);
                }
            }
        }
    }
    return None;
} */
#[derive(Debug, PartialEq)]
pub enum OpcodeName {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
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
impl OpcodeName {
    pub fn new<'s, S: std::ops::Deref<Target = &'s str>>(string: &S) -> Result<OpcodeName, ()> {
        match **string {
            "ADC" => Ok(OpcodeName::ADC),
            "AND" => Ok(OpcodeName::AND),
            "ASL" => Ok(OpcodeName::ASL),
            "BCC" => Ok(OpcodeName::BCC),
            "BCS" => Ok(OpcodeName::BCS),
            "BEQ" => Ok(OpcodeName::BEQ),
            "BIT" => Ok(OpcodeName::BIT),
            "BMI" => Ok(OpcodeName::BMI),
            "BNE" => Ok(OpcodeName::BNE),
            "BPL" => Ok(OpcodeName::BPL),
            "BRK" => Ok(OpcodeName::BRK),
            "BVC" => Ok(OpcodeName::BVC),
            "BVS" => Ok(OpcodeName::BVS),
            "CLC" => Ok(OpcodeName::CLC),
            "CLD" => Ok(OpcodeName::CLD),
            "CLI" => Ok(OpcodeName::CLI),
            "CLV" => Ok(OpcodeName::CLV),
            "CMP" => Ok(OpcodeName::CMP),
            "CPX" => Ok(OpcodeName::CPX),
            "CPY" => Ok(OpcodeName::CPY),
            "DEC" => Ok(OpcodeName::DEC),
            "DEX" => Ok(OpcodeName::DEX),
            "DEY" => Ok(OpcodeName::DEY),
            "EOR" => Ok(OpcodeName::EOR),
            "INC" => Ok(OpcodeName::INC),
            "INX" => Ok(OpcodeName::INX),
            "INY" => Ok(OpcodeName::INY),
            "JMP" => Ok(OpcodeName::JMP),
            "JSR" => Ok(OpcodeName::JSR),
            "LDA" => Ok(OpcodeName::LDA),
            "LDX" => Ok(OpcodeName::LDX),
            "LDY" => Ok(OpcodeName::LDY),
            "LSR" => Ok(OpcodeName::LSR),
            "NOP" => Ok(OpcodeName::NOP),
            "ORA" => Ok(OpcodeName::ORA),
            "PHA" => Ok(OpcodeName::PHA),
            "PHP" => Ok(OpcodeName::PHP),
            "PLA" => Ok(OpcodeName::PLA),
            "PLP" => Ok(OpcodeName::PLP),
            "ROL" => Ok(OpcodeName::ROL),
            "ROR" => Ok(OpcodeName::ROR),
            "RTI" => Ok(OpcodeName::RTI),
            "RTS" => Ok(OpcodeName::RTS),
            "SBC" => Ok(OpcodeName::SBC),
            "SEC" => Ok(OpcodeName::SEC),
            "SED" => Ok(OpcodeName::SED),
            "SEI" => Ok(OpcodeName::SEI),
            "STA" => Ok(OpcodeName::STA),
            "STX" => Ok(OpcodeName::STX),
            "STY" => Ok(OpcodeName::STY),
            "TAX" => Ok(OpcodeName::TAX),
            "TAY" => Ok(OpcodeName::TAY),
            "TSX" => Ok(OpcodeName::TSX),
            "TXA" => Ok(OpcodeName::TXA),
            "TXS" => Ok(OpcodeName::TXS),
            "TYA" => Ok(OpcodeName::TYA),
            _ => Err(()),
        }
    }
}
pub struct Opcode {
    name: OpcodeName,
    addr_mode: AddressingMode,
}
pub const OPCODES: [Option<Opcode>; 256] = [
    Some(Opcode {
        name: OpcodeName::BRK,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: OpcodeName::ORA,
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: OpcodeName::ORA,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: OpcodeName::ASL,
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::PHP,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: OpcodeName::ORA,
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: OpcodeName::ASL,
        addr_mode: AddressingMode::A,
    }),
    None,
    None,
    Some(Opcode {
        name: OpcodeName::ORA,
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: OpcodeName::ASL,
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::BPL,
        addr_mode: AddressingMode::REL,
    }),
    Some(Opcode {
        name: OpcodeName::ORA,
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: OpcodeName::ORA,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: OpcodeName::ASL,
        addr_mode: AddressingMode::ZPGX,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::CLC,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: OpcodeName::ORA,
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: OpcodeName::ORA,
        addr_mode: AddressingMode::ABSX,
    }),
    Some(Opcode {
        name: OpcodeName::ASL,
        addr_mode: AddressingMode::ABSX,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::JSR,
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: OpcodeName::AND,
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    Some(Opcode {
        name: OpcodeName::BIT,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: OpcodeName::AND,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: OpcodeName::ROL,
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::PLP,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: OpcodeName::AND,
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: OpcodeName::ROL,
        addr_mode: AddressingMode::A,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::BIT,
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: OpcodeName::AND,
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: OpcodeName::ROL,
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::BMI,
        addr_mode: AddressingMode::REL,
    }),
    Some(Opcode {
        name: OpcodeName::AND,
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: OpcodeName::AND,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: OpcodeName::ROL,
        addr_mode: AddressingMode::ZPGX,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::SEC,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: OpcodeName::AND,
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: OpcodeName::AND,
        addr_mode: AddressingMode::ABSX,
    }),
    Some(Opcode {
        name: OpcodeName::ROL,
        addr_mode: AddressingMode::ABSX,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::RTI,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: OpcodeName::EOR,
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: OpcodeName::EOR,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: OpcodeName::LSR,
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::PHA,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: OpcodeName::EOR,
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: OpcodeName::LSR,
        addr_mode: AddressingMode::A,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::JMP,
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: OpcodeName::EOR,
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: OpcodeName::LSR,
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::BVC,
        addr_mode: AddressingMode::REL,
    }),
    Some(Opcode {
        name: OpcodeName::EOR,
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: OpcodeName::EOR,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: OpcodeName::LSR,
        addr_mode: AddressingMode::ZPGX,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::CLI,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: OpcodeName::EOR,
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: OpcodeName::EOR,
        addr_mode: AddressingMode::ABSX,
    }),
    Some(Opcode {
        name: OpcodeName::LSR,
        addr_mode: AddressingMode::ABSX,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::RTS,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: OpcodeName::ADC,
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: OpcodeName::ADC,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: OpcodeName::ROR,
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::PLA,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: OpcodeName::ADC,
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: OpcodeName::ROR,
        addr_mode: AddressingMode::A,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::JMP,
        addr_mode: AddressingMode::IND,
    }),
    Some(Opcode {
        name: OpcodeName::ADC,
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: OpcodeName::ROR,
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::BVS,
        addr_mode: AddressingMode::REL,
    }),
    Some(Opcode {
        name: OpcodeName::ADC,
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: OpcodeName::ADC,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: OpcodeName::ROR,
        addr_mode: AddressingMode::ZPGX,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::SEI,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: OpcodeName::ADC,
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: OpcodeName::ADC,
        addr_mode: AddressingMode::ABSX,
    }),
    Some(Opcode {
        name: OpcodeName::ROR,
        addr_mode: AddressingMode::ABSX,
    }),
    None,
    None,
    Some(Opcode {
        name: OpcodeName::STA,
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    Some(Opcode {
        name: OpcodeName::STY,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: OpcodeName::STA,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: OpcodeName::STX,
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::DEY,
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::TXA,
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::STY,
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: OpcodeName::STA,
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: OpcodeName::STX,
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::BCC,
        addr_mode: AddressingMode::REL,
    }),
    Some(Opcode {
        name: OpcodeName::STA,
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    Some(Opcode {
        name: OpcodeName::STY,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: OpcodeName::STA,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: OpcodeName::STX,
        addr_mode: AddressingMode::ZPGY,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::TYA,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: OpcodeName::STA,
        addr_mode: AddressingMode::ABSY,
    }),
    Some(Opcode {
        name: OpcodeName::TXS,
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    None,
    Some(Opcode {
        name: OpcodeName::STA,
        addr_mode: AddressingMode::ABSX,
    }),
    None,
    None,
    Some(Opcode {
        name: OpcodeName::LDY,
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: OpcodeName::LDA,
        addr_mode: AddressingMode::INDX,
    }),
    Some(Opcode {
        name: OpcodeName::LDX,
        addr_mode: AddressingMode::IMM,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::LDY,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: OpcodeName::LDA,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: OpcodeName::LDX,
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::TAY,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: OpcodeName::LDA,
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: OpcodeName::TAX,
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::LDY,
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: OpcodeName::LDA,
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: OpcodeName::LDX,
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::BCS,
        addr_mode: AddressingMode::REL,
    }),
    Some(Opcode {
        name: OpcodeName::LDA,
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    Some(Opcode {
        name: OpcodeName::LDY,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: OpcodeName::LDA,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: OpcodeName::LDX,
        addr_mode: AddressingMode::ZPGY,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::CLV,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: OpcodeName::LDA,
        addr_mode: AddressingMode::ABSY,
    }),
    Some(Opcode {
        name: OpcodeName::TSX,
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::LDY,
        addr_mode: AddressingMode::ABSX,
    }),
    Some(Opcode {
        name: OpcodeName::LDA,
        addr_mode: AddressingMode::ABSX,
    }),
    Some(Opcode {
        name: OpcodeName::LDX,
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::CPY,
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: OpcodeName::CMP,
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    Some(Opcode {
        name: OpcodeName::CPY,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: OpcodeName::CMP,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: OpcodeName::DEC,
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::INY,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: OpcodeName::CMP,
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: OpcodeName::DEX,
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::CPY,
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: OpcodeName::CMP,
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: OpcodeName::DEC,
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::BNE,
        addr_mode: AddressingMode::REL,
    }),
    Some(Opcode {
        name: OpcodeName::CMP,
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: OpcodeName::CMP,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: OpcodeName::DEC,
        addr_mode: AddressingMode::ZPGX,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::CLD,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: OpcodeName::CMP,
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: OpcodeName::CMP,
        addr_mode: AddressingMode::ABSX,
    }),
    Some(Opcode {
        name: OpcodeName::DEC,
        addr_mode: AddressingMode::ABSX,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::CPX,
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: OpcodeName::SBC,
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    Some(Opcode {
        name: OpcodeName::CPX,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: OpcodeName::SBC,
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: OpcodeName::INC,
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::INX,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: OpcodeName::SBC,
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: OpcodeName::NOP,
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::CPX,
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: OpcodeName::SBC,
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: OpcodeName::INC,
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::BEQ,
        addr_mode: AddressingMode::REL,
    }),
    Some(Opcode {
        name: OpcodeName::SBC,
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: OpcodeName::SBC,
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: OpcodeName::INC,
        addr_mode: AddressingMode::ZPGX,
    }),
    None,
    Some(Opcode {
        name: OpcodeName::SED,
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: OpcodeName::SBC,
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: OpcodeName::SBC,
        addr_mode: AddressingMode::ABSX,
    }),
    Some(Opcode {
        name: OpcodeName::INC,
        addr_mode: AddressingMode::ABSX,
    }),
    None,
];

mod test {
    use super::OpcodeName;
    #[test]
    fn test_opcode_name() {
        let strings = vec![("LDA", true), ("STA", true), ("JMP", true), ("xd", false)];
        for (string, is_ok) in strings.iter() {
            let res = OpcodeName::new(&string);
            println!("{} -> {:?}", string, res);
            assert_eq!(res.is_ok(), *is_ok);
        }
    }
}
