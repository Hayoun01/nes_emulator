use crate::bus::{Bus, Byte, Word};
use bitflags::{Flags, bitflags};

pub mod addressing;
pub mod instructions;
pub mod stack;

use instructions::Instruction;

bitflags! {
    pub struct Flag: Byte {
        const CARRY             = 1 << 0;
        const ZERO              = 1 << 1;
        const INTERRUPT_DISABLE = 1 << 2;
        const DECIMAL_MODE      = 1 << 3;
        const BREAK_COMMAND     = 1 << 4;
        const UNUSED            = 1 << 5;
        const OVERFLOW          = 1 << 6;
        const NEGATIVE          = 1 << 7;
    }
}

pub enum Access {
    Read,
    Write,
}

/// This a CPU struct that emulate the 6502
pub struct CPU {
    /// ### Program counter AKA **IP** (Instruction Pointer)
    pub pc: Word,
    /// ### Stack pointer
    pub sp: Byte,

    /// ### general register
    pub a: Byte,
    /// ### general register
    pub x: Byte,
    /// ### general register
    pub y: Byte,

    /// ### Flag Register
    /// flag it is a `8bit` register that each bit in it indicate the following:
    ///
    /// | bit  | Flag | 0 | 1 |
    /// |------|-------------|---|---|
    /// | 0 | Carry | `False` | `True` |
    /// | 1 | Zero | `Non Zero` | `Zero` |
    /// | 2 | Interrupt Disable | `Enable` | `Disable` |
    /// | 3 | Decimal Mode | `Enable` | `Disable` |
    /// | 4 | Break Command | `No Break` | `Break` |
    /// | 5 | UNUSED |
    /// | 6 | Overflow | `False` | `True` |
    /// | 7 | Negative | `Positive` | `Negative` |
    ///
    pub flag: Flag,
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    pub fn new() -> Self {
        Self {
            pc: 0,
            sp: 0,
            a: 0,
            x: 0,
            y: 0,
            flag: Flag::empty(),
        }
    }
    pub fn reset(&mut self) {
        self.pc = 0xFFFC;
        self.sp = 0xFD;
        self.flag.clear();
        self.a = 0;
        self.x = 0;
        self.y = 0;
    }

    pub fn set_x(&mut self, x: Byte) {
        self.x = x
    }

