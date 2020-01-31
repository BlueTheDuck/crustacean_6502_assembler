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
