#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum Format {
    Hex,
}
impl<S: std::fmt::Display> std::convert::From<S> for Format {
    fn from(v: S) -> Self {
        match &*format!("{}", v) {
            "Hex" => Self::Hex,
            _ => panic!("Unkown format {}", v),
        }
    }
}
impl From<Format> for String {
    fn from(v: Format) -> String {
        String::from(match v {
            Format::Hex => "hex",
        })
    }
}
