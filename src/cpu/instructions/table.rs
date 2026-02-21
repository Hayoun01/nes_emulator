use super::opcode::Opcode;
use crate::{
    bus::Byte,
    cpu::{CPU, Flag, addressing::AddrMode},
};

type OperateFn = fn(&mut CPU) -> Byte;

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub name: &'static str,
    pub operate: OperateFn,
    pub addr_mode: AddrMode,
    pub cycles: Byte,
}

impl Instruction {
    const fn new(
        name: &'static str,
        operate: OperateFn,
        addr_mode: AddrMode,
        cycles: Byte,
    ) -> Self {
        Self {
            name,
            operate,
            addr_mode,
            cycles,
        }
    }
}

impl CPU {
    pub const INSTRUCTIONS: [Instruction; 256] = {
        let mut t = [Instruction::new("???", Self::xxx, AddrMode::XXX, 0); 256];
        // * LDA Instruction
        t[Opcode::LdaIMM as usize] = Instruction::new("LDA", Self::lda, AddrMode::IMM, 2);
        t[Opcode::LdaZPG as usize] = Instruction::new("LDA", Self::lda, AddrMode::ZPG, 3);
        t[Opcode::LdaZPX as usize] = Instruction::new("LDA", Self::lda, AddrMode::ZPX, 4);
        t[Opcode::LdaABS as usize] = Instruction::new("LDA", Self::lda, AddrMode::ABS, 4);
        t[Opcode::LdaABX as usize] = Instruction::new("LDA", Self::lda, AddrMode::ABX, 4);
        t[Opcode::LdaABY as usize] = Instruction::new("LDA", Self::lda, AddrMode::ABY, 4);
        t[Opcode::LdaIDX as usize] = Instruction::new("LDA", Self::lda, AddrMode::IDX, 6);
        t[Opcode::LdaIDY as usize] = Instruction::new("LDA", Self::lda, AddrMode::IDY, 5);
        // * LDX Instruction
        t[Opcode::LdxIMM as usize] = Instruction::new("LDX", Self::ldx, AddrMode::IMM, 2);
        t[Opcode::LdxZPG as usize] = Instruction::new("LDX", Self::ldx, AddrMode::ZPG, 3);
        t[Opcode::LdxZPY as usize] = Instruction::new("LDX", Self::ldx, AddrMode::ZPY, 4);
        t[Opcode::LdxABS as usize] = Instruction::new("LDX", Self::ldx, AddrMode::ABS, 4);
        t[Opcode::LdxABY as usize] = Instruction::new("LDX", Self::ldx, AddrMode::ABY, 4);
        // * LDY Instruction
        t[Opcode::LdyIMM as usize] = Instruction::new("LDY", Self::ldy, AddrMode::IMM, 2);
        t[Opcode::LdyZPG as usize] = Instruction::new("LDY", Self::ldy, AddrMode::ZPG, 3);
        t[Opcode::LdyZPX as usize] = Instruction::new("LDY", Self::ldy, AddrMode::ZPX, 4);
        t[Opcode::LdyABS as usize] = Instruction::new("LDY", Self::ldy, AddrMode::ABS, 4);
        t[Opcode::LdyABX as usize] = Instruction::new("LDY", Self::ldy, AddrMode::ABX, 4);
        // * STA Instruction
        t[Opcode::StaZPG as usize] = Instruction::new("STA", Self::sta, AddrMode::ZPG, 3);
        t[Opcode::StaZPX as usize] = Instruction::new("STA", Self::sta, AddrMode::ZPX, 4);
        t[Opcode::StaABS as usize] = Instruction::new("STA", Self::sta, AddrMode::ABS, 4);
        t[Opcode::StaABX as usize] = Instruction::new("STA", Self::sta, AddrMode::ABX, 5);
        t[Opcode::StaABY as usize] = Instruction::new("STA", Self::sta, AddrMode::ABY, 5);
        t[Opcode::StaIDX as usize] = Instruction::new("STA", Self::sta, AddrMode::IDX, 6);
        t[Opcode::StaIDY as usize] = Instruction::new("STA", Self::sta, AddrMode::IDY, 6);
        // * STX Instruction
        t[Opcode::StxZPG as usize] = Instruction::new("STX", Self::stx, AddrMode::ZPG, 3);
        t[Opcode::StxZPY as usize] = Instruction::new("STX", Self::stx, AddrMode::ZPY, 4);
        t[Opcode::StxABS as usize] = Instruction::new("STX", Self::stx, AddrMode::ABS, 4);
        // * STY Instruction
        t[Opcode::StyZPG as usize] = Instruction::new("STY", Self::sty, AddrMode::ZPG, 3);
        t[Opcode::StyZPX as usize] = Instruction::new("STY", Self::sty, AddrMode::ZPX, 4);
        t[Opcode::StyABS as usize] = Instruction::new("STY", Self::sty, AddrMode::ABS, 4);
        // * JSR Instruction
        t[Opcode::JsrABS as usize] = Instruction::new("JSR", Self::jsr, AddrMode::ABS, 6);
        // * RTS Instruction
        t[Opcode::RtsIMP as usize] = Instruction::new("RTS", Self::rts, AddrMode::IMP, 6);
        // * JMP Instruction
        t[Opcode::JmpABS as usize] = Instruction::new("JMP", Self::jmp, AddrMode::ABS, 3);
        t[Opcode::JmpIND as usize] = Instruction::new("JMP", Self::jmp, AddrMode::IND, 5);
        // * TSX Instruction
        t[Opcode::TsxIMP as usize] = Instruction::new("TSX", Self::tsx, AddrMode::IMP, 2);
        // * TXS Instruction
        t[Opcode::TxsIMP as usize] = Instruction::new("TXS", Self::txs, AddrMode::IMP, 2);
        // * PHA Instruction
        t[Opcode::PhaIMP as usize] = Instruction::new("PHA", Self::pha, AddrMode::IMP, 3);
        // * PHP Instruction
        t[Opcode::PhpIMP as usize] = Instruction::new("PHP", Self::php, AddrMode::IMP, 3);
        // * PLA Instruction
        t[Opcode::PlaIMP as usize] = Instruction::new("PLA", Self::pla, AddrMode::IMP, 4);
        // * PLP Instruction
        t[Opcode::PlpIMP as usize] = Instruction::new("PLP", Self::plp, AddrMode::IMP, 4);
        // * AND Instruction
        t[Opcode::AndIMM as usize] = Instruction::new("AND", Self::and, AddrMode::IMM, 2);
        t[Opcode::AndZPG as usize] = Instruction::new("AND", Self::and, AddrMode::ZPG, 3);
        t[Opcode::AndZPX as usize] = Instruction::new("AND", Self::and, AddrMode::ZPX, 4);
        t[Opcode::AndABS as usize] = Instruction::new("AND", Self::and, AddrMode::ABS, 4);
        t[Opcode::AndABX as usize] = Instruction::new("AND", Self::and, AddrMode::ABX, 4);
        t[Opcode::AndABY as usize] = Instruction::new("AND", Self::and, AddrMode::ABY, 4);
        t[Opcode::AndIDX as usize] = Instruction::new("AND", Self::and, AddrMode::IDX, 6);
        t[Opcode::AndIDY as usize] = Instruction::new("AND", Self::and, AddrMode::IDY, 5);
        // * EOR Instruction
        t[Opcode::EorIMM as usize] = Instruction::new("EOR", Self::eor, AddrMode::IMM, 2);
        t[Opcode::EorZPG as usize] = Instruction::new("EOR", Self::eor, AddrMode::ZPG, 3);
        t[Opcode::EorZPX as usize] = Instruction::new("EOR", Self::eor, AddrMode::ZPX, 4);
        t[Opcode::EorABS as usize] = Instruction::new("EOR", Self::eor, AddrMode::ABS, 4);
        t[Opcode::EorABX as usize] = Instruction::new("EOR", Self::eor, AddrMode::ABX, 4);
        t[Opcode::EorABY as usize] = Instruction::new("EOR", Self::eor, AddrMode::ABY, 4);
        t[Opcode::EorIDX as usize] = Instruction::new("EOR", Self::eor, AddrMode::IDX, 6);
        t[Opcode::EorIDY as usize] = Instruction::new("EOR", Self::eor, AddrMode::IDY, 5);
        // * ORA Instruction
        t[Opcode::OraIMM as usize] = Instruction::new("ORA", Self::ora, AddrMode::IMM, 2);
        t[Opcode::OraZPG as usize] = Instruction::new("ORA", Self::ora, AddrMode::ZPG, 3);
        t[Opcode::OraZPX as usize] = Instruction::new("ORA", Self::ora, AddrMode::ZPX, 4);
        t[Opcode::OraABS as usize] = Instruction::new("ORA", Self::ora, AddrMode::ABS, 4);
        t[Opcode::OraABX as usize] = Instruction::new("ORA", Self::ora, AddrMode::ABX, 4);
        t[Opcode::OraABY as usize] = Instruction::new("ORA", Self::ora, AddrMode::ABY, 4);
        t[Opcode::OraIDX as usize] = Instruction::new("ORA", Self::ora, AddrMode::IDX, 6);
        t[Opcode::OraIDY as usize] = Instruction::new("ORA", Self::ora, AddrMode::IDY, 5);
        // * BIT Instruction
        t[Opcode::BitZPG as usize] = Instruction::new("BIT", Self::bit, AddrMode::ZPG, 3);
        t[Opcode::BitABS as usize] = Instruction::new("BIT", Self::bit, AddrMode::ABS, 4);

        // * TAX Instruction
        t[Opcode::TaxIMP as usize] = Instruction::new("TAX", Self::tax, AddrMode::IMP, 2);
        // * TAY Instruction
        t[Opcode::TayIMP as usize] = Instruction::new("TAY", Self::tay, AddrMode::IMP, 2);
        // * TXA Instruction
        t[Opcode::TxaIMP as usize] = Instruction::new("TXA", Self::txa, AddrMode::IMP, 2);
        // * TYA Instruction
        t[Opcode::TyaIMP as usize] = Instruction::new("TYA", Self::tya, AddrMode::IMP, 2);
        // * INC Instruction
        t[Opcode::IncZPG as usize] = Instruction::new("INC", Self::inc, AddrMode::ZPG, 5);
        t[Opcode::IncZPX as usize] = Instruction::new("INC", Self::inc, AddrMode::ZPX, 6);
        t[Opcode::IncABS as usize] = Instruction::new("INC", Self::inc, AddrMode::ABS, 6);
        t[Opcode::IncABX as usize] = Instruction::new("INC", Self::inc, AddrMode::ABX, 7);
        // * INX Instruction
        t[Opcode::InxIMP as usize] = Instruction::new("INX", Self::inx, AddrMode::IMP, 2);
        // * INY Instruction
        t[Opcode::InyIMP as usize] = Instruction::new("INY", Self::iny, AddrMode::IMP, 2);

        // * DEC Instruction
        t[Opcode::DecZPG as usize] = Instruction::new("DEC", Self::dec, AddrMode::ZPG, 5);
        t[Opcode::DecZPX as usize] = Instruction::new("DEC", Self::dec, AddrMode::ZPX, 6);
        t[Opcode::DecABS as usize] = Instruction::new("DEC", Self::dec, AddrMode::ABS, 6);
        t[Opcode::DecABX as usize] = Instruction::new("DEC", Self::dec, AddrMode::ABX, 7);
        // * DEX Instruction
        t[Opcode::DexIMP as usize] = Instruction::new("DEX", Self::dex, AddrMode::IMP, 2);
        // * DEY Instruction
        t[Opcode::DeyIMP as usize] = Instruction::new("DEY", Self::dey, AddrMode::IMP, 2);
        t
    };

