use crate::{
    bus::{Bus, Byte, Word},
    cpu::CPU,
};

// * Stack Operations

impl CPU {
    pub fn stack_addr(&self) -> Word {
        0x0100 | self.sp as Word
    }

    pub fn push_byte<B: Bus>(&mut self, cycles: &mut i32, byte: Byte, bus: &mut B) {
        bus.write(self.stack_addr(), byte);
        *cycles -= 1;
        self.sp = self.sp.wrapping_sub(1);
        *cycles -= 1;
    }
    pub fn push_word<B: Bus>(&mut self, cycles: &mut i32, word: Word, bus: &mut B) {
        let hi = (word >> 8) as Byte;
        self.push_byte(cycles, hi, bus);
        let lo = word as Byte;
        self.push_byte(cycles, lo, bus);
    }

    pub fn pull_byte<B: Bus>(&mut self, cycles: &mut i32, bus: &mut B) -> Byte {
        self.sp = self.sp.wrapping_add(1);
        *cycles -= 1;
        let data = bus.read(self.stack_addr());
        *cycles -= 1;
        data
    }
    pub fn pull_word<B: Bus>(&mut self, cycles: &mut i32, bus: &mut B) -> Word {
        let lo = self.pull_byte(cycles, bus) as Word;
        let hi = self.pull_byte(cycles, bus) as Word;
        (hi << 8) | lo
    }
}
