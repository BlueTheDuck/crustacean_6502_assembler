use super::{bin_to_hex, eof, is_symbol, u8_to_hex};
use super::{AddressingMode, ArgumentType, Value};
use crate::nom;
use nom::{bytes::complete as bytes, character, combinator, sequence, IResult};

// #region Parsers
fn a(input: &[u8]) -> IResult<&[u8], ArgumentType> {
    let (input, _) = character::complete::char('A')(input)?;
    Ok((input, (AddressingMode::A, Value::None)))
}

fn hex_addr_short(input: &[u8]) -> IResult<&[u8], ArgumentType> {
    let (input, _) = character::complete::char('$')(input)?;
    let (input, value) = combinator::map_res(bytes::take(2usize), u8_to_hex)(input)?;
    let (input, _) = eof(input)?;
    Ok((input, (AddressingMode::ZPG, Value::Short(value as u8))))
}

fn hex_addr_long(input: &[u8]) -> IResult<&[u8], ArgumentType> {
    let (input, _) = character::complete::char('$')(input)?;
    let (input, value) = combinator::map_res(bytes::take(4usize), u8_to_hex)(input)?;
    let (input, _) = eof(input)?;
    Ok((input, (AddressingMode::ABS, Value::Long(value as u16))))
}

// absolute_indexed
named!(
    absolute_indexed<&[u8],ArgumentType>,
    do_parse!(
        char!('$')
            >> addr: map_res!(take!(4usize),u8_to_hex)
            >> char!(',')
            >> reg: alt!(
                char!('X')  => { |_| AddressingMode::ABSX } |
                char!('Y')  => { |_| AddressingMode::ABSY }
            )
            >> ( (reg, Value::Long(addr as u16)) )
    )
);

fn hex_value(input: &[u8]) -> IResult<&[u8], ArgumentType> {
    let (input, _) = character::complete::char('#')(input)?;
    let (input, _) = character::complete::char('$')(input)?;
    let (input, value) = combinator::map_res(bytes::take(2usize), u8_to_hex)(input)?;
    let (input, _) = eof(input)?;
    Ok((input, (AddressingMode::IMM, Value::Short(value as u8))))
}

fn bin_value(input: &[u8]) -> IResult<&[u8], ArgumentType> {
    let (input, _) = character::complete::char('#')(input)?;
    let (input, _) = character::complete::char('%')(input)?;
    let (input, value) = combinator::map_res(bytes::take(8usize), bin_to_hex)(input)?;
    let (input, _) = eof(input)?;
    Ok((input, (AddressingMode::IMM, Value::Short(value as u8))))
}

fn indexed_indirect(input: &[u8]) -> IResult<&[u8], ArgumentType> {
    let (input, _) = character::complete::char('(')(input)?;
    let (input, _) = character::complete::char('$')(input)?;
    let (input, value) = combinator::map_res(bytes::take(2usize), u8_to_hex)(input)?;
    let (input, _) = bytes::tag(",X")(input)?;
    let (input, _) = character::complete::char(')')(input)?;
    Ok((input, (AddressingMode::INDX, Value::Short(value as u8))))
}

// TODO: Improve label recognition
named!(
    label_name<&[u8],ArgumentType>,
    do_parse!(
        name: map_res!(take_while!(|c: u8| c.is_ascii_alphanumeric()), |s: &[u8]| {
            String::from_utf8(s.to_vec())
        })
        >> eof!()
        >> ((AddressingMode::ABS, Value::Label(name)))
    )
);

// text
named!(
    text<&[u8],ArgumentType>,
    do_parse!(
        text: delimited!(char!('\"'), take_while!(|c|(is_symbol(c)||c.is_ascii_alphanumeric())&&c!=b'"') , char!('\"'))
        >> ((AddressingMode::ABS, Value::Text(Box::from(text))))
    )
);

//#endregion
/// Parse argument
fn argument(input: &[u8]) -> IResult<&[u8], ArgumentType> {
    nom::branch::alt((
        a,
        hex_addr_short,
        hex_addr_long,
        absolute_indexed,
        hex_value,
        bin_value,
        indexed_indirect,
        label_name,
        text,
    ))(input)
}

pub fn parse_argument(input: &[u8]) -> IResult<&[u8], ArgumentType> {
    if input.is_empty() {
        return Ok((input, (AddressingMode::IMPL, Value::None)));
    }
    let (input, _) = character::complete::char(' ')(input)?;
    if input.is_empty() {
        return Ok((input, (AddressingMode::IMPL, Value::None)));
    }
    let (input, arg) = argument(input)?;
    if !input.is_empty() {
        return Err(nom::Err::Error((input, nom::error::ErrorKind::TooLarge)));
    }
    Ok((input, arg))
}

