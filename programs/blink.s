.segment "CODE"
.org $8000
    lda #$55
    sta $6002

_blink:  
    lda #$55
    sta $6000
    lda #$aa
    sta $6000
    jmp _blink

.segment "RESET"
.word $8000
