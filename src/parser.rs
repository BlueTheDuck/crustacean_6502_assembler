use crate::addressing_modes::AddressingMode;
use crate::nom;
use crate::opcodes::OpcodeType;
use nom::IResult;
use nom::{bytes::complete as bytes, character, combinator};
use std::str::from_utf8;

#[derive(Debug, PartialEq)]
enum Value {
    Short(u8),
    Long(u16),
    Label(String),
}
type ArgumentType = (AddressingMode, Value);

#[derive(Debug)]
pub struct Opcode {
    name: OpcodeType,
    arg: Option<ArgumentType>,
}
#[derive(Debug)]
pub enum LineType {
    Opcode(Opcode),
    LabelDef(String),
}

fn u8_to_hex(v: &[u8]) -> Result<usize, ()> {
    let text = from_utf8(v).map_err(|_| ())?;
    usize::from_str_radix(text, 16).map_err(|_| ())
}
// #region Arguments
named!(eof, eof!());

fn hex_addr_short(input: &[u8]) -> IResult<&[u8], ArgumentType> {
    let (input, _) = character::streaming::char('$')(input)?;
    let (input, value) = combinator::map_res(bytes::take(2usize), u8_to_hex)(input)?;
    let (input, _) = eof(input)?;
    Ok((input, (AddressingMode::ZPG, Value::Short(value as u8))))
}
fn hex_addr_long(input: &[u8]) -> IResult<&[u8], ArgumentType> {
    let (input, _) = character::streaming::char('$')(input)?;
    let (input, value) = combinator::map_res(bytes::take(4usize), u8_to_hex)(input)?;
    let (input, _) = eof(input)?;
    Ok((input, (AddressingMode::ABS, Value::Long(value as u16))))
}
fn hex_value(input: &[u8]) -> IResult<&[u8], ArgumentType> {
    let (input, _) = character::streaming::char('#')(input)?;
    let (input, _) = character::streaming::char('$')(input)?;
    let (input, value) = combinator::map_res(bytes::take(2usize), u8_to_hex)(input)?;
    let (input, _) = eof(input)?;
    Ok((input, (AddressingMode::IMM, Value::Short(value as u8))))
}
// TODO: Improve label recognition
fn label_name(input: &[u8]) -> IResult<&[u8], ArgumentType> {
    let (input, value) = combinator::map_res(character::complete::alphanumeric1, |s: &[u8]| {
        String::from_utf8(s.to_vec())
    })(input)?;
    let (input, _) = eof(input)?;
    Ok((input, (AddressingMode::ABS, Value::Label(value))))
}
fn argument(input: &[u8]) -> IResult<&[u8], ArgumentType> {
    nom::branch::alt((hex_addr_short, hex_addr_long, hex_value, label_name))(input)
}
// #endregion
// #region Types
// TODO: Improve margin recognition
named!(margin<&[u8]>, take_while!(character::is_space));
fn parse_argument(input: &[u8]) -> IResult<&[u8], Option<ArgumentType>> {
    if input.is_empty() {
        return Ok((input, None));
    }
    let (input, _) = character::streaming::char(' ')(input)?;
    if input.is_empty() {
        return Ok((input, None));
    }
    let (input, arg) = argument(input)?;
    if !input.is_empty() {
        return Err(nom::Err::Error((input, nom::error::ErrorKind::TooLarge)));
    }
    Ok((input, Some(arg)))
}
fn parse_opcode_line(input: &[u8]) -> IResult<&[u8], Opcode> {
    let (input, _) = margin(input)?;
    let (input, name) = bytes::take_while_m_n(3, 3, character::is_alphabetic)(input)?;
    let name = match OpcodeType::identify(&&from_utf8(name).expect("Couldn't convert [u8] to str"))
    {
        Ok(v) => v,
        Err(_) => return Err(nom::Err::Failure((input, nom::error::ErrorKind::MapRes))),
    };
    let (input, arg) = parse_argument(input)?;
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
// #endregion
named!(
    pub parse_line<LineType>,
    alt!(
        label_def => { |r|LineType::LabelDef(r) }|
        parse_opcode_line => { |r|LineType::Opcode(r) }
    )
);

mod tests {
    use super::ArgumentType;
    use super::Value;
    use super::{argument, hex_addr_short, label_name, parse_line, parse_opcode_line};
    use crate::addressing_modes::AddressingMode;
    use nom::IResult;
    use nom::{error::ErrorKind, Err as NErr};
    // #region Arguements
    #[test]
    fn test_hex_addr_short() {
        let (rest, ag_type): (&[u8], ArgumentType) =
            hex_addr_short(b"$23").expect("This should have been an Ok");
        assert_eq!(rest, &[][..]);
        assert_eq!(ag_type, (AddressingMode::ZPG, Value::Short(0x23)));

        let err: nom::Err<(&[u8], nom::error::ErrorKind)> = hex_addr_short(b"bd23")
            .err()
            .expect("This should have been an Err");
        assert_eq!(
            err,
            nom::Err::Error((&b"bd23"[..], nom::error::ErrorKind::Char))
        );

        let err: nom::Err<(&[u8], nom::error::ErrorKind)> = hex_addr_short(b"")
            .err()
            .expect("This should have been an Err");
        assert_eq!(err, nom::Err::Incomplete(nom::Needed::Size(1)));

        let err: nom::Err<(&[u8], nom::error::ErrorKind)> = hex_addr_short(b"$2345")
            .err()
            .expect("This should have been an Err");
        assert_eq!(
            err,
            nom::Err::Error((&[52, 53][..], nom::error::ErrorKind::Eof))
        );
    }
    #[test]
    fn test_hex_addr_long() {
        use super::hex_addr_long;
        let tests_error = [&b"$23"[..], &b"bd23"[..], &b"$2334sd"[..]];
        let errors_exp = [
            NErr::Error((&b"23"[..], ErrorKind::Eof)),
            NErr::Error((&b"bd23"[..], ErrorKind::Char)),
            NErr::Error((&b"sd"[..], ErrorKind::Eof)),
        ];
        for (test, error) in tests_error.iter().zip(errors_exp.iter()) {
            let res: IResult<&[u8], ArgumentType> = hex_addr_long(test);
            println!("{:?} -> {:?} / {:?}", test, res, error);
            assert_eq!(&res.err().unwrap(), error);
        }
        let res: IResult<&[u8], ArgumentType> = hex_addr_long(b"$2334");
        assert_eq!(
            res,
            Ok((&[][..], (AddressingMode::ABS, Value::Long(0x2334))))
        );
    }
    #[test]
    fn test_label() {
        let res = label_name(&b"hello"[..]);
        println!("{:?}", res);
    }
    #[test]
    fn test_argument() {
        let tests = [
            (&b"#$AD"[..], (AddressingMode::IMM, Value::Short(0xAD))),
            (&b"$ADDE"[..], (AddressingMode::ABS, Value::Long(0xADDE))),
            (&b"$AD"[..], (AddressingMode::ZPG, Value::Short(0xAD))),
            (
                &b"Hello"[..],
                (AddressingMode::ABS, Value::Label("Hello".to_string())),
            ),
        ];
        for test in tests.iter() {
            println!("{:X?}, {:?} ", test.0, test.1);
            println!("{:X?}", argument(test.0));
            assert_eq!(argument(test.0).ok().unwrap().1, test.1);
        }
    }
    // #endregion
    #[test]
    fn test_opcode() {
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
        let code: &str = include_str!("../assembly/custom.asm");
        for l in code.lines() {
            println!("{:X?}", parse_line(l.as_bytes()));
        }
    }
}
