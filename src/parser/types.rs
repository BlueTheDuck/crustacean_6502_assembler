use super::{AddressingMode, OpcodeType};

pub type NomError<'i> = nom::Err<(&'i [u8], nom::error::ErrorKind)>;

#[derive(Debug, PartialEq)]
pub enum Value {
    Short(u8),
    Long(u16),
    Label(String),
    Array(Vec<Value>),
    Text(Box<[u8]>),
    None,
}

impl Value {
    pub fn is_short(&self) -> bool {
        match self {
            Value::Short(_) => true,
            _ => false,
        }
    }
    pub fn is_long(&self) -> bool {
        match self {
            Value::Long(_) => true,
            _ => false,
        }
    }
    pub fn is_label(&self) -> bool {
        match self {
            Value::Label(_) => true,
            _ => false,
        }
    }
    pub fn is_array(&self) -> bool {
        match self {
            Value::Array(_) => true,
            _ => false,
        }
    }
    pub fn is_text(&self) -> bool {
        match self {
            Value::Text(_) => true,
            _ => false,
        }
    }
    pub fn is_none(&self) -> bool {
        match self {
            Value::None => true,
            _ => false,
        }
    }
}
impl std::convert::TryFrom<Vec<&[u8]>> for Value {
    type Error = crate::Error;
    fn try_from(v: Vec<&[u8]>) -> Result<Self, Self::Error> {
        Ok(Value::Array(
            v.into_iter()
                .map(|i: &[u8]| Ok(u8::from_str_radix(&String::from_utf8(i.to_vec())?, 16)?.into()))
                .collect::<Result<Vec<Value>, Self::Error>>()?,
        ))
    }
}
impl std::convert::From<u8> for Value {
    fn from(v: u8) -> Self {
        Self::Short(v)
    }
}

mod tests {
    #[test]
    fn test_value_try_from() {
        use std::convert::TryFrom;
        let res = super::Value::try_from(vec![&b"0F"[..], &b"FA"[..]]);
        println!("{:X?}", res);
    }
}

pub type ArgumentType = (AddressingMode, Value);
#[derive(Debug)]
pub struct Opcode {
    pub name: OpcodeType,
    pub arg: ArgumentType,
}
#[derive(Debug)]
pub enum LineType {
    Opcode(Opcode),
    LabelDef(String),
    Macro(String, Value),
}
