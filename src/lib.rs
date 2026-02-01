use std::ops::{Index, IndexMut};

use bitflags::{Flags, bitflags};

type Byte = u8;
type Word = u16;

enum Access {
    Read,
    Write,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    // * [LDA] Load Accumulator
    /// ### Load Accumulator Immediate
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xA9 | 2 | 2 |
    LdaIMM = 0xA9,
    /// ### Load Accumulator Zero Page
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xA5 | 2 | 3 |
    LdaZPG = 0xA5,
    /// ### Load Accumulator Zero Page X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xB5 | 2 | 4 |
    LdaZPX = 0xB5,
    /// ### Load Accumulator Absolute
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xAD | 3 | 4 |
    LdaABS = 0xAD,
    /// ### Load Accumulator Absolute X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xBD | 3 | 4 (+1 if page crossed) |
    LdaABX = 0xBD,
    /// ### Load Accumulator Absolute Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xB9 | 3 | 4 (+1 if page crossed) |
    LdaABY = 0xB9,
    /// ### Load Accumulator Indexed Indirect X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xA1 | 2 | 6 |
    LdaIDX = 0xA1,
    /// ### Load Accumulator Indirect Indexed Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xB1 | 2 | 5 (+1 if page crossed) |
    LdaIDY = 0xB1,
    // * [LDX] Load X Register
    /// ### Load X Register Immediate
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xA2 | 2 | 2 |
    LdxIMM = 0xA2,
    /// ### Load X Register Zero Page
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xA6 | 2 | 3 |
    LdxZPG = 0xA6,
    /// ### Load X Register Zero Page Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xB6 | 2 | 4 |
    LdxZPY = 0xB6,
    /// ### Load X Register Absolute
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xAE | 3 | 4 |
    LdxABS = 0xAE,
    /// ### Load X Register Absolute Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xBE | 3 | 4 (+1 if page crossed) |
    LdxABY = 0xBE,
    // * [LDY] Load Y Register
    /// ### Load Y Register Immediate
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xA0 | 2 | 2 |
    LdyIMM = 0xA0,
    /// ### Load Y Register Zero Page
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xA4 | 2 | 3 |
    LdyZPG = 0xA4,
    /// ### Load Y Register Zero Page X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xB4 | 2 | 4 |
    LdyZPX = 0xB4,
    /// ### Load Y Register Absolute
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xAC | 3 | 4 |
    LdyABS = 0xAC,
    /// ### Load Y Register Absolute X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xBC | 3 | 4 (+1 if page crossed) |
    LdyABX = 0xBC,
    // * [STA] Store Accumulator
    /// ### Store Accumulator Zero Page
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x85 | 2 | 3 |
    StaZPG = 0x85,
    /// ### Store Accumulator Zero Page X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x95 | 2 | 4 |
    StaZPX = 0x95,
    /// ### Store Accumulator Absolute
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x8D | 3 | 4 |
    StaABS = 0x8D,
    /// ### Store Accumulator Absolute X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x9D | 3 | 5 |
    StaABX = 0x9D,
    /// ### Store Accumulator Absolute Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x99 | 3 | 5 |
    StaABY = 0x99,
    /// ### Store Accumulator Indexed Indirect X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x81 | 2 | 6 |
    StaIDX = 0x81,
    /// ### Store Accumulator Indirect Indexed Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x91 | 2 | 6 |
    StaIDY = 0x91,
    // * [STX] Store X Register
    /// ### Store X Register Zero Page
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x86 | 2 | 3 |
    StxZPG = 0x86,
    /// ### Store X Register Zero Page
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x96 | 2 | 4 |
    StxZPY = 0x96,
    /// ### Store X Register Absolute
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x8E | 3 | 4 |
    StxABS = 0x8E,
    // * [STY] Store Y Register
    /// ### Store Y Register Zero Page
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x84 | 2 | 3 |
    StyZPG = 0x84,
    /// ### Store Y Register Zero Page
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x94 | 2 | 4 |
    StyZPX = 0x94,
    /// ### Store Y Register Absolute
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x8C | 3 | 4 |
    StyABS = 0x8C,
    // * [JSR] Jump to Subroutine
    /// ### Jump to Subroutine Absolute
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x20 | 3 | 6 |
    JsrABS = 0x20,
    // * [RTS] Return from Subroutine
    /// ### Return from Subroutine Implicit
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x60 | 1 | 6 |
    RtsIMP = 0x60,
}

impl TryFrom<Byte> for Instruction {
    type Error = &'static str;

