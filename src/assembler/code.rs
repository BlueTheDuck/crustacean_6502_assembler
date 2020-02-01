/// 0: Low 1: High
fn big_to_little_endian(value: u16) -> (u8, u8) {
    ((value & 0xFF) as u8, ((value & 0xFF00) >> 8) as u8)
}

pub struct Code {
    cart: [u8; 0x10000],
    pub pointer: usize,
}
impl Code {
    pub fn new() -> Self {
        Self {
            cart: [0x00u8; 0x10000],
            pointer: 0,
        }
    }
    /// Place a u8 on self.pointer, then increment by 1
    pub fn push_byte(&mut self, byte: u8) {
        self.cart[self.pointer] = byte;
        self.pointer += 1;
    }
    /// Take a u16, convert it to little endian then place it on self.pointer, finally increment by 2
    pub fn push_long(&mut self, long: u16) {
        let long = big_to_little_endian(long);
        self.push_byte(long.0);
        self.push_byte(long.1);
    }
    /// Add <amount> to self.pointer
    pub fn skip(&mut self, amount: usize) {
        self.pointer += amount;
    }
}
impl std::ops::Deref for Code {
    type Target = [u8; 0x10000];
    fn deref(&self) -> &[u8; 0x10000] {
        &self.cart
    }
}
