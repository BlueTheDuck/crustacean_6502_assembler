macro_rules! bytify {
    ($var: expr,$count: literal) => {{
        let mut bytes: Vec<u8> = Vec::with_capacity($count);
        let mut value = $var;
        for i in 0..$count {
            bytes.push((value & 0xFF) as u8);
            value = value / 8;
        }
        bytes
    }};
}

//A,abs,absX,absY,imm,impl,ind,indX,indY,rel,zpg,zpgX,zpgY
//1,  3,   3,   3,   2,  1,  3,   2,   2,  2,  2,   2,   2
pub static OP_SIZES: [usize; 13] = [1, 3, 3, 3, 2, 1, 3, 2, 2, 2, 2, 2, 2];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AddressingMode {
    A = 0, // LSR A
    ABS,   // LDA $1234
    ABSX,  // STA $3000,X
    ABSY,  // AND $4000,Y
    IMM,   // LDA #10
    IMPL,  // CLC
    IND,   // JMP ($FFFC)
    INDX,  // LDA ($40,X)
    INDY,  // LDA ($40),Y
    REL,   // LABEL // +4
    ZPG,   // LDA $10
    ZPGX,  // LDA $10,X
    ZPGY,  // LDA $10,Y
}
