CCTRL = $6000           ; Console control address 
COUT  = $6001           ; Console address for printing characters

.segment "CODE"
reset:
    lda #%00000001      ; Instruction to clear the output
    sta CCTRL           ; Execute instruction

    ldx #0              ; x = 0

print_message:
    lda message,x       ; a = message[x]
    beq end             ; if a == 0 then jmp end
    sta COUT            ; print a
    inx                 ; x++
    jmp print_message   ; recur

end:
    jmp reset           ; Endless loop
    

message: .asciiz "Hello world!"

.segment "RESET"
.word reset
