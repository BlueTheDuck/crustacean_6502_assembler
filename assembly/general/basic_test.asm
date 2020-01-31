.org $8000
main:
    LDA #$FF
    STA $FF
    SEI
    JMP main
    JMP end
end: