use crate::{parser,opcodes,addressing_modes,Context};

pub fn assemble(ctx: &mut Context) {
    for token in ctx.tokens {
        let token: parser::Token = token;
        match token.line_data {
            parser::LineData::Code(code) => {
                let hex_opcode = opcodes::get_code(&code.name, code.addr_mode)
                    .expect(&format!("Unavailable addr mode for {:#?}", code));
                let mut hex = match code.arg {
                    Some(v) => {
                        addressing_modes::AddressingMode::assemble(&v, code.addr_mode, &ctx.labels)
                    }
                    None => vec![],
                };
                hex.push(hex_opcode as u8);
                hex.reverse(); // LSB endianness
                println!("{:02X?}", hex);
            }
            parser::LineData::Macro(r#macro) => {
                ctx.hex.push(&r#macro.bytes);
            }
            _ => (),
        }
    }
}