use super::addressing_modes;

struct Code {
    name: String,
    arg: Option<String>,
    addr_mode: addressing_modes::AddressingMode,
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
pub enum LineData {
    Label(String),
    Code(Code),
}
impl LineData {
    fn new(line: &String) -> Option<LineData> {
        if line.rfind(':') == Some(line.len() - 1) {
            let mut label_name = line.to_string();
            label_name.pop();
            return Some(LineData::Label(label_name));
        } else if line.find('.') == Some(0) {
            unimplemented!("Macros are not implemented yet");
        } else {
            return Some(LineData::Code(Code::new(&line)));
        }
    }
}

pub struct Token {
    pub text: String,
    pub line_data: LineData,
}
impl Token {
    pub fn new(text: String) -> Token {
        let line_data = LineData::new(&text).expect("line_data");
        Token { text, line_data }
    }
    pub fn get_size(&self) -> usize {
        match &self.line_data {
            LineData::Code(code) => code.size,
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
                None => "---",
            }
        )
    }
}
impl std::fmt::Debug for Code {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "{} {} // size: {}",
            self.name,
            match &self.arg {
                Some(s) => s,
                None => "---",
            },
            self.size
        )
    }
}

impl std::fmt::Display for LineData {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            LineData::Label(name) => write!(fmt, "{}:", name),
            LineData::Code(code) => write!(fmt, "{}", code),
        }
    }
}
impl std::fmt::Debug for LineData {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "{:#?}",
            match self {
                LineData::Label(name) => name,
                LineData::Code(code) => "---",
            }
        )
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.line_data)
    }
}
impl std::fmt::Debug for Token {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{} // {:#?}", self.text, self.line_data)
    }
}
// #endregion
