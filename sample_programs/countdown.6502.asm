; countdown.6502.asm
; by Will Howes
;
; counts down from 10 to zero and stores each count in memory
LDX #$10    ; x = 0x10
LDY #10     ; y = 0x0a

loop:
STY $00, X  ; write y to memory[x]
INX         ; x++
DEY         ; y--
CPY #00     ; z = 1 if y == 0 else 0
BNE loop    ; if z != 1 then branch loop
