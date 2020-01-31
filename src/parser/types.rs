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
            Value::Long(_) => false,
            Value::Label(_) => false,
            Value::Array(_) => false,
            Value::None => false,
        }
    }
    pub fn is_long(&self) -> bool {
        match self {
            Value::Short(_) => false,
            Value::Long(_) => true,
            Value::Label(_) => false,
            Value::Array(_) => false,
            Value::None => false,
        }
    }
    pub fn is_label(&self) -> bool {
        match self {
            Value::Short(_) => false,
            Value::Long(_) => false,
            Value::Label(_) => true,
            Value::Array(_) => false,
            Value::None => false,
        }
    }
    pub fn is_array(&self) -> bool {
        match self {
            Value::Short(_) => false,
            Value::Long(_) => false,
            Value::Label(_) => false,
            Value::Array(_) => true,
            Value::None => false,
        }
    }
    pub fn is_none(&self) -> bool {
        match self {
            Value::Short(_) => false,
            Value::Long(_) => false,
            Value::Label(_) => false,
            Value::Array(_) => false,
            Value::None => true,
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
