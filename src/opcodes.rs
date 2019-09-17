use super::addressing_modes::AddressingMode;

pub fn get_code(name: &str, addr_mode: AddressingMode) -> Option<usize> {
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
}

pub struct Opcode {
    name: &'static str,
    addr_mode: AddressingMode,
}
pub const OPCODES: [Option<Opcode>; 256] = [
    Some(Opcode {
        name: "BRK",
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: "ORA",
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: "ORA",
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: "ASL",
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(Opcode {
        name: "PHP",
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: "ORA",
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: "ASL",
        addr_mode: AddressingMode::A,
    }),
    None,
    None,
    Some(Opcode {
        name: "ORA",
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: "ASL",
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(Opcode {
        name: "BPL",
        addr_mode: AddressingMode::REL,
    }),
    Some(Opcode {
        name: "ORA",
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: "ORA",
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: "ASL",
        addr_mode: AddressingMode::ZPGX,
    }),
    None,
    Some(Opcode {
        name: "CLC",
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: "ORA",
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: "ORA",
        addr_mode: AddressingMode::ABSX,
    }),
    Some(Opcode {
        name: "ASL",
        addr_mode: AddressingMode::ABSX,
    }),
    None,
    Some(Opcode {
        name: "JSR",
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: "AND",
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    Some(Opcode {
        name: "BIT",
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: "AND",
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: "ROL",
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(Opcode {
        name: "PLP",
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: "AND",
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: "ROL",
        addr_mode: AddressingMode::A,
    }),
    None,
    Some(Opcode {
        name: "BIT",
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: "AND",
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: "ROL",
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(Opcode {
        name: "BMI",
        addr_mode: AddressingMode::REL,
    }),
    Some(Opcode {
        name: "AND",
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: "AND",
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: "ROL",
        addr_mode: AddressingMode::ZPGX,
    }),
    None,
    Some(Opcode {
        name: "SEC",
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: "AND",
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: "AND",
        addr_mode: AddressingMode::ABSX,
    }),
    Some(Opcode {
        name: "ROL",
        addr_mode: AddressingMode::ABSX,
    }),
    None,
    Some(Opcode {
        name: "RTI",
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: "EOR",
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: "EOR",
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: "LSR",
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(Opcode {
        name: "PHA",
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: "EOR",
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: "LSR",
        addr_mode: AddressingMode::A,
    }),
    None,
    Some(Opcode {
        name: "JMP",
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: "EOR",
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: "LSR",
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(Opcode {
        name: "BVC",
        addr_mode: AddressingMode::REL,
    }),
    Some(Opcode {
        name: "EOR",
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: "EOR",
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: "LSR",
        addr_mode: AddressingMode::ZPGX,
    }),
    None,
    Some(Opcode {
        name: "CLI",
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: "EOR",
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: "EOR",
        addr_mode: AddressingMode::ABSX,
    }),
    Some(Opcode {
        name: "LSR",
        addr_mode: AddressingMode::ABSX,
    }),
    None,
    Some(Opcode {
        name: "RTS",
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: "ADC",
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: "ADC",
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: "ROR",
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(Opcode {
        name: "PLA",
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: "ADC",
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: "ROR",
        addr_mode: AddressingMode::A,
    }),
    None,
    Some(Opcode {
        name: "JMP",
        addr_mode: AddressingMode::IND,
    }),
    Some(Opcode {
        name: "ADC",
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: "ROR",
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(Opcode {
        name: "BVS",
        addr_mode: AddressingMode::REL,
    }),
    Some(Opcode {
        name: "ADC",
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: "ADC",
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: "ROR",
        addr_mode: AddressingMode::ZPGX,
    }),
    None,
    Some(Opcode {
        name: "SEI",
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: "ADC",
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: "ADC",
        addr_mode: AddressingMode::ABSX,
    }),
    Some(Opcode {
        name: "ROR",
        addr_mode: AddressingMode::ABSX,
    }),
    None,
    None,
    Some(Opcode {
        name: "STA",
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    Some(Opcode {
        name: "STY",
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: "STA",
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: "STX",
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(Opcode {
        name: "DEY",
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    Some(Opcode {
        name: "TXA",
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    Some(Opcode {
        name: "STY",
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: "STA",
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: "STX",
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(Opcode {
        name: "BCC",
        addr_mode: AddressingMode::REL,
    }),
    Some(Opcode {
        name: "STA",
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    Some(Opcode {
        name: "STY",
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: "STA",
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: "STX",
        addr_mode: AddressingMode::ZPGY,
    }),
    None,
    Some(Opcode {
        name: "TYA",
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: "STA",
        addr_mode: AddressingMode::ABSY,
    }),
    Some(Opcode {
        name: "TXS",
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    None,
    Some(Opcode {
        name: "STA",
        addr_mode: AddressingMode::ABSX,
    }),
    None,
    None,
    Some(Opcode {
        name: "LDY",
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: "LDA",
        addr_mode: AddressingMode::INDX,
    }),
    Some(Opcode {
        name: "LDX",
        addr_mode: AddressingMode::IMM,
    }),
    None,
    Some(Opcode {
        name: "LDY",
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: "LDA",
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: "LDX",
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(Opcode {
        name: "TAY",
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: "LDA",
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: "TAX",
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    Some(Opcode {
        name: "LDY",
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: "LDA",
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: "LDX",
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(Opcode {
        name: "BCS",
        addr_mode: AddressingMode::REL,
    }),
    Some(Opcode {
        name: "LDA",
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    Some(Opcode {
        name: "LDY",
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: "LDA",
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: "LDX",
        addr_mode: AddressingMode::ZPGY,
    }),
    None,
    Some(Opcode {
        name: "CLV",
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: "LDA",
        addr_mode: AddressingMode::ABSY,
    }),
    Some(Opcode {
        name: "TSX",
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    Some(Opcode {
        name: "LDY",
        addr_mode: AddressingMode::ABSX,
    }),
    Some(Opcode {
        name: "LDA",
        addr_mode: AddressingMode::ABSX,
    }),
    Some(Opcode {
        name: "LDX",
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    Some(Opcode {
        name: "CPY",
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: "CMP",
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    Some(Opcode {
        name: "CPY",
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: "CMP",
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: "DEC",
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(Opcode {
        name: "INY",
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: "CMP",
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: "DEX",
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    Some(Opcode {
        name: "CPY",
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: "CMP",
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: "DEC",
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(Opcode {
        name: "BNE",
        addr_mode: AddressingMode::REL,
    }),
    Some(Opcode {
        name: "CMP",
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: "CMP",
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: "DEC",
        addr_mode: AddressingMode::ZPGX,
    }),
    None,
    Some(Opcode {
        name: "CLD",
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: "CMP",
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: "CMP",
        addr_mode: AddressingMode::ABSX,
    }),
    Some(Opcode {
        name: "DEC",
        addr_mode: AddressingMode::ABSX,
    }),
    None,
    Some(Opcode {
        name: "CPX",
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: "SBC",
        addr_mode: AddressingMode::INDX,
    }),
    None,
    None,
    Some(Opcode {
        name: "CPX",
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: "SBC",
        addr_mode: AddressingMode::ZPG,
    }),
    Some(Opcode {
        name: "INC",
        addr_mode: AddressingMode::ZPG,
    }),
    None,
    Some(Opcode {
        name: "INX",
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: "SBC",
        addr_mode: AddressingMode::IMM,
    }),
    Some(Opcode {
        name: "NOP",
        addr_mode: AddressingMode::IMPL,
    }),
    None,
    Some(Opcode {
        name: "CPX",
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: "SBC",
        addr_mode: AddressingMode::ABS,
    }),
    Some(Opcode {
        name: "INC",
        addr_mode: AddressingMode::ABS,
    }),
    None,
    Some(Opcode {
        name: "BEQ",
        addr_mode: AddressingMode::REL,
    }),
    Some(Opcode {
        name: "SBC",
        addr_mode: AddressingMode::INDY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: "SBC",
        addr_mode: AddressingMode::ZPGX,
    }),
    Some(Opcode {
        name: "INC",
        addr_mode: AddressingMode::ZPGX,
    }),
    None,
    Some(Opcode {
        name: "SED",
        addr_mode: AddressingMode::IMPL,
    }),
    Some(Opcode {
        name: "SBC",
        addr_mode: AddressingMode::ABSY,
    }),
    None,
    None,
    None,
    Some(Opcode {
        name: "SBC",
        addr_mode: AddressingMode::ABSX,
    }),
    Some(Opcode {
        name: "INC",
        addr_mode: AddressingMode::ABSX,
    }),
    None,
];
//Opcode {name: "LDA",addr_mode: AddressingMode},
