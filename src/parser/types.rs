use super::{AddressingMode, OpcodeType};

pub type NomError<'i> = nom::Err<(&'i [u8], nom::error::ErrorKind)>;

#[derive(Debug, PartialEq)]
pub enum Value {
    Short(u8),
    Long(u16),
    Label(String),
    Array(Vec<Value>),
    None,
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
