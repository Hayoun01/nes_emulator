* = $1000


start
lda #$CC
sta $42
lda #$33
bit $42
jmp start