    pub fn execute<B: Bus>(&mut self, mut cycles: u32, bus: &mut B) -> u32 {
        let requested_cycles = cycles;
        while cycles > 0 {
            let ins = self.fetch_byte(&mut cycles, bus);
            match ins.try_into() {
                // * LDA Instructions
                Ok(Instruction::LdaIMM) => {
                    let fetched_byte = self.fetch_byte(&mut cycles, bus);
                    self.lda(fetched_byte);
                }
                Ok(Instruction::LdaZPG) => {
                    let addr = self.addr_zero_page(&mut cycles, bus);
                    let v = self.read_byte_zp(&mut cycles, addr, bus);
                    self.lda(v);
                }
                Ok(Instruction::LdaZPX) => {
                    let addr = self.addr_zero_page_x(&mut cycles, bus);
                    let v = self.read_byte_zp(&mut cycles, addr, bus);
                    self.lda(v);
                }
                Ok(Instruction::LdaABS) => {
                    let addr = self.addr_absolute(&mut cycles, bus);
                    let v = self.read_byte(&mut cycles, addr, bus);
                    self.lda(v);
                }
                Ok(Instruction::LdaABX) => {
                    let addr = self.addr_absolute_x(&mut cycles, bus, Access::Read);
                    let v = self.read_byte(&mut cycles, addr, bus);
                    self.lda(v);
                }
                Ok(Instruction::LdaABY) => {
                    let addr = self.addr_absolute_y(&mut cycles, bus, Access::Read);
                    let v = self.read_byte(&mut cycles, addr, bus);
                    self.lda(v);
                }
                Ok(Instruction::LdaIDX) => {
                    let addr = self.addr_indirect_x(&mut cycles, bus);
                    let v = self.read_byte(&mut cycles, addr, bus);
                    self.lda(v);
                }
                Ok(Instruction::LdaIDY) => {
                    let addr = self.addr_indirect_y(&mut cycles, bus, Access::Read);
                    let v = self.read_byte(&mut cycles, addr, bus);
                    self.lda(v);
                }
                // * LDX Instructions
                Ok(Instruction::LdxIMM) => {
                    let v = self.fetch_byte(&mut cycles, bus);
                    self.ldx(v);
                }
                Ok(Instruction::LdxZPG) => {
                    let addr = self.addr_zero_page(&mut cycles, bus);
                    let data = self.read_byte_zp(&mut cycles, addr, bus);
                    self.ldx(data);
                }
                Ok(Instruction::LdxZPY) => {
                    let addr = self.addr_zero_page_y(&mut cycles, bus);
                    let v = self.read_byte_zp(&mut cycles, addr, bus);
                    self.ldx(v);
                }
                Ok(Instruction::LdxABS) => {
                    let addr = self.addr_absolute(&mut cycles, bus);
                    let v = self.read_byte(&mut cycles, addr, bus);
                    self.ldx(v);
                }
                Ok(Instruction::LdxABY) => {
                    let addr = self.addr_absolute_y(&mut cycles, bus, Access::Read);
                    let v = self.read_byte(&mut cycles, addr, bus);
                    self.ldx(v);
                }
                // * LDX Instructions
                Ok(Instruction::LdyIMM) => {
                    let v = self.fetch_byte(&mut cycles, bus);
                    self.ldy(v);
                }
                Ok(Instruction::LdyZPG) => {
                    let addr = self.addr_zero_page(&mut cycles, bus);
                    let v = self.read_byte_zp(&mut cycles, addr, bus);
                    self.ldy(v);
                }
                Ok(Instruction::LdyZPX) => {
                    let addr = self.addr_zero_page_x(&mut cycles, bus);
                    let v = self.read_byte_zp(&mut cycles, addr, bus);
                    self.ldy(v);
                }
                Ok(Instruction::LdyABS) => {
                    let addr = self.addr_absolute(&mut cycles, bus);
                    let v = self.read_byte(&mut cycles, addr, bus);
                    self.ldy(v);
                }
                Ok(Instruction::LdyABX) => {
                    let addr = self.addr_absolute_x(&mut cycles, bus, Access::Read);
                    let v = self.read_byte(&mut cycles, addr, bus);
                    self.ldy(v);
                }
                // * STA Instructions
                Ok(Instruction::StaZPG) => {
                    let addr = self.addr_zero_page(&mut cycles, bus);
                    self.write_byte(&mut cycles, addr as Word, self.a, bus);
                }
                Ok(Instruction::StaZPX) => {
                    let addr = self.addr_zero_page_x(&mut cycles, bus);
                    self.write_byte(&mut cycles, addr as Word, self.a, bus);
                }
                Ok(Instruction::StaABS) => {
                    let addr = self.addr_absolute(&mut cycles, bus);
                    self.write_byte(&mut cycles, addr, self.a, bus);
                }
                Ok(Instruction::StaABX) => {
                    let addr = self.addr_absolute_x(&mut cycles, bus, Access::Write);
                    self.write_byte(&mut cycles, addr, self.a, bus);
                }
                Ok(Instruction::StaABY) => {
                    let addr = self.addr_absolute_y(&mut cycles, bus, Access::Write);
                    self.write_byte(&mut cycles, addr, self.a, bus);
                }
                Ok(Instruction::StaIDX) => {
                    let addr = self.addr_indirect_x(&mut cycles, bus);
                    self.write_byte(&mut cycles, addr, self.a, bus);
                }
                Ok(Instruction::StaIDY) => {
                    let addr = self.addr_indirect_y(&mut cycles, bus, Access::Write);
                    self.write_byte(&mut cycles, addr, self.a, bus);
                }
                // * STX Instructions
                Ok(Instruction::StxZPG) => {
                    let addr = self.addr_zero_page(&mut cycles, bus);
                    self.write_byte(&mut cycles, addr as Word, self.x, bus);
                }
                Ok(Instruction::StxZPY) => {
                    let addr = self.addr_zero_page_y(&mut cycles, bus);
                    self.write_byte(&mut cycles, addr as Word, self.x, bus);
                }
                Ok(Instruction::StxABS) => {
                    let addr = self.addr_absolute(&mut cycles, bus);
                    self.write_byte(&mut cycles, addr, self.x, bus);
                }
                // * STY Instructions
                Ok(Instruction::StyZPG) => {
                    let addr = self.addr_zero_page(&mut cycles, bus);
                    self.write_byte(&mut cycles, addr as Word, self.y, bus);
                }
                Ok(Instruction::StyZPX) => {
                    let addr = self.addr_zero_page_x(&mut cycles, bus);
                    self.write_byte(&mut cycles, addr as Word, self.y, bus);
                }
                Ok(Instruction::StyABS) => {
                    let addr = self.addr_absolute(&mut cycles, bus);
                    self.write_byte(&mut cycles, addr, self.y, bus);
                }
                // * JSR Instructions
                Ok(Instruction::JsrABS) => {
                    cycles += 1;
                    let addr = self.fetch_word(&mut cycles, bus);
                    self.push_word(&mut cycles, self.pc - 1, bus);
                    self.pc = addr;
                }
                // * RTS Instructions
                Ok(Instruction::RtsIMP) => {
                    let addr = self.pull_word(&mut cycles, bus);
                    self.pc = addr.wrapping_add(1);
                    cycles -= 1;
                }
                // * JMP Instructions
                Ok(Instruction::JmpABS) => {
                    let addr = self.addr_absolute(&mut cycles, bus);
                    self.pc = addr;
                }
                Ok(Instruction::JmpIND) => {
                    let addr = self.addr_indirect(&mut cycles, bus);
                    self.pc = addr;
                }
                // * TSX Instructions
                Ok(Instruction::TsxIMP) => {
                    self.ldx(self.sp);
                    cycles -= 1;
                }
                // * TXS Instructions
                Ok(Instruction::TxsIMP) => {
                    self.sp = self.x;
                    cycles -= 1;
                }
                // * PHA Instruction
                Ok(Instruction::PhaIMP) => {
                    self.push_byte(&mut cycles, self.a, bus);
                }
                // * PHP Instruction
                Ok(Instruction::PhpIMP) => {
                    self.push_byte(&mut cycles, self.flag.bits(), bus);
                }
                // * PLA Instruction
                Ok(Instruction::PlaIMP) => {
                    let v = self.pull_byte(&mut cycles, bus);
                    self.lda(v);
                    cycles -= 1;
                }
                // * PLP Instruction
                Ok(Instruction::PlpIMP) => {
                    let v = self.pull_byte(&mut cycles, bus);
                    self.flag.insert(Flag::from_bits(v).unwrap());
                    cycles -= 1;
                }
                // * AND Instruction
                Ok(Instruction::AndIMM) => {
                    let byte = self.fetch_byte(&mut cycles, bus);
                    self.lda(self.a & byte);
                }
                Ok(Instruction::AndZPG) => {
                    let addr = self.addr_zero_page(&mut cycles, bus);
                    let byte = self.read_byte_zp(&mut cycles, addr, bus);
                    self.lda(byte & self.a);
                }
                Ok(Instruction::AndZPX) => {
                    let addr = self.addr_zero_page_x(&mut cycles, bus);
                    let byte = self.read_byte_zp(&mut cycles, addr, bus);
                    self.lda(byte & self.a);
                }
                Ok(Instruction::AndABS) => {
                    let addr = self.addr_absolute(&mut cycles, bus);
                    let byte = self.read_byte(&mut cycles, addr, bus);
                    self.lda(byte & self.a);
                }
                Ok(Instruction::AndABX) => {
                    let addr = self.addr_absolute_x(&mut cycles, bus, Access::Read);
                    let byte = self.read_byte(&mut cycles, addr, bus);
                    self.lda(byte & self.a);
                }
                Ok(Instruction::AndABY) => {
                    let addr = self.addr_absolute_y(&mut cycles, bus, Access::Read);
                    let byte = self.read_byte(&mut cycles, addr, bus);
                    self.lda(byte & self.a);
                }
                Ok(Instruction::AndIDX) => {
                    let addr = self.addr_indirect_x(&mut cycles, bus);
                    let byte = self.read_byte(&mut cycles, addr, bus);
                    self.lda(byte & self.a);
                }
                Ok(Instruction::AndIDY) => {
                    let addr = self.addr_indirect_y(&mut cycles, bus, Access::Read);
                    let byte = self.read_byte(&mut cycles, addr, bus);
                    self.lda(byte & self.a);
                }
                // * EOR Instruction
                Ok(Instruction::EorIMM) => {
                    let byte = self.fetch_byte(&mut cycles, bus);
                    self.lda(self.a ^ byte);
                }
                Ok(Instruction::EorZPG) => {
                    let addr = self.addr_zero_page(&mut cycles, bus);
                    let byte = self.read_byte_zp(&mut cycles, addr, bus);
                    self.lda(byte ^ self.a);
                }
                Ok(Instruction::EorZPX) => {
                    let addr = self.addr_zero_page_x(&mut cycles, bus);
                    let byte = self.read_byte_zp(&mut cycles, addr, bus);
                    self.lda(byte ^ self.a);
                }
                Ok(Instruction::EorABS) => {
                    let addr = self.addr_absolute(&mut cycles, bus);
                    let byte = self.read_byte(&mut cycles, addr, bus);
                    self.lda(byte ^ self.a);
                }
                Ok(Instruction::EorABX) => {
                    let addr = self.addr_absolute_x(&mut cycles, bus, Access::Read);
                    let byte = self.read_byte(&mut cycles, addr, bus);
                    self.lda(byte ^ self.a);
                }
                Ok(Instruction::EorABY) => {
                    let addr = self.addr_absolute_y(&mut cycles, bus, Access::Read);
                    let byte = self.read_byte(&mut cycles, addr, bus);
                    self.lda(byte ^ self.a);
                }
                Ok(Instruction::EorIDX) => {
                    let addr = self.addr_indirect_x(&mut cycles, bus);
                    let byte = self.read_byte(&mut cycles, addr, bus);
                    self.lda(byte ^ self.a);
                }
                Ok(Instruction::EorIDY) => {
                    let addr = self.addr_indirect_y(&mut cycles, bus, Access::Read);
                    let byte = self.read_byte(&mut cycles, addr, bus);
                    self.lda(byte ^ self.a);
                }
                // * ORA Instruction
                Ok(Instruction::OraIMM) => {
                    let byte = self.fetch_byte(&mut cycles, bus);
                    self.lda(self.a | byte);
                }
                Ok(Instruction::OraZPG) => {
                    let addr = self.addr_zero_page(&mut cycles, bus);
                    let byte = self.read_byte_zp(&mut cycles, addr, bus);
                    self.lda(byte | self.a);
                }
                Ok(Instruction::OraZPX) => {
                    let addr = self.addr_zero_page_x(&mut cycles, bus);
                    let byte = self.read_byte_zp(&mut cycles, addr, bus);
                    self.lda(byte | self.a);
                }
                Ok(Instruction::OraABS) => {
                    let addr = self.addr_absolute(&mut cycles, bus);
                    let byte = self.read_byte(&mut cycles, addr, bus);
                    self.lda(byte | self.a);
                }
                Ok(Instruction::OraABX) => {
                    let addr = self.addr_absolute_x(&mut cycles, bus, Access::Read);
                    let byte = self.read_byte(&mut cycles, addr, bus);
                    self.lda(byte | self.a);
                }
                Ok(Instruction::OraABY) => {
                    let addr = self.addr_absolute_y(&mut cycles, bus, Access::Read);
                    let byte = self.read_byte(&mut cycles, addr, bus);
                    self.lda(byte | self.a);
                }
                Ok(Instruction::OraIDX) => {
                    let addr = self.addr_indirect_x(&mut cycles, bus);
                    let byte = self.read_byte(&mut cycles, addr, bus);
                    self.lda(byte | self.a);
                }
                Ok(Instruction::OraIDY) => {
                    let addr = self.addr_indirect_y(&mut cycles, bus, Access::Read);
                    let byte = self.read_byte(&mut cycles, addr, bus);
                    self.lda(byte | self.a);
                }
                // * BIT Instruction
                Ok(Instruction::BitZPG) => {
                    let addr = self.addr_zero_page(&mut cycles, bus);
                    let byte = self.read_byte_zp(&mut cycles, addr, bus);
                    self.flag.set(Flag::ZERO, (self.a & byte) == 0);
                    self.flag.set(Flag::OVERFLOW, (byte & 0x40) != 0);
                    self.flag.set(Flag::NEGATIVE, (byte & 0x80) != 0);
                }
                Ok(Instruction::BitABS) => {
                    let addr = self.addr_absolute(&mut cycles, bus);
                    let byte = self.read_byte(&mut cycles, addr, bus);
                    self.flag.set(Flag::ZERO, (self.a & byte) == 0);
                    self.flag.set(Flag::OVERFLOW, (byte & 0x40) != 0);
                    self.flag.set(Flag::NEGATIVE, (byte & 0x80) != 0);
                }
                Err(e) => {
                    panic!("{e}")
                }
            }
        }
        requested_cycles - cycles
    }