    fn try_from(v: Byte) -> Result<Self, Self::Error> {
        match v {
            // * [LDA]
            x if x == Self::LdaIMM as Byte => Ok(Self::LdaIMM),
            x if x == Self::LdaZPG as Byte => Ok(Self::LdaZPG),
            x if x == Self::LdaZPX as Byte => Ok(Self::LdaZPX),
            x if x == Self::LdaABS as Byte => Ok(Self::LdaABS),
            x if x == Self::LdaABX as Byte => Ok(Self::LdaABX),
            x if x == Self::LdaABY as Byte => Ok(Self::LdaABY),
            x if x == Self::LdaIDX as Byte => Ok(Self::LdaIDX),
            x if x == Self::LdaIDY as Byte => Ok(Self::LdaIDY),
            // * [LDX]
            x if x == Self::LdxIMM as Byte => Ok(Self::LdxIMM),
            x if x == Self::LdxZPG as Byte => Ok(Self::LdxZPG),
            x if x == Self::LdxZPY as Byte => Ok(Self::LdxZPY),
            x if x == Self::LdxABS as Byte => Ok(Self::LdxABS),
            x if x == Self::LdxABY as Byte => Ok(Self::LdxABY),
            // * [LDY]
            x if x == Self::LdyIMM as Byte => Ok(Self::LdyIMM),
            x if x == Self::LdyZPG as Byte => Ok(Self::LdyZPG),
            x if x == Self::LdyZPX as Byte => Ok(Self::LdyZPX),
            x if x == Self::LdyABS as Byte => Ok(Self::LdyABS),
            x if x == Self::LdyABX as Byte => Ok(Self::LdyABX),
            // * [STA]
            x if x == Self::StaZPG as Byte => Ok(Self::StaZPG),
            x if x == Self::StaZPX as Byte => Ok(Self::StaZPX),
            x if x == Self::StaABS as Byte => Ok(Self::StaABS),
            x if x == Self::StaABX as Byte => Ok(Self::StaABX),
            x if x == Self::StaABY as Byte => Ok(Self::StaABY),
            x if x == Self::StaIDX as Byte => Ok(Self::StaIDX),
            x if x == Self::StaIDY as Byte => Ok(Self::StaIDY),
            // * [STX]
            x if x == Self::StxZPG as Byte => Ok(Self::StxZPG),
            x if x == Self::StxZPY as Byte => Ok(Self::StxZPY),
            x if x == Self::StxABS as Byte => Ok(Self::StxABS),
            // * [STY]
            x if x == Self::StyZPG as Byte => Ok(Self::StyZPG),
            x if x == Self::StyZPX as Byte => Ok(Self::StyZPX),
            x if x == Self::StyABS as Byte => Ok(Self::StyABS),
            // * [JSR]
            x if x == Self::JsrABS as Byte => Ok(Self::JsrABS),
            // * [RTS]
            x if x == Self::RtsIMP as Byte => Ok(Self::RtsIMP),
            _ => Err("unknown CPU instruction"),
        }
    }
}

impl From<Instruction> for Byte {
    fn from(val: Instruction) -> Self {
        val as Byte
    }
}

/// ### Memory size
/// it fixed on `1024 * 64` (65,536) because the 6502 CPU can only have `64KB` as Total Addressable Space
const MEMORY_SIZE: usize = 1024 * 64;

pub struct Mem {
    data: [Byte; MEMORY_SIZE],
}

impl Default for Mem {
    fn default() -> Self {
        Self::new()
    }
}

impl Mem {
    pub fn new() -> Self {
        Self {
            data: [0; MEMORY_SIZE],
        }
    }
    fn initialize(&mut self) {
        for byte in &mut self.data {
            *byte = 0;
        }
    }
}

