use crate::nom;
use nom::IResult;
use nom::{bytes::streaming, character, combinator};
use std::str::from_utf8;

#[derive(PartialEq, Debug)]
pub enum ArgumentType {
    AddrShort(u8),
    AddrLong(u16),
    Value(u8),
    Label(String),
}
#[derive(Debug)]
pub struct Opcode<'a> {
    name: &'a str,
    arg: Option<ArgumentType>,
}
#[derive(Debug)]
pub enum LineType<'a> {
    Opcode(Opcode<'a>),
    LabelDef(String),
}

fn u8_to_hex(v: &[u8]) -> Result<usize, ()> {
    let text = from_utf8(v).map_err(|_| ())?;
    usize::from_str_radix(text, 16).map_err(|_| ())
}

// #region Arguments
named!(
    hex_addr_short<ArgumentType>,
    do_parse!(
        char!('$')
            >> value: map_res!(take!(2), |s| u8_to_hex(s))
            >> eof!()
            >> (ArgumentType::AddrShort(value as u8))
    )
);
named!(
    hex_addr_long<ArgumentType>,
    do_parse!(
        char!('$')
            >> value: map_res!(take!(4), |s| u8_to_hex(s))
            >> eof!()
            >> (ArgumentType::AddrLong(value as u16))
    )
);
named!(
    hex_value<ArgumentType>,
    do_parse!(
        char!('#')
            >> char!('$')
            >> value: map_res!(take!(2), |s| u8_to_hex(s))
            >> eof!()
            >> (ArgumentType::Value(value as u8))
    )
);
// TODO: Improve label recognition
named!(
    label_name<ArgumentType>,
    do_parse!(
        value: map_res!(character::complete::alphanumeric1, |v: &[u8]| {
            String::from_utf8(v.to_vec())
        }) >> eof!()
            >> (ArgumentType::Label(value))
    )
);
named!(
    argument<ArgumentType>,
    do_parse!(ret: alt!(hex_addr_short | hex_addr_long | hex_value | label_name) >> (ret))
);
// #endregion
// #region Types
// TODO: Improve marign recognition
named!(margin<&[u8]>, take_while!(character::is_space));
fn parse_argument(input: &[u8]) -> IResult<&[u8], Option<ArgumentType>> {
    if input.len() == 0 {
        return Ok((input, None));
    }
    let (rest, _) = character::streaming::char(' ')(input)?;
    if (rest.len() == 0) {
        return Ok((rest, None));
    }
    let (rest, arg) = argument(rest)?;
    if rest.len() != 0 {
        return Err(nom::Err::Error((input, nom::error::ErrorKind::TooLarge)));
    }
    Ok((rest, Some(arg)))
}
fn parse_opcode_line(input: &[u8]) -> IResult<&[u8], Opcode> {
    let (rest, _) = margin(input)?;
    let (rest, name) = streaming::take_while_m_n(3, 3, character::is_alphabetic)(rest)?;
    let (rest, arg) = parse_argument(rest)?;
    Ok((
        rest,
        Opcode {
            name: from_utf8(name).unwrap(),
            arg,
        },
    ))
}
named!(
    label_def<String>,
    do_parse!(
        name: map_res!(character::complete::alphanumeric1, |v: &[u8]| {
            String::from_utf8(v.to_vec())
        }) >> char!(':')
            >> eof!()
            >> (name)
    )
);
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
    use super::{
        argument, hex_addr_long, hex_addr_short, label_name, parse_line, parse_opcode_line,
    };
    use nom::IResult;
    // #region Arguements
    #[test]
    fn test_hex_addr_short() {
        let res: IResult<&[u8], ArgumentType> = hex_addr_short(b"$23");
        assert_eq!(res, Ok((&[][..], ArgumentType::AddrShort(0x23))));
        let res: IResult<&[u8], ArgumentType> = hex_addr_short(b"bd23");
        assert_eq!(
            res,
            Err(nom::Err::Error((&b"bd23"[..], nom::error::ErrorKind::Char)))
        );
        let res: IResult<&[u8], ArgumentType> = hex_addr_short(b"");
        assert_eq!(res, Err(nom::Err::Incomplete(nom::Needed::Size(1))));
        let res: IResult<&[u8], ArgumentType> = hex_addr_short(b"$2334");
        assert_eq!(
            res,
            Err(nom::Err::Error((&b"34"[..], nom::error::ErrorKind::Eof)))
        );
    }
    #[test]
    fn test_hex_addr_long() {
        let res: IResult<&[u8], ArgumentType> = hex_addr_long(b"$23");
        assert_eq!(res, Err(nom::Err::Incomplete(nom::Needed::Size(4))));
        let res: IResult<&[u8], ArgumentType> = hex_addr_long(b"bd23");
        assert_eq!(
            res,
            Err(nom::Err::Error((&b"bd23"[..], nom::error::ErrorKind::Char)))
        );
        let res: IResult<&[u8], ArgumentType> = hex_addr_long(b"");
        assert_eq!(res, Err(nom::Err::Incomplete(nom::Needed::Size(1))));
        let res: IResult<&[u8], ArgumentType> = hex_addr_long(b"$2334");
        assert_eq!(res, Ok((&[][..], ArgumentType::AddrLong(0x2334))));
        let res: IResult<&[u8], ArgumentType> = hex_addr_long(b"$2334sd4");
        assert_eq!(
            res,
            Err(nom::Err::Error((&b"sd4"[..], nom::error::ErrorKind::Eof)))
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
            (&b"#$AD"[..], ArgumentType::Value(0xAD)),
            (&b"$ADDE"[..], ArgumentType::AddrLong(0xADDE)),
            (&b"$AD"[..], ArgumentType::AddrShort(0xAD)),
            (&b"Hello"[..], ArgumentType::Label("Hello".to_string())),
        ];
        for test in tests.iter() {
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
