* = $1000


start
lda #$0
php
lda #$CC
sta $42
lda #$33
bit $42
plp
jmp start
