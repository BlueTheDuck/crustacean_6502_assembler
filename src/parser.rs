use crate::nom;
use nom::IResult;
use nom::{bytes::streaming, character, combinator};
use std::str::from_utf8;

#[derive(PartialEq, Debug)]
pub enum ParameterType {
    AddrShort(u8),
    AddrLong(u16),
    Value(u8),
    Label(String),
}

fn u8_to_hex(v: &[u8]) -> Result<usize, ()> {
    let text = from_utf8(v).map_err(|_| ())?;
    usize::from_str_radix(text, 16).map_err(|_| ())
}
/* fn take_utf8<'a>(count: usize) -> (impl Fn(&'a [u8]) -> IResult<&'a [u8], &'a str>) {
    |input: &[u8]| {
        let (rest, taken) = match streaming::take(count)(input) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let string = match from_utf8(taken) {
            Ok(v) => v,
            Err(e) => return Err(nom::Err::Failure((rest, nom::error::ErrorKind::MapRes))),
        };
        Ok((rest, string))
    }
} */
named!(
    pub hex_addr_short<ParameterType>,
    do_parse!(
        char!('$') >>
        value: map_res!(take!(2),|s|u8_to_hex(s)) >>
        eof!() >>
        ( ParameterType::AddrShort(value as u8) )
    )
);
named!(
    pub hex_addr_long<ParameterType>,
    do_parse!(
        char!('$') >>
        value: map_res!(take!(4),|s|u8_to_hex(s)) >>
        eof!() >>
        ( ParameterType::AddrLong(value as u16) )
    )
);
named!(
    pub hex_value<ParameterType>,
    do_parse!(
        char!('#') >>
        char!('$') >>
        value: map_res!(take!(2),|s|u8_to_hex(s)) >>
        eof!() >>
        ( ParameterType::Value(value as u8) )
    )
);
named!(
    pub label_name<ParameterType>,
    do_parse!(
        value: map_res!(nom::combinator::rest,|v: &[u8]|String::from_utf8(v.to_vec())) >>
        eof!() >>
        ( ParameterType::Label(value) )
    )
);
named!(
    pub parameter<ParameterType>,
    do_parse!(
        ret: alt!(hex_addr_short|hex_addr_long|hex_value|label_name) >>
        ( ret )
    )
);

mod tests {
    use super::ParameterType;
    use super::{hex_addr_long, hex_addr_short, label_name, parameter};
    use nom::IResult;
    #[test]
    fn test_hex_addr_short() {
        let res: IResult<&[u8], ParameterType> = hex_addr_short(b"$23");
        assert_eq!(res, Ok((&[][..], ParameterType::AddrShort(0x23))));
        let res: IResult<&[u8], ParameterType> = hex_addr_short(b"bd23");
        assert_eq!(
            res,
            Err(nom::Err::Error((&b"bd23"[..], nom::error::ErrorKind::Char)))
        );
        let res: IResult<&[u8], ParameterType> = hex_addr_short(b"");
        assert_eq!(res, Err(nom::Err::Incomplete(nom::Needed::Size(1))));
        let res: IResult<&[u8], ParameterType> = hex_addr_short(b"$2334");
        assert_eq!(
            res,
            Err(nom::Err::Error((&b"34"[..], nom::error::ErrorKind::Eof)))
        );
    }
    #[test]
    fn test_hex_addr_long() {
        let res: IResult<&[u8], ParameterType> = hex_addr_long(b"$23");
        assert_eq!(res, Err(nom::Err::Incomplete(nom::Needed::Size(4))));
        let res: IResult<&[u8], ParameterType> = hex_addr_long(b"bd23");
        assert_eq!(
            res,
            Err(nom::Err::Error((&b"bd23"[..], nom::error::ErrorKind::Char)))
        );
        let res: IResult<&[u8], ParameterType> = hex_addr_long(b"");
        assert_eq!(res, Err(nom::Err::Incomplete(nom::Needed::Size(1))));
        let res: IResult<&[u8], ParameterType> = hex_addr_long(b"$2334");
        assert_eq!(res, Ok((&[][..], ParameterType::AddrLong(0x2334))));
        let res: IResult<&[u8], ParameterType> = hex_addr_long(b"$2334sd4");
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
    fn test_parameter() {
        let tests = [
            (&b"#$AD"[..], ParameterType::Value(0xAD)),
            (&b"$ADDE"[..], ParameterType::AddrLong(0xADDE)),
            (&b"$AD"[..], ParameterType::AddrShort(0xAD)),
            (&b"Hello"[..], ParameterType::Label("Hello".to_string())),
        ];
        for test in tests.iter() {
            assert_eq!(parameter(test.0).ok().unwrap().1, test.1);
        }
    }
}