    fn lda_set_status(&mut self) {
        self.flag.set(Flag::ZERO, self.a == 0);
        self.flag.set(Flag::NEGATIVE, (self.a & 0x80) != 0);
    }

    fn ldx_set_status(&mut self) {
        self.flag.set(Flag::ZERO, self.x == 0);
        self.flag.set(Flag::NEGATIVE, (self.x & 0x80) != 0);
    }

    fn ldy_set_status(&mut self) {
        self.flag.set(Flag::ZERO, self.y == 0);
        self.flag.set(Flag::NEGATIVE, (self.y & 0x80) != 0);
    }

    fn lda(&mut self, v: u8) {
        self.a = v;
        self.lda_set_status();
    }

    fn ldx(&mut self, v: u8) {
        self.x = v;
        self.ldx_set_status();
    }

    fn ldy(&mut self, v: u8) {
        self.y = v;
        self.ldy_set_status();
    }

    fn fetch_byte<B: Bus>(&mut self, cycles: &mut u32, bus: &mut B) -> Byte {
        let data = bus.read(self.pc);
        *cycles -= 1;
        self.pc = self.pc.wrapping_add(1);
        data
    }

    fn read_byte<B: Bus>(&mut self, cycles: &mut u32, addr: Word, bus: &mut B) -> Byte {
        let data = bus.read(addr);
        *cycles -= 1;
        data
    }

