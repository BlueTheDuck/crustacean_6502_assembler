pub fn get_size(addr_mode: AddressingMode) -> usize {
    OP_SIZES[addr_mode as usize]
}

//A,abs,absX,absY,imm,impl,ind,indX,indY,rel,zpg,zpgX,zpgY
//1,  3,   3,   3,   2,  1,  3,   2,   2,  2,  2,   2,   2
pub static OP_SIZES: [usize; 13] = [1, 3, 3, 3, 2, 1, 3, 2, 2, 2, 2, 2, 2];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AddressingMode {
    /// Ej.: `LSR A`
    A,
    /// Ej.: `LDA $1234`
    ABS,
    /// Ej.: `STA $3000,X`
    ABSX,
    /// Ej.: `AND $4000,Y`
    ABSY,
    /// Ej.: `LDA #$10`
    IMM,
    /// Ej.: `CLC`
    IMPL,
    /// Ej.: `JMP ($FFFC)`
    IND,
    /// Ej.: `LDA ($40,X)`
    INDX,
    /// Ej.: `LDA ($40),Y`
    INDY,
    /// Ej.: `LABEL // +4`
    REL,
    /// Ej.: `LDA $10`
    ZPG,
    /// Ej.: `LDA $10,X`
    ZPGX,
    /// Ej.: `LDA $10,Y`
    ZPGY,
}
