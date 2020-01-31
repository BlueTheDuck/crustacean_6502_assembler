use super::arguments::parse_argument;
use super::helpers::{eof, margin};
use super::nom;
use super::types::{LineType, Opcode};
use super::OpcodeType;
use super::{AddressingMode, ArgumentType, Value};
use nom::{bytes::complete as bytes, character, combinator, IResult};
use std::str::from_utf8;

// #region Parsers
fn parse_opcode_line(input: &[u8]) -> IResult<&[u8], Opcode> {
    let (input, _) = margin(input)?;
    let (input, name) = bytes::take_while_m_n(3, 3, character::is_alphabetic)(input)?;
    let name: OpcodeType =
        match OpcodeType::identify(&&from_utf8(name).expect("Couldn't convert [u8] to str")) {
            Ok(v) => v,
            Err(_) => return Err(nom::Err::Failure((input, nom::error::ErrorKind::MapRes))),
        };
    let (input, mut arg): (_, ArgumentType) = parse_argument(input)?;
    // If the OPCODE is any kind of branch, then we DO NOT USE ABS as addressing mode,
    // even if its argument is a label, so we manually patch this
    if name.is_branch_op() {
        arg = (AddressingMode::REL, arg.1);
    }
    Ok((input, Opcode { name, arg }))
}

fn label_def(input: &[u8]) -> IResult<&[u8], String> {
    let (input, value) = combinator::map_res(character::complete::alphanumeric1, |v: &[u8]| {
        String::from_utf8(v.to_vec())
    })(input)?;
    let (input, _) = character::complete::char(':')(input)?;
    let (input, _) = eof(input)?;
    Ok((input, value))
}

fn parse_macro(input: &[u8]) -> IResult<&[u8], (String, Value)> {
    let (input, _) = character::complete::char('.')(input)?;
    let (input, name) = character::complete::alpha1(input)?;
    let name = String::from_utf8(name.to_vec())
        .map_err(|_| nom::Err::Error((input, nom::error::ErrorKind::MapRes)))?;
    let (input, (_, arg)) = parse_argument(input)?;
    Ok((input, (name, arg)))
}
// #endregion

named!(
    pub parse_line<LineType>,
    alt!(
        label_def => { |r|LineType::LabelDef(r) }|
        parse_opcode_line => { |r|LineType::Opcode(r) }|
        parse_macro => { |(n,a)|LineType::Macro(n,a) }
    )
);

mod tests {
    #[test]
    fn test_macro() {
        use super::parse_macro;
        let res = parse_macro(&b".org $8000"[..]);
        println!("{:?}", res);
    }
    #[test]
    fn test_opcode() {
        use super::parse_opcode_line;
        let tests = [
            &b"\tLDA $FF"[..],
            &b"  STA $F00F"[..],
            &b"  ROL"[..],
            &b"    LDX #$FF"[..],
        ];
        for test in tests.iter() {
            println!(
                "Test: {:?} Result: {:?}",
                std::str::from_utf8(test).unwrap(),
                parse_opcode_line(test)
            );
        }
    }
    #[test]
    fn test_line() {
        use crate::parser::parse_line;
        let code: &str = include_str!("../../assembly/general/custom.asm");
        for l in code.lines() {
            println!("{:X?}", parse_line(l.as_bytes()));
        }
    }
}