    fn xxx(&mut self) -> Byte {
        panic!("Illegal instruction!")
    }

    fn fetch(&mut self) -> Byte {
        if Self::INSTRUCTIONS[self.opcode as usize].addr_mode != AddrMode::IMP {
            self.fetched = self.read_byte(self.addr_abs);
        }
        return self.fetched;
    }

    fn set_a_flags(&mut self) {
        self.flag.set(Flag::ZERO, self.a == 0);
        self.flag.set(Flag::NEGATIVE, (self.a & 0x80) != 0);
    }

    fn set_x_flags(&mut self) {
        self.flag.set(Flag::ZERO, self.x == 0);
        self.flag.set(Flag::NEGATIVE, (self.x & 0x80) != 0);
    }

    fn set_y_flags(&mut self) {
        self.flag.set(Flag::ZERO, self.y == 0);
        self.flag.set(Flag::NEGATIVE, (self.y & 0x80) != 0);
    }

    fn lda(&mut self) -> Byte {
        self.fetch();
        self.a = self.fetched;
        self.set_a_flags();
        1
    }
    fn ldx(&mut self) -> Byte {
        self.fetch();
        self.x = self.fetched;
        self.set_x_flags();
        1
    }
    fn ldy(&mut self) -> Byte {
        self.fetch();
        self.y = self.fetched;
        self.set_y_flags();
        1
    }
    fn sta(&mut self) -> Byte {
        self.write(self.addr_abs, self.a);
        0
    }
    fn stx(&mut self) -> Byte {
        self.write(self.addr_abs, self.x);
        0
    }
    fn sty(&mut self) -> Byte {
        self.write(self.addr_abs, self.y);
        0
    }
    fn jsr(&mut self) -> Byte {
        self.pc -= 1;
        self.push_word(self.pc);
        self.pc = self.addr_abs;
        0
    }
    fn rts(&mut self) -> Byte {
        let addr = self.pull_word();
        self.pc = addr + 1;
        0
    }
    fn jmp(&mut self) -> Byte {
        self.pc = self.addr_abs;
        0
    }
    fn tsx(&mut self) -> Byte {
        self.x = self.sp;
        self.set_x_flags();
        0
    }
    fn txs(&mut self) -> Byte {
        self.sp = self.x;
        0
    }
    fn pha(&mut self) -> Byte {
        self.push_byte(self.a);
        0
    }
    fn php(&mut self) -> Byte {
        self.push_byte(self.flag.bits());
        0
    }
    fn pla(&mut self) -> Byte {
        self.a = self.pull_byte();
        self.set_a_flags();
        0
    }
    fn plp(&mut self) -> Byte {
        let bits = self.pull_byte();
        self.flag = Flag::from_bits_truncate(bits);
        0
    }
    fn and(&mut self) -> Byte {
        self.fetch();
        self.a = self.a & self.fetched;
        self.set_a_flags();
        1
    }
    fn eor(&mut self) -> Byte {
        self.fetch();
        self.a = self.a ^ self.fetched;
        self.set_a_flags();
        1
    }
    fn ora(&mut self) -> Byte {
        self.fetch();
        self.a = self.a | self.fetched;
        self.set_a_flags();
        1
    }
    fn bit(&mut self) -> Byte {
        self.fetch();
        self.flag.set(Flag::ZERO, (self.a & self.fetched) == 0);
        self.flag.set(Flag::OVERFLOW, (self.fetched & 0x40) != 0);
        self.flag.set(Flag::NEGATIVE, (self.fetched & 0x80) != 0);
        0
    }
    fn tax(&mut self) -> Byte {
        self.x = self.a;
        self.set_x_flags();
        0
    }
    fn tay(&mut self) -> Byte {
        self.y = self.a;
        self.set_y_flags();
        0
    }
    fn txa(&mut self) -> Byte {
        self.a = self.x;
        self.set_a_flags();
        0
    }
    fn tya(&mut self) -> Byte {
        self.a = self.y;
        self.set_a_flags();
        0
    }
    fn inc(&mut self) -> Byte {
        self.fetch();
        let tmp = self.fetched.wrapping_add(1);
        self.write(self.addr_abs, tmp);
        self.flag = Flag::empty();
        self.flag.set(Flag::ZERO, tmp == 0);
        self.flag.set(Flag::NEGATIVE, (tmp & 0x80) != 0);
        0
    }
    fn dec(&mut self) -> Byte {
        self.fetch();
        let tmp = self.fetched.wrapping_sub(1);
        self.write(self.addr_abs, tmp);
        self.flag = Flag::empty();
        self.flag.set(Flag::ZERO, tmp == 0);
        self.flag.set(Flag::NEGATIVE, (tmp & 0x80) != 0);
        0
    }
    fn inx(&mut self) -> Byte {
        self.x = self.x.wrapping_add(1);
        self.set_x_flags();
        0
    }
    fn iny(&mut self) -> Byte {
        self.y = self.y.wrapping_add(1);
        self.set_y_flags();
        0
    }
    fn dex(&mut self) -> Byte {
        self.x = self.x.wrapping_sub(1);
        self.set_x_flags();
        0
    }
    fn dey(&mut self) -> Byte {
        self.y = self.y.wrapping_sub(1);
        self.set_y_flags();
        0
    }
    fn _tmp(&mut self) -> Byte {
        0
    }
}
