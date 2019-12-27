main:
    LDA #$FF
    STA $FF
    SEI
    JMP main
    JMP end
end: