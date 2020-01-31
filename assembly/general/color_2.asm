.org $0000
.dw $0200
.byte $00

.org $8000
loop:
    LDA $02
    STA ($00,X)
	ADC #$01
	STA $02

    LDA $00
    ADC #$01
    STA $00
	BNE loop

    LDA $02
    ADC #$01
    STA $02
    JMP loop

.org $FFFA; Vectors
.dw $0000; INT
.dw loop; Reset
.dw $0000; NMI