mod tests {
    // #region Arguements
    #[test]
    fn test_bin() {
        use super::bin_value;
        use crate::parser::Value;
        let tests_ok = [
            &b"#%11111111",
            &b"#%11111101",
            &b"#%01010101",
            &b"#%11100111",
            &b"#%00000000",
        ];
        let oks_exp = [
            Value::Short(0b1111_1111),
            Value::Short(0b1111_1101),
            Value::Short(0b0101_0101),
            Value::Short(0b1110_0111),
            Value::Short(0b0000_0000),
        ];
        for (&test, ok) in tests_ok.iter().zip(&oks_exp) {
            let (rest, arg_type) = bin_value(*test).expect("This should have been an Ok");
            assert_eq!(rest, &[][..]);
            assert_eq!(&arg_type.1, ok);
        }
    }

    #[test]
    fn test_hex_addr_short() {
        use super::hex_addr_short;
        use crate::addressing_modes::AddressingMode;
        use crate::parser::{ArgumentType, Value};
        use nom::{error::ErrorKind, Err as NErr};
        let tests_error = [&b"$2A43"[..], &b"bd23"[..], &b"$23sd"[..], &b"$0"[..]];
        let errors_exp = [
            NErr::Error((&b"43"[..], ErrorKind::Eof)),
            NErr::Error((&b"bd23"[..], ErrorKind::Char)),
            NErr::Error((&b"sd"[..], ErrorKind::Eof)),
            NErr::Error((&b"0"[..], ErrorKind::Eof)),
        ];
        for (test, error) in tests_error.iter().zip(errors_exp.iter()) {
            let res: NErr<(&[u8], ErrorKind)> = hex_addr_short(test)
                .err()
                .expect("This should have errored");
            println!("{:?} -> {:?} / {:?}", test, res, error);
            assert_eq!(&res, error);
        }
        let (rest, arg_type): (&[u8], ArgumentType) =
            hex_addr_short(b"$23").expect("This should have been an Ok");
        assert_eq!(rest, &[][..]);
        assert_eq!(arg_type, (AddressingMode::ZPG, Value::Short(0x23)));
    }
    #[test]
    fn test_hex_addr_long() {
        use super::hex_addr_long;
        use crate::addressing_modes::AddressingMode;
        use crate::parser::{ArgumentType, Value};
        use nom::{error::ErrorKind, Err as NErr, IResult};
        let tests_error = [&b"$23"[..], &b"bd23"[..], &b"$2334sd"[..]];
        let errors_exp = [
            NErr::Error((&b"23"[..], ErrorKind::Eof)),
            NErr::Error((&b"bd23"[..], ErrorKind::Char)),
            NErr::Error((&b"sd"[..], ErrorKind::Eof)),
        ];
        for (test, error) in tests_error.iter().zip(errors_exp.iter()) {
            let res: NErr<(&[u8], ErrorKind)> =
                hex_addr_long(test).err().expect("This should have errored");
            println!("{:?} -> {:?} / {:?}", test, res, error);
            assert_eq!(&res, error);
        }
        let res: IResult<&[u8], ArgumentType> = hex_addr_long(b"$2334");
        assert_eq!(
            res,
            Ok((&[][..], (AddressingMode::ABS, Value::Long(0x2334))))
        );
    }
    #[test]
    fn test_label() {
        use super::label_name;
        let res = label_name(&b"hello"[..]);
        println!("{:?}", res);
    }

    #[test]
    fn test_ind_x() {
        use super::indexed_indirect;
        use super::{AddressingMode, Value};

        let test = b"($02,X)";
        let res = indexed_indirect(test);
        println!("{:#?}", res);
        res.expect(":(");
        //assert_eq!(res, (AddressingMode::INDX, Value::Short(0x02)));
    }

    #[test]
    fn test_text() {
        let tests = [r#""Hello""#, r#""123.456""#, r#""hello.world""#];
        for test in &tests {
            let test = test.as_bytes();
            let text =
                super::text(test).unwrap_or_else(|e| panic!("Test failed: {:?}. {:?}", test, e));
        }
    }

    #[test]
    fn test_argument() {
        use super::super::{types::Value, AddressingMode};
        use super::argument;
        let tests = [
            &b"#$AD"[..],
            &b"$ADDE"[..],
            &b"$AD"[..],
            &b"Hello"[..],
            &b"($FE,X)"[..],
        ];
        let tests_results = [
            (AddressingMode::IMM, Value::Short(0xAD)),
            (AddressingMode::ABS, Value::Long(0xADDE)),
            (AddressingMode::ZPG, Value::Short(0xAD)),
            (AddressingMode::ABS, Value::Label("Hello".to_string())),
            (AddressingMode::INDX, Value::Short(0xFE)),
        ];
        for (test, exp) in tests.iter().zip(tests_results.iter()) {
            let (_, res) = argument(test).expect("This shouldn't haver errored");
            println!("{:X?} -> {:?} / {:?}", test, exp, res);
            assert_eq!(&res, exp);
        }
    }
    // #endregion
}
