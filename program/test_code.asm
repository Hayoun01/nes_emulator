* = $1000


start
lda #$1
inc $2a
eor $2a
beq start
dec $37
php
lda #$CC
sta $42
lda #$33
bit $42
plp
jmp start
