.segment "CODE"
reset:
    lda #$55
    sta $6002

    lda #$50
    sta $6000

loop:  
    ror
    sta $6000
    
    jmp loop

.segment "RESET"
.word reset
