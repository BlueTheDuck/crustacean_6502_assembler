.bank 0
.org $C000
reset:
    SEI
    CLD
    LDX #$00
    STX $2000
    STX $2001
    DEX
    TXS
    LDA #$00
    LDX #$01
    INC $FFFD
    STA $8000
    STX $8000
    STX $8000
    STX $8000
    STA $8000
    STA $E000
    STA $E000
    STA $E000
    STA $E000
    STA $E000
loop:
    LDA #%10101010
    ROL A
    JMP loop

.bank 1
.org $FFFA
.dw $0F0F
.dw reset
.dw $F0F0

.bank 2
.org $0000
.dw $AAAA
.dw $AAAA
.dw $AAAA
.dw $AAAA
.dw $AAAA
.dw $AAAA
.dw $AAAA
.dw $AAAA
.dw $AAAA
.dw $AAAA
.dw $AAAA