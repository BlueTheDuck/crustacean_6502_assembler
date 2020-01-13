.org $0600
	LDA #$00
loop:
	ADC #$01
	STA $0200
	JMP loop
