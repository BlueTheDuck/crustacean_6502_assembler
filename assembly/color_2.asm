.org $0600

    LDA #$02
    STA $01
    LDA #$00
    STA $00

    LDA #$00
	STA $02

loop:
    LDA $02
    STA ($00,X)
	ADC #$01
	STA $02

    LDA $00
    ADC #$01
    STA $00
	BNE loop
    JMP end
end:
	NOP
