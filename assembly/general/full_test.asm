main:
    SEI
    CLD
    LDX #$40
    STX $4017
    LDX #$FF
    TXS
    INX
    INC A
    STX $2000
    STX $2001
    STX $4010
    BIT $2002
    JMP main
    JMP (data)
data:
.byte AA
.byte BB