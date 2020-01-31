.org $8000
main:
    LDA #$FF
.org $0200
.byte #$DE
.byte #$AD
.dw $BEEF
.db $DB
.db $DE,$AD,$BE,$EF,$DE,$AD,$BE,$EF
.db $DE, $AD, $BE, $EF, $DE, $AD, $BE, $EF, $DE, $AD, $BE, $EF, $DE, $AD, $BE, $EF
.incbin "data.bin"
