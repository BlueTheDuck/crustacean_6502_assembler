main:
    LDA #$02
    STA $0200
    LDA #%00101010
    CLC
    JMP main