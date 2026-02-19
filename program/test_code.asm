* = $1000


start
inc $2a
dec $1337
lda #$0
php
lda #$CC
sta $42
lda #$33
bit $42
plp
jmp start