impl Index<usize> for Mem {
    type Output = Byte;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Mem {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

bitflags! {
    struct Flag: Byte {
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

/// This a CPU struct that emulate the 6502
pub struct CPU {
    /// ### Program counter AKA **IP** (Instruction Pointer)
    pc: Word,
    /// ### Stack pointer
    sp: Byte,

    /// ### general register
    a: Byte,
    /// ### general register
    x: Byte,
    /// ### general register
    y: Byte,

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
    flag: Flag,
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
    pub fn reset(&mut self, mem: &mut Mem) {
        self.pc = 0xFFFC;
        self.sp = 0xFD;
        self.flag.clear();
        self.a = 0;
        self.x = 0;
        self.y = 0;
        mem.initialize();
    }

    pub fn set_x(&mut self, x: Byte) {
        self.x = x
    }

    pub fn execute(&mut self, mut cycles: u32, mem: &mut Mem) -> u32 {
        let requested_cycles = cycles;
        while cycles > 0 {
            let ins = self.fetch_byte(&mut cycles, mem);
            match ins.try_into() {
                // * LDA Instructions
                Ok(Instruction::LdaIMM) => {
                    let fetched_byte = self.fetch_byte(&mut cycles, mem);
                    self.lda(fetched_byte);
                }
                Ok(Instruction::LdaZPG) => {
                    let addr = self.addr_zero_page(&mut cycles, mem);
                    let v = self.read_byte_zp(&mut cycles, addr, mem);
                    self.lda(v);
                }
                Ok(Instruction::LdaZPX) => {
                    let addr = self.addr_zero_page_x(&mut cycles, mem);
                    let v = self.read_byte_zp(&mut cycles, addr, mem);
                    self.lda(v);
                }
                Ok(Instruction::LdaABS) => {
                    let addr = self.addr_absolute(&mut cycles, mem);
                    let v = self.read_byte(&mut cycles, addr, mem);
                    self.lda(v);
                }
                Ok(Instruction::LdaABX) => {
                    let addr = self.addr_absolute_x(&mut cycles, mem, Access::Read);
                    let v = self.read_byte(&mut cycles, addr, mem);
                    self.lda(v);
                }
                Ok(Instruction::LdaABY) => {
                    let addr = self.addr_absolute_y(&mut cycles, mem, Access::Read);
                    let v = self.read_byte(&mut cycles, addr, mem);
                    self.lda(v);
                }
                Ok(Instruction::LdaIDX) => {
                    let addr = self.addr_indirect_x(&mut cycles, mem);
                    let v = self.read_byte(&mut cycles, addr, mem);
                    self.lda(v);
                }
                Ok(Instruction::LdaIDY) => {
                    let addr = self.addr_indirect_y(&mut cycles, mem, Access::Read);
                    let v = self.read_byte(&mut cycles, addr, mem);
                    self.lda(v);
                }
                // * LDX Instructions
                Ok(Instruction::LdxIMM) => {
                    let v = self.fetch_byte(&mut cycles, mem);
                    self.ldx(v);
                }
                Ok(Instruction::LdxZPG) => {
                    let addr = self.addr_zero_page(&mut cycles, mem);
                    let data = self.read_byte_zp(&mut cycles, addr, mem);
                    self.ldx(data);
                }
                Ok(Instruction::LdxZPY) => {
                    let addr = self.addr_zero_page_y(&mut cycles, mem);
                    let v = self.read_byte_zp(&mut cycles, addr, mem);
                    self.ldx(v);
                }
                Ok(Instruction::LdxABS) => {
                    let addr = self.addr_absolute(&mut cycles, mem);
                    let v = self.read_byte(&mut cycles, addr, mem);
                    self.ldx(v);
                }
                Ok(Instruction::LdxABY) => {
                    let addr = self.addr_absolute_y(&mut cycles, mem, Access::Read);
                    let v = self.read_byte(&mut cycles, addr, mem);
                    self.ldx(v);
                }
                // * LDX Instructions
                Ok(Instruction::LdyIMM) => {
                    let v = self.fetch_byte(&mut cycles, mem);
                    self.ldy(v);
                }
                Ok(Instruction::LdyZPG) => {
                    let addr = self.addr_zero_page(&mut cycles, mem);
                    let v = self.read_byte_zp(&mut cycles, addr, mem);
                    self.ldy(v);
                }
                Ok(Instruction::LdyZPX) => {
                    let addr = self.addr_zero_page_x(&mut cycles, mem);
                    let v = self.read_byte_zp(&mut cycles, addr, mem);
                    self.ldy(v);
                }
                Ok(Instruction::LdyABS) => {
                    let addr = self.addr_absolute(&mut cycles, mem);
                    let v = self.read_byte(&mut cycles, addr, mem);
                    self.ldy(v);
                }
                Ok(Instruction::LdyABX) => {
                    let addr = self.addr_absolute_x(&mut cycles, mem, Access::Read);
                    let v = self.read_byte(&mut cycles, addr, mem);
                    self.ldy(v);
                }
                // * STA Instructions
                Ok(Instruction::StaZPG) => {
                    let addr = self.addr_zero_page(&mut cycles, mem);
                    self.write_byte(&mut cycles, addr as Word, self.a, mem);
                }
                Ok(Instruction::StaZPX) => {
                    let addr = self.addr_zero_page_x(&mut cycles, mem);
                    self.write_byte(&mut cycles, addr as Word, self.a, mem);
                }
                Ok(Instruction::StaABS) => {
                    let addr = self.addr_absolute(&mut cycles, mem);
                    self.write_byte(&mut cycles, addr, self.a, mem);
                }
                Ok(Instruction::StaABX) => {
                    let addr = self.addr_absolute_x(&mut cycles, mem, Access::Write);
                    self.write_byte(&mut cycles, addr, self.a, mem);
                }
                Ok(Instruction::StaABY) => {
                    let addr = self.addr_absolute_y(&mut cycles, mem, Access::Write);
                    self.write_byte(&mut cycles, addr, self.a, mem);
                }
                Ok(Instruction::StaIDX) => {
                    let addr = self.addr_indirect_x(&mut cycles, mem);
                    self.write_byte(&mut cycles, addr, self.a, mem);
                }
                Ok(Instruction::StaIDY) => {
                    let addr = self.addr_indirect_y(&mut cycles, mem, Access::Write);
                    self.write_byte(&mut cycles, addr, self.a, mem);
                }
                // * STX Instructions
                Ok(Instruction::StxZPG) => {
                    let addr = self.addr_zero_page(&mut cycles, mem);
                    self.write_byte(&mut cycles, addr as Word, self.x, mem);
                }
                Ok(Instruction::StxZPY) => {
                    let addr = self.addr_zero_page_y(&mut cycles, mem);
                    self.write_byte(&mut cycles, addr as Word, self.x, mem);
                }
                Ok(Instruction::StxABS) => {
                    let addr = self.addr_absolute(&mut cycles, mem);
                    self.write_byte(&mut cycles, addr, self.x, mem);
                }
                // * STY Instructions
                Ok(Instruction::StyZPG) => {
                    let addr = self.addr_zero_page(&mut cycles, mem);
                    self.write_byte(&mut cycles, addr as Word, self.y, mem);
                }
                Ok(Instruction::StyZPX) => {
                    let addr = self.addr_zero_page_x(&mut cycles, mem);
                    self.write_byte(&mut cycles, addr as Word, self.y, mem);
                }
                Ok(Instruction::StyABS) => {
                    let addr = self.addr_absolute(&mut cycles, mem);
                    self.write_byte(&mut cycles, addr, self.y, mem);
                }
                // * JSR Instructions
                Ok(Instruction::JsrABS) => {
                    let addr = self.fetch_word(&mut cycles, mem);
                    self.push_word(&mut cycles, self.pc - 1, mem);
                    cycles -= 1;
                    self.pc = addr;
                }
                // * RTS Instructions
                Ok(Instruction::RtsIMP) => {
                    let addr = self.pull_word(&mut cycles, mem);
                    self.pc = addr.wrapping_add(1);
                    cycles -= 3;
                }
                Err(e) => {
                    panic!("{e}")
                }
            }
        }
        requested_cycles - cycles
    }

    fn lda_set_status(&mut self) {
        if self.a == 0 {
            self.flag.insert(Flag::ZERO);
        }
        if (self.a & 0b10000000) > 0 {
            self.flag.insert(Flag::NEGATIVE);
        }
    }

    fn ldx_set_status(&mut self) {
        if self.x == 0 {
            self.flag.insert(Flag::ZERO);
        }
        if (self.x & 0b10000000) > 0 {
            self.flag.insert(Flag::NEGATIVE);
        }
    }

    fn ldy_set_status(&mut self) {
        if self.y == 0 {
            self.flag.insert(Flag::ZERO);
        }
        if (self.y & 0b10000000) > 0 {
            self.flag.insert(Flag::NEGATIVE);
        }
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

    fn fetch_byte(&mut self, cycles: &mut u32, mem: &Mem) -> Byte {
        let data = mem[self.pc as usize];
        *cycles -= 1;
        self.pc = self.pc.wrapping_add(1);
        data
    }

    fn read_byte(&mut self, cycles: &mut u32, addr: Word, mem: &Mem) -> Byte {
        let data = mem[addr as usize];
        *cycles -= 1;
        data
    }

    fn read_byte_zp(&mut self, cycles: &mut u32, addr: Byte, mem: &Mem) -> Byte {
        let data = mem[addr as usize];
        *cycles -= 1;
        data
    }

    fn fetch_word(&mut self, cycles: &mut u32, mem: &Mem) -> Word {
        let mut data = mem[self.pc as usize] as Word;
        self.pc = self.pc.wrapping_add(1);
        data |= (mem[self.pc as usize] as Word) << 8;
        self.pc = self.pc.wrapping_add(1);
        *cycles -= 2;
        data
    }

    fn _read_word(&mut self, cycles: &mut u32, addr: Word, mem: &Mem) -> Word {
        let lo = self.read_byte(cycles, addr, mem);
        let hi = self.read_byte(cycles, addr.wrapping_add(1), mem);
        lo as Word | (hi as Word) << 8
    }

    fn read_word_zp(&mut self, cycles: &mut u32, addr: Byte, mem: &Mem) -> Word {
        let lo = self.read_byte_zp(cycles, addr, mem);
        let hi = self.read_byte_zp(cycles, addr.wrapping_add(1), mem);
        lo as Word | (hi as Word) << 8
    }

    fn write_word(&self, cycles: &mut u32, addr: Word, word: Word, mem: &mut Mem) {
        mem[addr as usize] = (word & 0xFF) as u8;
        mem[(addr + 1) as usize] = (word >> 8) as u8;
        *cycles -= 2;
    }

    fn write_byte(&self, cycles: &mut u32, addr: Word, byte: Byte, mem: &mut Mem) {
        mem[addr as usize] = byte;
        *cycles -= 1;
    }

    fn page_crossed(base: Word, index: Byte) -> bool {
        (base & 0xFF) + index as Word >= 0x100
    }

    // * Stack Operations
    fn stack_addr(&self) -> Word {
        0x0100 | self.sp as Word
    }

    fn push_byte(&mut self, cycles: &mut u32, byte: Byte, mem: &mut Mem) {
        mem[self.stack_addr() as usize] = byte;
        self.sp = self.sp.wrapping_sub(1);
        *cycles -= 1;
    }
    fn push_word(&mut self, cycles: &mut u32, word: Word, mem: &mut Mem) {
        let hi = (word >> 8) as Byte;
        self.push_byte(cycles, hi, mem);
        let lo = word as Byte;
        self.push_byte(cycles, lo, mem);
    }

    fn pull_byte(&mut self, cycles: &mut u32, mem: &mut Mem) -> Byte {
        self.sp = self.sp.wrapping_add(1);
        *cycles -= 1;
        mem[self.stack_addr() as usize]
    }
    fn pull_word(&mut self, cycles: &mut u32, mem: &mut Mem) -> Word {
        let lo = self.pull_byte(cycles, mem) as Word;
        let hi = self.pull_byte(cycles, mem) as Word;
        (hi << 8) | lo
    }

    // * Addressing Modes
    /// ### Addressing Modes - Zero page
    fn addr_zero_page(&mut self, cycles: &mut u32, mem: &Mem) -> Byte {
        self.fetch_byte(cycles, mem)
    }
    /// ### Addressing Modes - Zero page with X offset
    fn addr_zero_page_x(&mut self, cycles: &mut u32, mem: &Mem) -> Byte {
        let addr = self.fetch_byte(cycles, mem).wrapping_add(self.x);
        *cycles -= 1;
        addr
    }
    /// ### Addressing Modes - Zero page with Y offset
    fn addr_zero_page_y(&mut self, cycles: &mut u32, mem: &Mem) -> Byte {
        let addr = self.fetch_byte(cycles, mem).wrapping_add(self.y);
        *cycles -= 1;
        addr
    }
    /// ### Addressing Modes - Absolute
    fn addr_absolute(&mut self, cycles: &mut u32, mem: &Mem) -> Word {
        self.fetch_word(cycles, mem)
    }
    /// ### Addressing Modes - Absolute with X offset
    fn addr_absolute_x(&mut self, cycles: &mut u32, mem: &Mem, access: Access) -> Word {
        let mut addr = self.fetch_word(cycles, mem);
        if matches!(access, Access::Write) || Self::page_crossed(addr, self.x) {
            *cycles -= 1;
        }
        addr = addr.wrapping_add(self.x as Word);
        addr
    }
    /// ### Addressing Modes - Absolute with Y offset
    fn addr_absolute_y(&mut self, cycles: &mut u32, mem: &Mem, access: Access) -> Word {
        let mut addr = self.fetch_word(cycles, mem);
        if matches!(access, Access::Write) || Self::page_crossed(addr, self.y) {
            *cycles -= 1;
        }
        addr = addr.wrapping_add(self.y as Word);
        addr
    }
    /// ### Addressing Modes - Indexed Indirect (X)
    fn addr_indirect_x(&mut self, cycles: &mut u32, mem: &Mem) -> Word {
        let mut addr = self.fetch_byte(cycles, mem);
        addr = addr.wrapping_add(self.x);
        *cycles -= 1;

        self.read_word_zp(cycles, addr, mem)
    }
    /// ### Addressing Modes - Indirect Indexed (Y)
    fn addr_indirect_y(&mut self, cycles: &mut u32, mem: &Mem, access: Access) -> Word {
        let addr = self.fetch_byte(cycles, mem);
        let mut addr = self.read_word_zp(cycles, addr, mem);
        if matches!(access, Access::Write) || Self::page_crossed(addr, self.y) {
            *cycles -= 1;
        }
        addr = addr.wrapping_add(self.y as Word);
        addr
    }
}

#[cfg(test)]
pub mod test {

    use crate::*;

    fn setup_cpu_mem() -> (CPU, Mem) {
        let mut mem = Mem::new();
        let mut cpu = CPU::new();
        cpu.reset(&mut mem);
        (cpu, mem)
    }

    #[test]
    fn cpu_initialized_properly() {
        let (cpu, mem) = setup_cpu_mem();
        assert_eq!(cpu.pc, 0xFFFC);
        assert_eq!(cpu.sp, 0xFD);
        assert!(cpu.flag.is_empty());
        assert_eq!(mem.data, [0x0; MEMORY_SIZE]);
    }

    #[test]
    #[should_panic(expected = "unknown CPU instruction")]
    fn invalid_cpu_instruction() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = 0x0;
        cpu.execute(1, &mut mem);
    }

    #[test]
    fn lda_immediate_load_zero_value_to_register_a() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdaIMM.into();
        mem[0xFFFD] = 0x0;
        let cycle_used = cpu.execute(2, &mut mem);
        assert_eq!(cycle_used, 2);
        assert_eq!(cpu.a, 0x0);
        assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
    }

    #[test]
    fn lda_immediate_load_negative_value_to_register_a() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdaIMM.into();
        mem[0xFFFD] = 0x84;
        let cycle_used = cpu.execute(2, &mut mem);
        assert_eq!(cycle_used, 2);
        assert_eq!(cpu.a, 0x84);
        assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
    }

    #[test]
    fn lda_immediate_load_value_to_register_a() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdaIMM.into();
        mem[0xFFFD] = 0x2A;
        let cycle_used = cpu.execute(2, &mut mem);
        assert_eq!(cycle_used, 2);
        assert_eq!(cpu.a, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn lda_zero_page_load_value_to_register_a() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdaZPG.into();
        mem[0xFFFD] = 0x42;
        mem[0x0042] = 0x2A;
        let cycle_used = cpu.execute(3, &mut mem);
        assert_eq!(cycle_used, 3);
        assert_eq!(cpu.a, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn lda_zero_page_x_load_value_to_register_a() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdaZPX.into();
        mem[0xFFFD] = 0x40;
        mem[0x0042] = 0x2A;
        cpu.x = 0x2;
        let cycle_used = cpu.execute(4, &mut mem);
        assert_eq!(cycle_used, 4);
        assert_eq!(cpu.a, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn lda_zero_page_x_must_wrap_load_value_to_register_a() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdaZPX.into();
        mem[0xFFFD] = 0x43;
        mem[0x0042] = 0x2A;
        cpu.x = 0xFF;
        let cycle_used = cpu.execute(4, &mut mem);
        assert_eq!(cycle_used, 4);
        assert_eq!(cpu.a, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn lda_abs_load_value_to_register_a() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdaABS.into();
        mem[0xFFFD] = 0x42;
        mem[0xFFFE] = 0x41; // Ox4142
        mem[0x4142] = 0x2A;
        let cycle_used = cpu.execute(4, &mut mem);
        assert_eq!(cycle_used, 4);
        assert_eq!(cpu.a, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn lda_abx_load_value_to_register_a() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdaABX.into();
        mem[0xFFFD] = 0x41;
        mem[0xFFFE] = 0x42; // 0x4241
        mem[0x4242] = 0x2A;
        cpu.x = 0x01;
        let cycle_used = cpu.execute(4, &mut mem);
        assert_eq!(cycle_used, 4);
        assert_eq!(cpu.a, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn lda_abx_cross_page_load_value_to_register_a() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdaABX.into();
        mem[0xFFFD] = 0xF0;
        mem[0xFFFE] = 0x02; // 0x0300
        mem[0x0300] = 0x2A;
        cpu.x = 0x10;
        let cycle_used = cpu.execute(5, &mut mem);
        assert_eq!(cycle_used, 5);
        assert_eq!(cpu.a, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn lda_aby_load_value_to_register_a() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdaABY.into();
        mem[0xFFFD] = 0x41;
        mem[0xFFFE] = 0x42; // 0x4241
        mem[0x4242] = 0x2A;
        cpu.y = 0x01;
        let cycle_used = cpu.execute(4, &mut mem);
        assert_eq!(cycle_used, 4);
        assert_eq!(cpu.a, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn lda_aby_cross_page_load_value_to_register_a() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdaABY.into();
        mem[0xFFFD] = 0xF0;
        mem[0xFFFE] = 0x02; // 0x0300
        mem[0x0300] = 0x2A;
        cpu.y = 0x10;
        let cycle_used = cpu.execute(5, &mut mem);
        assert_eq!(cycle_used, 5);
        assert_eq!(cpu.a, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn lda_idx_load_value_to_register_a() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdaIDX.into();
        mem[0xFFFD] = 0x20;
        mem[0x0024] = 0x00;
        mem[0x0025] = 0x80;
        mem[0x8000] = 0x2A;
        cpu.x = 0x4;
        let cycle_used = cpu.execute(6, &mut mem);
        assert_eq!(cycle_used, 6);
        assert_eq!(cpu.a, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn lda_idy_load_value_to_register_a() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdaIDY.into();
        mem[0xFFFD] = 0x20;
        mem[0x0020] = 0x00;
        mem[0x0021] = 0x80;
        mem[0x8004] = 0x2A;
        cpu.y = 0x4;
        let cycle_used = cpu.execute(5, &mut mem);
        assert_eq!(cycle_used, 5);
        assert_eq!(cpu.a, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn lda_idy_cross_page_load_value_to_register_a() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdaIDY.into();
        mem[0xFFFD] = 0x20;
        mem[0x0020] = 0x10;
        mem[0x0021] = 0x80;
        mem[0x8100] = 0x2A;
        cpu.y = 0xF0;
        let cycle_used = cpu.execute(6, &mut mem);
        assert_eq!(cycle_used, 6);
        assert_eq!(cpu.a, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn ldx_immediate_load_zero_value_to_register_x() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdxIMM.into();
        mem[0xFFFD] = 0x0;
        let cycle_used = cpu.execute(2, &mut mem);
        assert_eq!(cycle_used, 2);
        assert_eq!(cpu.x, 0x0);
        assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
    }

    #[test]
    fn ldx_immediate_load_negative_value_to_register_x() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdxIMM.into();
        mem[0xFFFD] = 0x84;
        let cycle_used = cpu.execute(2, &mut mem);
        assert_eq!(cycle_used, 2);
        assert_eq!(cpu.x, 0x84);
        assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
    }

    #[test]
    fn ldx_immediate_load_value_to_register_x() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdxIMM.into();
        mem[0xFFFD] = 0x2A;
        let cycle_used = cpu.execute(2, &mut mem);
        assert_eq!(cycle_used, 2);
        assert_eq!(cpu.x, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn ldx_zero_page_load_value_to_register_x() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdxZPG.into();
        mem[0xFFFD] = 0x42;
        mem[0x0042] = 0x2A;
        let cycle_used = cpu.execute(3, &mut mem);
        assert_eq!(cycle_used, 3);
        assert_eq!(cpu.x, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn ldx_zero_page_y_load_value_to_register_x() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdxZPY.into();
        mem[0xFFFD] = 0x40;
        mem[0x0042] = 0x2A;
        cpu.y = 0x2;
        let cycle_used = cpu.execute(4, &mut mem);
        assert_eq!(cycle_used, 4);
        assert_eq!(cpu.x, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn ldx_abs_load_value_to_register_x() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdxABS.into();
        mem[0xFFFD] = 0x42; // lo
        mem[0xFFFE] = 0x41; // Ox4142
        mem[0x4142] = 0x2A;
        let cycle_used = cpu.execute(4, &mut mem);
        assert_eq!(cycle_used, 4);
        assert_eq!(cpu.x, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn ldx_aby_load_value_to_register_x() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdxABY.into();
        mem[0xFFFD] = 0x41;
        mem[0xFFFE] = 0x42; // 0x4241
        mem[0x4242] = 0x2A;
        cpu.y = 0x01;
        let cycle_used = cpu.execute(4, &mut mem);
        assert_eq!(cycle_used, 4);
        assert_eq!(cpu.x, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn ldx_aby_cross_page_load_value_to_register_x() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdxABY.into();
        mem[0xFFFD] = 0xF0;
        mem[0xFFFE] = 0x02; // 0x0300
        mem[0x0300] = 0x2A;
        cpu.y = 0x10;
        let cycle_used = cpu.execute(5, &mut mem);
        assert_eq!(cycle_used, 5);
        assert_eq!(cpu.x, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn ldy_immediate_load_zero_value_to_register_y() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdyIMM.into();
        mem[0xFFFD] = 0x0;
        let cycle_used = cpu.execute(2, &mut mem);
        assert_eq!(cycle_used, 2);
        assert_eq!(cpu.y, 0x0);
        assert_eq!(cpu.flag.bits(), Flag::ZERO.bits());
    }

    #[test]
    fn ldy_immediate_load_negative_value_to_register_y() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdyIMM.into();
        mem[0xFFFD] = 0x84;
        let cycle_used = cpu.execute(2, &mut mem);
        assert_eq!(cycle_used, 2);
        assert_eq!(cpu.y, 0x84);
        assert_eq!(cpu.flag.bits(), Flag::NEGATIVE.bits());
    }

    #[test]
    fn ldy_immediate_load_value_to_register_y() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdyIMM.into();
        mem[0xFFFD] = 0x2A;
        let cycle_used = cpu.execute(2, &mut mem);
        assert_eq!(cycle_used, 2);
        assert_eq!(cpu.y, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn ldy_zero_page_load_value_to_register_y() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdyZPG.into();
        mem[0xFFFD] = 0x42;
        mem[0x0042] = 0x2A;
        let cycle_used = cpu.execute(3, &mut mem);
        assert_eq!(cycle_used, 3);
        assert_eq!(cpu.y, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn ldy_zero_page_x_load_value_to_register_y() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdyZPX.into();
        mem[0xFFFD] = 0x40;
        mem[0x0042] = 0x2A;
        cpu.x = 0x2;
        let cycle_used = cpu.execute(4, &mut mem);
        assert_eq!(cycle_used, 4);
        assert_eq!(cpu.y, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn ldy_abs_load_value_to_register_y() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdyABS.into();
        mem[0xFFFD] = 0x42; // lo
        mem[0xFFFE] = 0x41; // Ox4142
        mem[0x4142] = 0x2A;
        let cycle_used = cpu.execute(4, &mut mem);
        assert_eq!(cycle_used, 4);
        assert_eq!(cpu.y, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn ldy_abx_load_value_to_register_y() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdyABX.into();
        mem[0xFFFD] = 0x41;
        mem[0xFFFE] = 0x42; // 0x4241
        mem[0x4242] = 0x2A;
        cpu.x = 0x01;
        let cycle_used = cpu.execute(4, &mut mem);
        assert_eq!(cycle_used, 4);
        assert_eq!(cpu.y, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn ldy_abx_cross_page_load_value_to_register_y() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::LdyABX.into();
        mem[0xFFFD] = 0xF0;
        mem[0xFFFE] = 0xFF; // 0xFFF0
        mem[0x0000] = 0x2A;
        cpu.x = 0x10;
        let cycle_used = cpu.execute(5, &mut mem);
        assert_eq!(cycle_used, 5);
        assert_eq!(cpu.y, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    // * STA TESTS
    #[test]
    fn sta_zero_page_can_store_the_a_register_into_memory() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0x0FFFC] = Instruction::StaZPG.into();
        mem[0x0FFFD] = 0x42;
        cpu.a = 0x2A;
        let cycle_used = cpu.execute(3, &mut mem);
        assert_eq!(cycle_used, 3);
        assert_eq!(mem[0x42], 42);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn sta_zero_page_x_can_store_the_a_register_into_memory() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0x0FFFC] = Instruction::StaZPX.into();
        mem[0x0FFFD] = 0x40;
        cpu.x = 0x02;
        cpu.a = 0x2A;
        let cycle_used = cpu.execute(4, &mut mem);
        assert_eq!(cycle_used, 4);
        assert_eq!(mem[0x42], 42);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn sta_absolute_can_store_the_a_register_into_memory() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0x0FFFC] = Instruction::StaABS.into();
        mem[0x0FFFD] = 0x42;
        mem[0x0FFFE] = 0x37; // 0x3742
        cpu.a = 0x2A;
        let cycle_used = cpu.execute(4, &mut mem);
        assert_eq!(cycle_used, 4);
        assert_eq!(mem[0x3742], 42);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn sta_absolute_x_can_store_the_a_register_into_memory() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0x0FFFC] = Instruction::StaABX.into();
        mem[0x0FFFD] = 0x37;
        mem[0x0FFFE] = 0x42; // 0x4237
        cpu.x = 0xB;
        cpu.a = 0x2A;
        let cycle_used = cpu.execute(5, &mut mem);
        assert_eq!(cycle_used, 5);
        assert_eq!(mem[0x4242], 42);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn sta_absolute_y_can_store_the_a_register_into_memory() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0x0FFFC] = Instruction::StaABY.into();
        mem[0x0FFFD] = 0x37;
        mem[0x0FFFE] = 0x42; // 0x4237
        cpu.y = 0xB;
        cpu.a = 0x2A;
        let cycle_used = cpu.execute(5, &mut mem);
        assert_eq!(cycle_used, 5);
        assert_eq!(mem[0x4242], 42);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn sta_indexed_indirect_x_can_store_the_a_register_into_memory() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0x0FFFC] = Instruction::StaIDX.into();
        mem[0x0FFFD] = 0x37;
        mem[0x0042] = 0x42;
        mem[0x0043] = 0x42;
        cpu.x = 0xB;
        cpu.a = 0x2A;
        let cycle_used = cpu.execute(6, &mut mem);
        assert_eq!(cycle_used, 6);
        assert_eq!(mem[0x4242], 42);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn sta_indirect_indexed_y_can_store_the_a_register_into_memory() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0x0FFFC] = Instruction::StaIDY.into();
        mem[0x0FFFD] = 0x37;
        mem[0x0037] = 0x37;
        mem[0x0038] = 0x42;
        cpu.y = 0xB;
        cpu.a = 0x2A;
        let cycle_used = cpu.execute(6, &mut mem);
        assert_eq!(cycle_used, 6);
        assert_eq!(mem[0x4242], 42);
        assert!(cpu.flag.is_empty());
    }
    // * STX TESTS
    #[test]
    fn stx_zero_page_can_store_the_a_register_into_memory() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0x0FFFC] = Instruction::StxZPG.into();
        mem[0x0FFFD] = 0x42;
        cpu.x = 0x2A;
        let cycle_used = cpu.execute(3, &mut mem);
        assert_eq!(cycle_used, 3);
        assert_eq!(mem[0x42], 42);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn stx_zero_page_y_can_store_the_a_register_into_memory() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0x0FFFC] = Instruction::StxZPY.into();
        mem[0x0FFFD] = 0x40;
        cpu.y = 0x02;
        cpu.x = 0x2A;
        let cycle_used = cpu.execute(4, &mut mem);
        assert_eq!(cycle_used, 4);
        assert_eq!(mem[0x42], 42);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn stx_absolute_can_store_the_a_register_into_memory() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0x0FFFC] = Instruction::StxABS.into();
        mem[0x0FFFD] = 0x42;
        mem[0x0FFFE] = 0x37; // 0x3742
        cpu.x = 0x2A;
        let cycle_used = cpu.execute(4, &mut mem);
        assert_eq!(cycle_used, 4);
        assert_eq!(mem[0x3742], 42);
        assert!(cpu.flag.is_empty());
    }
    // * STY TESTS
    #[test]
    fn sty_zero_page_can_store_the_a_register_into_memory() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0x0FFFC] = Instruction::StyZPG.into();
        mem[0x0FFFD] = 0x42;
        cpu.y = 0x2A;
        let cycle_used = cpu.execute(3, &mut mem);
        assert_eq!(cycle_used, 3);
        assert_eq!(mem[0x42], 42);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn sty_zero_page_x_can_store_the_a_register_into_memory() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0x0FFFC] = Instruction::StyZPX.into();
        mem[0x0FFFD] = 0x40;
        cpu.x = 0x02;
        cpu.y = 0x2A;
        let cycle_used = cpu.execute(4, &mut mem);
        assert_eq!(cycle_used, 4);
        assert_eq!(mem[0x42], 42);
        assert!(cpu.flag.is_empty());
    }

    #[test]
    fn sty_absolute_can_store_the_a_register_into_memory() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0x0FFFC] = Instruction::StyABS.into();
        mem[0x0FFFD] = 0x42;
        mem[0x0FFFE] = 0x37; // 0x3742
        cpu.y = 0x2A;
        let cycle_used = cpu.execute(4, &mut mem);
        assert_eq!(cycle_used, 4);
        assert_eq!(mem[0x3742], 42);
        assert!(cpu.flag.is_empty());
    }

    // * JSR TESTS
    #[test]
    fn jsr_absolute_load_value_to_register_a() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::JsrABS.into();
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x80;
        mem[0x8080] = Instruction::LdaIMM.into();
        mem[0x8081] = 0x2A;
        // 6 cycle to execute JSR instruction
        let cycle_used = cpu.execute(6, &mut mem);
        assert_eq!(cycle_used, 6);
        // 2 cycle to execute LdaIMM instruction
        let cycle_used = cpu.execute(2, &mut mem);
        assert_eq!(cycle_used, 2);
        assert_eq!(cpu.a, 0x2A);
        assert!(cpu.flag.is_empty());
    }
    
    // * RTS TESTS
    #[test]
    fn jsr_rts_load_value_to_register_a() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        mem[0xFFFC] = Instruction::JsrABS.into();
        mem[0xFFFD] = 0x80;
        mem[0xFFFE] = 0x80;
        mem[0xFFFF] = Instruction::LdaIMM.into();
        mem[0x0000] = 0x2A;
        mem[0x8080] = Instruction::RtsIMP.into(); // return
        // 6 cycle to execute JSR instruction
        let cycle_used = cpu.execute(6, &mut mem);
        assert_eq!(cycle_used, 6);
        // another 6 cycle to execute RTS instruction
        let cycle_used = cpu.execute(6, &mut mem);
        assert_eq!(cycle_used, 6);
        // check if the RTS returned successfully
        assert_eq!(cpu.pc, 0xFFFF);
        // 2 cycle to execute LdaIMM instruction
        let cycle_used = cpu.execute(2, &mut mem);
        assert_eq!(cycle_used, 2);
        assert_eq!(cpu.a, 0x2A);
        assert!(cpu.flag.is_empty());
    }

    // * Stack Operations TESTS
    #[test]
    fn stack_can_push_and_pull_byte() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        let mut cycles = 2; // push and pull must cost 2 cycles
        cpu.push_byte(&mut cycles, 0x2A, &mut mem);
        assert_eq!(mem[0x01FD], 0x2A); // check the data if stored in the correct place
        assert_eq!(cpu.sp, 0xFC); // check if the SP decremented by 1 after storing the data
        let data = cpu.pull_byte(&mut cycles, &mut mem);
        assert_eq!(data, 0x2A);
        assert_eq!(cpu.sp, 0xFD); // the SP must return to its initial position
    }

    #[test]
    fn stack_can_push_and_pull_array_of_bytes() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        let mut cycles = 20; // push and pull must cost 20 cycles
        for i in 1..=10 {
            cpu.push_byte(&mut cycles, i, &mut mem);
            let addr = 0x0100 | (cpu.sp + 1) as usize;
            assert_eq!(mem[addr], i); // check the data if stored in the correct place
        }
        assert_eq!(cpu.sp, 0xFD - 10); // check if the SP decremented by 10 after storing the data
        for i in (1..=10).rev() {
            let data = cpu.pull_byte(&mut cycles, &mut mem);
            assert_eq!(data, i);
        }
        assert_eq!(cpu.sp, 0xFD); // the SP must return to its initial position
    }

    #[test]
    fn stack_can_push_and_pull_word() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        let mut cycles = 4; // push and pull must cost 4 cycles
        cpu.push_word(&mut cycles, 0x2A37, &mut mem);
        assert_eq!(mem[0x01FD], 0x2A); // hi first
        assert_eq!(mem[0x01FC], 0x37); // lo
        assert_eq!(cpu.sp, 0xFB); // check if the SP decremented by 2 after storing the data
        let data = cpu.pull_word(&mut cycles, &mut mem);
        assert_eq!(data, 0x2A37);
        assert_eq!(cpu.sp, 0xFD); // the SP must return to its initial position
    }

    #[test]
    fn stack_can_push_and_pull_array_of_words() {
        let (mut cpu, mut mem) = setup_cpu_mem();
        let mut cycles = 40; // push and pull must cost 40 cycles
        for i in 0x2A2A..0x2A34 {
            cpu.push_word(&mut cycles, i, &mut mem);
            let mut addr = 0x0100 | (cpu.sp + 1) as usize;
            assert_eq!(mem[addr], i as Byte); // check lo first
            addr += 1;
            assert_eq!(mem[addr], (i >> 8) as Byte); // check hi
        }
        assert_eq!(cpu.sp, 0xFD - 20); // check if the SP decremented by 20 after storing the data
        for i in (0x2A2A..0x2A34).rev() {
            let data = cpu.pull_word(&mut cycles, &mut mem);
            assert_eq!(data, i);
        }
        assert_eq!(cpu.sp, 0xFD); // the SP must return to its initial position
    }
}
