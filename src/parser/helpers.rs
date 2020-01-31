use nom::character;
use std::str::from_utf8;

pub fn u8_to_hex(v: &[u8]) -> Result<usize, ()> {
    let text = from_utf8(v).map_err(|_| ())?;
    usize::from_str_radix(text, 16).map_err(|_| ())
}
pub fn bin_to_hex(v: &[u8]) -> Result<usize, ()> {
    Ok(v.to_vec()
        .iter()
        .map(|bit| match bit {
            b'0' => Ok(0),
            b'1' => Ok(1),
            _ => Err(()),
        })
        .collect::<Result<Vec<usize>, ()>>()?
        .iter()
        .fold(0usize, |value: usize, bit| value * 2 + bit))
}

named!(pub eof, eof!());
// TODO: Improve margin recognition
named!(pub margin<&[u8]>, take_while!(character::is_space));

named!(pub is_text,
    do_parse!(
        text: take_while!(|c|is_symbol(c)||c.is_ascii_alphanumeric()) 
        >> (text)
    )
);

pub fn is_symbol(c: u8) -> bool {
    c.is_ascii_graphic() || c.is_ascii_punctuation()
}
