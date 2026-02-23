use crate::{
    bus::{Byte, Word},
    cpu::CPU,
};

// * Addressing Modes

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AddrMode {
    IMP,
    IMM,
    ZPG,
    ZPX,
    ZPY,
    ABS,
    ABX,
    ABY,
    IND,
    IDX,
    IDY,
    REL,
    XXX,
}

impl CPU {
    pub fn resolve_addr(&mut self, mode: AddrMode) -> Byte {
        match mode {
            AddrMode::IMP => self.imp(),
            AddrMode::IMM => self.imm(),
            AddrMode::ZPG => self.zpg(),
            AddrMode::ZPX => self.zpx(),
            AddrMode::ZPY => self.zpy(),
            AddrMode::ABS => self.abs(),
            AddrMode::ABX => self.abx(),
            AddrMode::ABY => self.aby(),
            AddrMode::IND => self.ind(),
            AddrMode::IDX => self.idx(),
            AddrMode::IDY => self.idy(),
            AddrMode::REL => self.rel(),
            AddrMode::XXX => self.nnn(),
        }
    }

    fn nnn(&mut self) -> Byte {
        panic!("Illegal instruction!")
    }

    /// ### Addressing Modes - Implied
    pub fn imp(&mut self) -> Byte {
        self.fetched = self.a;
        0
    }
    /// ### Addressing Modes - Immediate
    pub fn imm(&mut self) -> Byte {
        self.addr_abs = self.pc;
        self.pc = self.pc.wrapping_add(1);
        0
    }
    /// ### Addressing Modes - Zero page
    pub fn zpg(&mut self) -> Byte {
        self.addr_abs = self.fetch_byte() as Word;
        self.addr_abs &= 0x00FF;
        0
    }

    /// ### Addressing Modes - Zero page with X offset
    pub fn zpx(&mut self) -> Byte {
        let addr: Byte = self.fetch_byte().wrapping_add(self.x);
        self.addr_abs = addr as Word;
        self.addr_abs &= 0x00FF;
        0
    }
    /// ### Addressing Modes - Zero page with Y offset
    pub fn zpy(&mut self) -> Byte {
        let addr: Byte = self.fetch_byte().wrapping_add(self.y);
        self.addr_abs = addr as Word;
        self.addr_abs &= 0x00FF;
        0
    }
    /// ### Addressing Modes - Absolute
    pub fn abs(&mut self) -> Byte {
        self.addr_abs = self.fetch_word();
        0
    }
    /// ### Addressing Modes - Absolute with X offset
    pub fn abx(&mut self) -> Byte {
        self.addr_abs = self.fetch_word();
        let [_, hi] = self.addr_abs.to_le_bytes();
        self.addr_abs = self.addr_abs.wrapping_add(self.x as Word);
        if Self::page_crossed(self.addr_abs, hi) {
            1
        } else {
            0
        }
    }
    /// ### Addressing Modes - Absolute with Y offset
    pub fn aby(&mut self) -> Byte {
        self.addr_abs = self.fetch_word();
        let [_, hi] = self.addr_abs.to_le_bytes();
        self.addr_abs = self.addr_abs.wrapping_add(self.y as Word);
        if Self::page_crossed(self.addr_abs, hi) {
            1
        } else {
            0
        }
    }
    /// ### Addressing Modes - Indirect (aka Pointers)
    /// only used with **JMP** instruction
    // ! This address mode has a bug that i have to simulate as well
    pub fn ind(&mut self) -> Byte {
        let addr = self.fetch_word();
        self.addr_abs = self.read_word(addr);
        0
    }
    /// ### Addressing Modes - Indexed Indirect (X)
    pub fn idx(&mut self) -> Byte {
        let mut addr = self.fetch_byte();
        addr = addr.wrapping_add(self.x);
        self.addr_abs = self.read_word_zp(addr);
        0
    }

    /// ### Addressing Modes - Indirect Indexed (Y)
    pub fn idy(&mut self) -> Byte {
        let zp_addr = self.fetch_byte();
        let addr = self.read_word_zp(zp_addr);
        let [_, hi] = addr.to_le_bytes();
        self.addr_abs = addr.wrapping_add(self.y as Word);
        if Self::page_crossed(self.addr_abs, hi) {
            1
        } else {
            0
        }
    }
    /// ### Addressing Modes - Relative
    /// Relative addressing mode is used by branch instructions (e.g. BEQ, BNE, etc.)
    /// which contain a signed 8 bit relative offset (e.g. -128 to +127)
    /// which is added to program counter if the condition is true
    pub fn rel(&mut self) -> Byte {
        self.addr_rel = self.fetch_byte() as Word;
        if (self.addr_rel & 0x80) != 0 {
            self.addr_rel |= 0xFF00;
        }
        0
    }
}
