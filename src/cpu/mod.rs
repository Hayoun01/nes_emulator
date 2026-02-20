use std::collections::BTreeMap;

use crate::{
    bus::{Bus, Byte, Word},
    cpu::instructions::AddrMode,
};
use bitflags::{Flags, bitflags};

pub mod addressing;
pub mod instructions;
pub mod stack;

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

    // * Internals
    pub fetched: Byte,
    pub addr_abs: Word,
    pub opcode: Byte,
    pub bus: Option<Box<dyn Bus>>,
    pub general_cycles: u64,
    pub cycles: Byte,
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
            fetched: 0,
            addr_abs: 0,
            opcode: 0,
            bus: None,
            general_cycles: 0,
            cycles: 0,
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

    pub fn connect_bus(&mut self, bus: Box<dyn Bus>) {
        if self.bus.is_none() {
            self.bus = Some(bus);
        }
    }

    pub fn read_byte(&mut self, addr: Word) -> Byte {
        match self.bus {
            Some(ref mut bus) => bus.read(addr, false),
            _ => panic!("You must connect to a bus first"),
        }
    }

    pub fn read(&mut self, addr: Word, read_only: bool) -> Byte {
        match self.bus {
            Some(ref mut bus) => bus.read(addr, read_only),
            _ => panic!("You must connect to a bus first"),
        }
    }

    pub fn write(&mut self, addr: Word, data: Byte) {
        match self.bus {
            Some(ref mut bus) => {
                bus.write(addr, data);
            }
            _ => panic!("You must connect to a bus first"),
        }
    }

    pub fn execute(&mut self) -> i32 {
        let mut cycles: i32 = 0;
        self.opcode = self.fetch_byte();
        let ins = &Self::INSTRUCTIONS[self.opcode as usize];
        let additional_1 = self.resolve_addr(ins.addr_mode);
        let additional_2 = (ins.operate)(self);
        cycles += ins.cycles as i32;
        cycles += (additional_1 & additional_2) as i32;
        self.general_cycles += cycles as u64;
        cycles
    }

    /// Advances the CPU by a single clock cycle.
    pub fn clock(&mut self) {
        if self.cycles == 0 {
            self.opcode = self.fetch_byte();
            let ins = &Self::INSTRUCTIONS[self.opcode as usize];
            let additional_1 = self.resolve_addr(ins.addr_mode);
            let additional_2 = (ins.operate)(self);
            self.cycles += ins.cycles;
            self.cycles += additional_1 & additional_2;
        }
        self.cycles -= 1;
        self.general_cycles += 1;
    }

    pub fn load_program(&mut self, program: &[Byte]) {
        let mut iter = program.iter();
        if program.len() >= 2 {
            let mut start_addr = {
                let mut tmp = *iter.next().unwrap() as Word;
                tmp |= (*iter.next().unwrap() as Word) << 8;
                tmp
            };
            for &v in iter {
                self.write(start_addr, v);
                start_addr += 1;
            }
        }
    }

    fn fetch_byte(&mut self) -> Byte {
        let data = self.read_byte(self.pc);
        self.pc = self.pc.wrapping_add(1);
        data
    }

    fn read_byte_zp(&mut self, addr: Byte) -> Byte {
        self.read_byte(addr as Word)
    }

    fn fetch_word(&mut self) -> Word {
        let lo = self.fetch_byte();
        let hi = self.fetch_byte();
        Word::from_le_bytes([lo, hi])
    }

    fn read_word(&mut self, addr: Word) -> Word {
        let lo = self.read_byte(addr);
        let hi = self.read_byte(addr.wrapping_add(1));
        Word::from_le_bytes([lo, hi])
    }

    fn read_word_zp(&mut self, addr: Byte) -> Word {
        let lo = self.read_byte_zp(addr);
        let hi = self.read_byte_zp(addr.wrapping_add(1));
        Word::from_le_bytes([lo, hi])
    }

    fn page_crossed(base: Word, hi: Byte) -> bool {
        base & 0xFF00 != ((hi as Word) << 8)
    }

    pub fn disassemble(&mut self, start: Word, stop: Word) -> BTreeMap<Word, String> {
        let mut lines = BTreeMap::new();
        let mut addr: u32 = start as u32;
        let mut line_addr;
        while addr <= stop as u32 {
            line_addr = addr;
            let mut ins = format!("${:04X}: ", line_addr);
            let opcode = self.read(addr as Word, true);
            addr += 1;
            ins += Self::INSTRUCTIONS[opcode as usize].name;
            ins += " ";
            match Self::INSTRUCTIONS[opcode as usize].addr_mode {
                AddrMode::IMP => {
                    ins += &format!("{{IMP}}");
                }
                AddrMode::IMM => {
                    let v = self.read(addr as Word, true);
                    addr += 1;
                    ins += &format!("#${v:02X} {{IMM}}");
                }
                AddrMode::ZPG => {
                    let byte = self.read(addr as Word, true);
                    addr += 1;
                    ins += &format!("${:02X} {{ZPG}}", byte);
                }
                AddrMode::ZPX => {
                    let byte = self.read(addr as Word, true);
                    addr += 1;
                    ins += &format!("${:02X}, X {{ZPX}}", byte);
                }
                AddrMode::ZPY => {
                    let byte = self.read(addr as Word, true);
                    addr += 1;
                    ins += &format!("${:02X}, Y {{ZPY}}", byte);
                }
                AddrMode::IDX => {
                    let byte = self.read(addr as Word, true);
                    addr += 1;
                    ins += &format!("(${:02X}, X) {{IDX}}", byte);
                }
                AddrMode::IDY => {
                    let byte = self.read(addr as Word, true);
                    addr += 1;
                    ins += &format!("(${:02X}, Y) {{IDY}}", byte);
                }
                AddrMode::ABS => {
                    let lo = self.read(addr as Word, true);
                    addr += 1;
                    let hi = self.read(addr as Word, true);
                    addr += 1;
                    ins += &format!("${:04X} {{ABS}}", Word::from_le_bytes([lo, hi]));
                }
                AddrMode::ABX => {
                    let lo = self.read(addr as Word, true);
                    addr += 1;
                    let hi = self.read(addr as Word, true);
                    addr += 1;
                    ins += &format!("${:04X}, X {{ABX}}", Word::from_le_bytes([lo, hi]));
                }
                AddrMode::ABY => {
                    let lo = self.read(addr as Word, true);
                    addr += 1;
                    let hi = self.read(addr as Word, true);
                    addr += 1;
                    ins += &format!("${:04X}, Y {{ABY}}", Word::from_le_bytes([lo, hi]));
                }
                AddrMode::IND => {
                    let lo = self.read(addr as Word, true);
                    addr += 1;
                    let hi = self.read(addr as Word, true);
                    addr += 1;
                    ins += &format!("(${:04X}) {{IND}}", Word::from_le_bytes([lo, hi]));
                }
                _ => {}
            }
            lines.insert(line_addr as Word, ins);
        }
        lines
    }
}
