use crate::parser::NomError;
use custom_error::custom_error;
use nom::{error::ErrorKind, Err as NErr};

custom_error! {pub Error
    Parser{cause: String} = "Parser error: {cause}",
    UnkownOpcode{name: String} = "Unkown opcode {name} (Maybe the addressing mode is not valid?)",
    UndefLabel{labels: String} = "These labels were used, but a definition couldn't be found: {labels}",
    IoError{source: std::io::Error} = "IO Error {source}"
}
impl<'i> std::convert::From<NomError<'i>> for Error {
    fn from(err: NomError<'i>) -> Error {
        let cause = match err {
            NErr::Error(n) | NErr::Failure(n) => {
                let (i, e): (&[u8], ErrorKind) = n;
                let i =
                    String::from_utf8(i.to_vec()).unwrap_or_else(|_| format!("{:?}", i.to_vec()));
                format!("Data: {}; Type: {:?}", i, e)
            }
            NErr::Incomplete(_) => "Needs more data to decide".to_string(),
        };
        Self::Parser { cause }
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_from_parser_to_main() {
        let parser_error = nom::character::complete::char('$')(&b"Hello"[..]);
        let parser_error: NomError = parser_error.err().unwrap();
        let error_enum: Error = parser_error.into();
        println!("{}", error_enum);
        assert_eq!(
            std::mem::discriminant(&error_enum),
            std::mem::discriminant(&Error::Parser {
                cause: "Data: Hello; Type: Char".to_string(),
            }),
        )
    }
}
