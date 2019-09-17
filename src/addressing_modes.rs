#[derive(Debug)]
pub enum AddressingModes {
    A,
    ABS,
    ABSX,
    ABSY,
    IMM,
    IMPL,
    IND,
    INDX,
    INDY,
    REL,
    ZPG,
    ZPGX,
    ZPGY,
}
impl AddressingModes {
    pub fn identify(text: &String) -> Self {
        if text == "A" {
            AddressingModes::A
        } else if text.find("$")==Some(0) {
            AddressingModes::ABS
        } else if text == "" {
            AddressingModes::IMPL
        } else {
            panic!("");
        }
    }
}
