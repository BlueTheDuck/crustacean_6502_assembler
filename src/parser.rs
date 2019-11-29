use super::addressing_modes;

pub struct Code {
    pub name: String,
    pub arg: Option<String>,
    pub addr_mode: addressing_modes::AddressingMode,
    size: usize,
}
impl Code {
    fn new(line: &String) -> Code {
        let parts: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        let arg = parts.get(1).map_or(None, |v| Some(v.to_string()));
        let addr_mode = addressing_modes::AddressingMode::identify(&arg);
        Code {
            name: parts.get(0).unwrap().to_string(),
            arg,
            size: addressing_modes::OP_SIZES[addr_mode as usize],
            addr_mode,
        }
    }
}
enum MacroType {
    Bytes,
    Address,
}
#[derive(Debug)]
pub struct Macro {
    name: String,
    arg: Option<String>,
    size: usize,
    pub bytes: Vec<u8>,
}
impl Macro {
    pub fn new(text: &String) -> Macro {
        let mut split = text.split_whitespace();
        let name = (split.next().unwrap()[1..]).to_string();
        let arg = match split.next() {
            Some(v) => Some(v.to_string()),
            None => None,
        };
        let mut bytes = match name.as_ref() {
            "bytes" => match &arg {
                Some(v) => v
                    .to_string()
                    .split(",")
                    .map(|v| u8::from_str_radix(v, 16).unwrap())
                    .collect::<Vec<u8>>(),
                None => vec![],
            },
            _ => vec![],
        };
        Macro {
            name,
            arg,
            size: bytes.len(),
            bytes,
        }
    }
}
pub enum LineData {
    Label(String),
    Code(Code),
    Macro(Macro),
}
impl LineData {
    fn new(line: &String) -> Option<LineData> {
        if line.rfind(':') == Some(line.len() - 1) {
            let mut label_name = line.to_string();
            label_name.pop();
            return Some(LineData::Label(label_name));
        } else if line.find('.') == Some(0) {
            return Some(LineData::Macro(Macro::new(&line)));
        } else {
            return Some(LineData::Code(Code::new(&line)));
        }
    }
}

pub struct Token {
    pub text: String,
    pub line_data: LineData,
    pub addr: usize,
}
impl Token {
    pub fn new(text: String, addr: usize) -> Token {
        let line_data = LineData::new(&text).expect("line_data");
        Token {
            text,
            line_data,
            addr,
        }
    }
    pub fn get_size(&self) -> usize {
        match &self.line_data {
            LineData::Code(code) => code.size,
            LineData::Macro(r#macro) => r#macro.size,
            _ => 0,
        }
    }
}

// #region Traits
impl std::fmt::Display for Code {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "{} {}",
            self.name,
            match &self.arg {
                Some(s) => s,
                None => "",
            }
        )
    }
}
impl std::fmt::Debug for Code {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "{} {}({:?}) s{}",
            self.name,
            match &self.arg {
                Some(s) => s,
                None => "",
            },
            self.addr_mode,
            self.size
        )
    }
}

impl std::fmt::Display for LineData {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            LineData::Label(name) => write!(fmt, "{}:", name),
            LineData::Code(code) => write!(fmt, "{}", code),
            _ => unimplemented!(""),
        }
    }
}
impl std::fmt::Debug for LineData {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            LineData::Label(name) => write!(fmt, "{}", name),
            LineData::Code(code) => write!(fmt, "{:?}", code),
            _ => unimplemented!(""),
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.line_data)
    }
}
impl std::fmt::Debug for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "{:#06X}: {} // {:#?}",
            self.addr, self.text, self.line_data
        )
    }
}
// #endregion