    fn read_byte_zp<B: Bus>(&mut self, cycles: &mut u32, addr: Byte, bus: &mut B) -> Byte {
        let data = bus.read(addr as Word);
        *cycles -= 1;
        data
    }

    fn fetch_word<B: Bus>(&mut self, cycles: &mut u32, bus: &mut B) -> Word {
        let mut data = bus.read(self.pc) as Word;
        self.pc = self.pc.wrapping_add(1);
        data |= (bus.read(self.pc) as Word) << 8;
        self.pc = self.pc.wrapping_add(1);
        *cycles -= 2;
        data
    }

    fn read_word<B: Bus>(&mut self, cycles: &mut u32, addr: Word, bus: &mut B) -> Word {
        let lo = self.read_byte(cycles, addr, bus);
        let hi = self.read_byte(cycles, addr.wrapping_add(1), bus);
        lo as Word | (hi as Word) << 8
    }

    fn read_word_zp<B: Bus>(&mut self, cycles: &mut u32, addr: Byte, bus: &mut B) -> Word {
        let lo = self.read_byte_zp(cycles, addr, bus);
        let hi = self.read_byte_zp(cycles, addr.wrapping_add(1), bus);
        lo as Word | (hi as Word) << 8
    }

    fn _write_word<B: Bus>(&self, cycles: &mut u32, addr: Word, word: Word, bus: &mut B) {
        bus.write(addr, (word & 0xFF) as u8);
        bus.write(addr + 1, (word >> 8) as u8);
        *cycles -= 2;
    }

    fn write_byte<B: Bus>(&self, cycles: &mut u32, addr: Word, byte: Byte, bus: &mut B) {
        bus.write(addr, byte);
        *cycles -= 1;
    }

    fn page_crossed(base: Word, index: Byte) -> bool {
        (base & 0xFF) + index as Word >= 0x100
    }
}
