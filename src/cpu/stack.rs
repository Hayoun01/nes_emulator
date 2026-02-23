use crate::{
    bus::{Byte, Word},
    cpu::CPU,
};

// * Stack Operations

impl CPU {
    pub fn stack_addr(&self) -> Word {
        0x0100 | self.sp as Word
    }

    pub fn push_byte(&mut self, byte: Byte) {
        self.write(self.stack_addr(), byte);
        self.sp = self.sp.wrapping_sub(1);
    }
    pub fn push_word(&mut self, word: Word) {
        let hi = (word >> 8) as Byte;
        self.push_byte(hi);
        let lo = word as Byte;
        self.push_byte(lo);
    }

    pub fn pull_byte(&mut self) -> Byte {
        self.sp = self.sp.wrapping_add(1);
        self.read_byte(self.stack_addr())
    }
    pub fn pull_word(&mut self) -> Word {
        let lo = self.pull_byte() as Word;
        let hi = self.pull_byte() as Word;
        (hi << 8) | lo
    }
}
