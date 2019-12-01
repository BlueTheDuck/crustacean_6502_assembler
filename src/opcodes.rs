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
    fn new(string: &str) -> Option<OpcodeName> {
        match string {
            _ => None,
            "ADC" => Some(OpcodeName::ADC),
            "AND" => Some(OpcodeName::AND),
            "ASL" => Some(OpcodeName::ASL),
            "BCC" => Some(OpcodeName::BCC),
            "BCS" => Some(OpcodeName::BCS),
            "BEQ" => Some(OpcodeName::BEQ),
            "BIT" => Some(OpcodeName::BIT),
            "BMI" => Some(OpcodeName::BMI),
            "BNE" => Some(OpcodeName::BNE),
            "BPL" => Some(OpcodeName::BPL),
            "BRK" => Some(OpcodeName::BRK),
            "BVC" => Some(OpcodeName::BVC),
            "BVS" => Some(OpcodeName::BVS),
            "CLC" => Some(OpcodeName::CLC),
            "CLD" => Some(OpcodeName::CLD),
            "CLI" => Some(OpcodeName::CLI),
            "CLV" => Some(OpcodeName::CLV),
            "CMP" => Some(OpcodeName::CMP),
            "CPX" => Some(OpcodeName::CPX),
            "CPY" => Some(OpcodeName::CPY),
            "DEC" => Some(OpcodeName::DEC),
            "DEX" => Some(OpcodeName::DEX),
            "DEY" => Some(OpcodeName::DEY),
            "EOR" => Some(OpcodeName::EOR),
            "INC" => Some(OpcodeName::INC),
            "INX" => Some(OpcodeName::INX),
            "INY" => Some(OpcodeName::INY),
            "JMP" => Some(OpcodeName::JMP),
            "JSR" => Some(OpcodeName::JSR),
            "LDA" => Some(OpcodeName::LDA),
            "LDX" => Some(OpcodeName::LDX),
            "LDY" => Some(OpcodeName::LDY),
            "LSR" => Some(OpcodeName::LSR),
            "NOP" => Some(OpcodeName::NOP),
            "ORA" => Some(OpcodeName::ORA),
            "PHA" => Some(OpcodeName::PHA),
            "PHP" => Some(OpcodeName::PHP),
            "PLA" => Some(OpcodeName::PLA),
            "PLP" => Some(OpcodeName::PLP),
            "ROL" => Some(OpcodeName::ROL),
            "ROR" => Some(OpcodeName::ROR),
            "RTI" => Some(OpcodeName::RTI),
            "RTS" => Some(OpcodeName::RTS),
            "SBC" => Some(OpcodeName::SBC),
            "SEC" => Some(OpcodeName::SEC),
            "SED" => Some(OpcodeName::SED),
            "SEI" => Some(OpcodeName::SEI),
            "STA" => Some(OpcodeName::STA),
            "STX" => Some(OpcodeName::STX),
            "STY" => Some(OpcodeName::STY),
            "TAX" => Some(OpcodeName::TAX),
            "TAY" => Some(OpcodeName::TAY),
            "TSX" => Some(OpcodeName::TSX),
            "TXA" => Some(OpcodeName::TXA),
            "TXS" => Some(OpcodeName::TXS),
            "TYA" => Some(OpcodeName::TYA),
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
//Opcode {name: "LDA",addr_mode: AddressingMode},
