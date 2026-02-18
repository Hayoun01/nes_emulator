use crate::{
    bus::Byte,
    cpu::{CPU, Flag},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Opcode {
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
    // * [JMP] Jump
    /// ### Jump Absolute
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x4C | 3 | 3 |
    JmpABS = 0x4C,
    /// ### Jump indirect
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x6C | 3 | 5 |
    JmpIND = 0x6C,
    // * [TSX] Transfer Stack Pointer to X
    /// ### Transfer Stack Pointer to X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xBA | 1 | 2 |
    TsxIMP = 0xBA,
    // * [TSX] Transfer X to Stack Pointer
    /// ### Transfer X to Stack Pointer
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x9A | 1 | 2 |
    TxsIMP = 0x9A,
    // * [PHA] Push Accumulator
    /// ### Push Accumulator to the stack
    /// Pushes a copy of the accumulator on to the stack.
    ///
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x48 | 1 | 3 |
    PhaIMP = 0x48,
    // * [PHP] Push Processor Status
    /// ### Push Processor Status
    /// Pushes a copy of the status flags on to the stack.
    ///
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x08 | 1 | 3 |
    PhpIMP = 0x08,
    // * [PLA] Pull Accumulator
    /// ### Pull Accumulator
    /// Pulls an 8 bit value from the stack and into the accumulator. The zero and negative flags are set as appropriate.
    ///
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x68 | 1 | 4 |
    PlaIMP = 0x68,
    // * [PLP] Pull Processor Status
    /// ### Pull Processor Status
    /// Pulls an 8 bit value from the stack and into the processor flags. The flags will take on new states as determined by the value pulled.
    ///
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x28 | 1 | 4 |
    PlpIMP = 0x28,
    // * [AND] Logical AND
    /// ### Logical AND Immediate
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x29 | 2 | 2 |
    AndIMM = 0x29,
    /// ### Logical AND Zero Page
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x25 | 2 | 3 |
    AndZPG = 0x25,
    /// ### Logical AND Zero Page X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x35 | 2 | 4 |
    AndZPX = 0x35,
    /// ### Logical AND Absolute
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x2D | 3 | 4 |
    AndABS = 0x2D,
    /// ### Logical AND Absolute X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x3D | 3 | 4 (+1 if page crossed) |
    AndABX = 0x3D,
    /// ### Logical AND Absolute Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x39 | 3 | 4 (+1 if page crossed) |
    AndABY = 0x39,
    /// ### Logical AND Indexed Indirect X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x21 | 2 | 6 |
    AndIDX = 0x21,
    /// ### Logical AND Indirect Indexed Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x31 | 2 | 5 (+1 if page crossed) |
    AndIDY = 0x31,
    // * [EOR] Exclusive OR
    /// ### Exclusive OR Immediate
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x49 | 2 | 2 |
    EorIMM = 0x49,
    /// ### Exclusive OR Zero Page
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x05 | 2 | 3 |
    EorZPG = 0x05,
    /// ### Exclusive OR Zero Page X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x15 | 2 | 4 |
    EorZPX = 0x15,
    /// ### Exclusive OR Absolute
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x0D | 3 | 4 |
    EorABS = 0x0D,
    /// ### Exclusive OR Absolute X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x1D | 3 | 4 (+1 if page crossed) |
    EorABX = 0x1D,
    /// ### Exclusive OR Absolute Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x19 | 3 | 4 (+1 if page crossed) |
    EorABY = 0x19,
    /// ### Exclusive OR Indexed Indirect X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x01 | 2 | 6 |
    EorIDX = 0x01,
    /// ### Exclusive OR Indirect Indexed Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x11 | 2 | 5 (+1 if page crossed) |
    EorIDY = 0x11,
    // * [ORA] Logical Inclusive OR
    /// ### Logical Inclusive OR Immediate
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x09 | 2 | 2 |
    OraIMM = 0x09,
    /// ### Logical Inclusive OR Zero Page
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x45 | 2 | 3 |
    OraZPG = 0x45,
    /// ### Logical Inclusive OR Zero Page X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x55 | 2 | 4 |
    OraZPX = 0x55,
    /// ### Logical Inclusive OR Absolute
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x4D | 3 | 4 |
    OraABS = 0x4D,
    /// ### Logical Inclusive OR Absolute X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x5D | 3 | 4 (+1 if page crossed) |
    OraABX = 0x5D,
    /// ### Logical Inclusive OR Absolute Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x59 | 3 | 4 (+1 if page crossed) |
    OraABY = 0x59,
    /// ### Logical Inclusive OR Indexed Indirect X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x41 | 2 | 6 |
    OraIDX = 0x41,
    /// ### Logical Inclusive OR Indirect Indexed Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x51 | 2 | 5 (+1 if page crossed) |
    OraIDY = 0x51,
    // * [BIT] Bit Test
    /// ### Bit Test Zero Page
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x24 | 2 | 3 |
    BitZPG = 0x24,
    /// ### Bit Test Zero Page
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x2C | 3 | 4 |
    BitABS = 0x2C,
    // * [TAX] Transfer Accumulator to X
    /// ### Transfer Accumulator to X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xAA | 1 | 2 |
    TaxIMP = 0xAA,
    // * [TAY] Transfer Accumulator to Y
    /// ### Transfer Accumulator to Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xA8 | 1 | 2 |
    TayIMP = 0xA8,
    // * [TXA] Transfer X to Accumulator
    /// ### Transfer X to Accumulator
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x8A | 1 | 2 |
    TxaIMP = 0x8A,
    // * [TYA] Transfer Y to Accumulator
    /// ### Transfer Y to Accumulator
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x98 | 1 | 2 |
    TyaIMP = 0x98,
}

impl TryFrom<Byte> for Opcode {
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
            // * [JMP]
            x if x == Self::JmpABS as Byte => Ok(Self::JmpABS),
            x if x == Self::JmpIND as Byte => Ok(Self::JmpIND),
            // * [TSX]
            x if x == Self::TsxIMP as Byte => Ok(Self::TsxIMP),
            // * [TSX]
            x if x == Self::TxsIMP as Byte => Ok(Self::TxsIMP),
            // * [PHA]
            x if x == Self::PhaIMP as Byte => Ok(Self::PhaIMP),
            // * [PHP]
            x if x == Self::PhpIMP as Byte => Ok(Self::PhpIMP),
            // * [PLA]
            x if x == Self::PlaIMP as Byte => Ok(Self::PlaIMP),
            // * [PLP]
            x if x == Self::PlpIMP as Byte => Ok(Self::PlpIMP),
            // * [AND]
            x if x == Self::AndIMM as Byte => Ok(Self::AndIMM),
            x if x == Self::AndZPG as Byte => Ok(Self::AndZPG),
            x if x == Self::AndZPX as Byte => Ok(Self::AndZPX),
            x if x == Self::AndABS as Byte => Ok(Self::AndABS),
            x if x == Self::AndABX as Byte => Ok(Self::AndABX),
            x if x == Self::AndABY as Byte => Ok(Self::AndABY),
            x if x == Self::AndIDX as Byte => Ok(Self::AndIDX),
            x if x == Self::AndIDY as Byte => Ok(Self::AndIDY),
            // * [EOR]
            x if x == Self::EorIMM as Byte => Ok(Self::EorIMM),
            x if x == Self::EorZPG as Byte => Ok(Self::EorZPG),
            x if x == Self::EorZPX as Byte => Ok(Self::EorZPX),
            x if x == Self::EorABS as Byte => Ok(Self::EorABS),
            x if x == Self::EorABX as Byte => Ok(Self::EorABX),
            x if x == Self::EorABY as Byte => Ok(Self::EorABY),
            x if x == Self::EorIDX as Byte => Ok(Self::EorIDX),
            x if x == Self::EorIDY as Byte => Ok(Self::EorIDY),
            // * [Ora]
            x if x == Self::OraIMM as Byte => Ok(Self::OraIMM),
            x if x == Self::OraZPG as Byte => Ok(Self::OraZPG),
            x if x == Self::OraZPX as Byte => Ok(Self::OraZPX),
            x if x == Self::OraABS as Byte => Ok(Self::OraABS),
            x if x == Self::OraABX as Byte => Ok(Self::OraABX),
            x if x == Self::OraABY as Byte => Ok(Self::OraABY),
            x if x == Self::OraIDX as Byte => Ok(Self::OraIDX),
            x if x == Self::OraIDY as Byte => Ok(Self::OraIDY),
            // * [BIT]
            x if x == Self::BitZPG as Byte => Ok(Self::BitZPG),
            x if x == Self::BitABS as Byte => Ok(Self::BitABS),
            // * [TAX]
            x if x == Self::TaxIMP as Byte => Ok(Self::TaxIMP),
            // * [TAY]
            x if x == Self::TayIMP as Byte => Ok(Self::TayIMP),
            // * [TXA]
            x if x == Self::TxaIMP as Byte => Ok(Self::TxaIMP),
            // * [TYA]
            x if x == Self::TyaIMP as Byte => Ok(Self::TyaIMP),
            _ => Err("unknown CPU instruction"),
        }
    }
}

impl From<Opcode> for Byte {
    fn from(val: Opcode) -> Self {
        val as Byte
    }
}

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
    XXX,
}

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
        // * LDA Instructions
        t[Opcode::LdaIMM as usize] = Instruction::new("LDA", Self::lda, AddrMode::IMM, 2);
        t[Opcode::LdaZPG as usize] = Instruction::new("LDA", Self::lda, AddrMode::ZPG, 3);
        t[Opcode::LdaZPX as usize] = Instruction::new("LDA", Self::lda, AddrMode::ZPX, 4);
        t[Opcode::LdaABS as usize] = Instruction::new("LDA", Self::lda, AddrMode::ABS, 4);
        t[Opcode::LdaABX as usize] = Instruction::new("LDA", Self::lda, AddrMode::ABX, 4);
        t[Opcode::LdaABY as usize] = Instruction::new("LDA", Self::lda, AddrMode::ABY, 4);
        t[Opcode::LdaIDX as usize] = Instruction::new("LDA", Self::lda, AddrMode::IDX, 6);
        t[Opcode::LdaIDY as usize] = Instruction::new("LDA", Self::lda, AddrMode::IDY, 5);
        // * LDX Instructions
        t[Opcode::LdxIMM as usize] = Instruction::new("LDX", Self::ldx, AddrMode::IMM, 2);
        t[Opcode::LdxZPG as usize] = Instruction::new("LDX", Self::ldx, AddrMode::ZPG, 3);
        t[Opcode::LdxZPY as usize] = Instruction::new("LDX", Self::ldx, AddrMode::ZPY, 4);
        t[Opcode::LdxABS as usize] = Instruction::new("LDX", Self::ldx, AddrMode::ABS, 4);
        t[Opcode::LdxABY as usize] = Instruction::new("LDX", Self::ldx, AddrMode::ABY, 4);
        // * LDY Instructions
        t[Opcode::LdyIMM as usize] = Instruction::new("LDY", Self::ldy, AddrMode::IMM, 2);
        t[Opcode::LdyZPG as usize] = Instruction::new("LDY", Self::ldy, AddrMode::ZPG, 3);
        t[Opcode::LdyZPX as usize] = Instruction::new("LDY", Self::ldy, AddrMode::ZPX, 4);
        t[Opcode::LdyABS as usize] = Instruction::new("LDY", Self::ldy, AddrMode::ABS, 4);
        t[Opcode::LdyABX as usize] = Instruction::new("LDY", Self::ldy, AddrMode::ABX, 4);
        // * STA Instructions
        t[Opcode::StaZPG as usize] = Instruction::new("STA", Self::sta, AddrMode::ZPG, 3);
        t[Opcode::StaZPX as usize] = Instruction::new("STA", Self::sta, AddrMode::ZPX, 4);
        t[Opcode::StaABS as usize] = Instruction::new("STA", Self::sta, AddrMode::ABS, 4);
        t[Opcode::StaABX as usize] = Instruction::new("STA", Self::sta, AddrMode::ABX, 5);
        t[Opcode::StaABY as usize] = Instruction::new("STA", Self::sta, AddrMode::ABY, 5);
        t[Opcode::StaIDX as usize] = Instruction::new("STA", Self::sta, AddrMode::IDX, 6);
        t[Opcode::StaIDY as usize] = Instruction::new("STA", Self::sta, AddrMode::IDY, 6);
        // * STX Instructions
        t[Opcode::StxZPG as usize] = Instruction::new("STX", Self::stx, AddrMode::ZPG, 3);
        t[Opcode::StxZPY as usize] = Instruction::new("STX", Self::stx, AddrMode::ZPY, 4);
        t[Opcode::StxABS as usize] = Instruction::new("STX", Self::stx, AddrMode::ABS, 4);
        // * STY Instructions
        t[Opcode::StyZPG as usize] = Instruction::new("STY", Self::sty, AddrMode::ZPG, 3);
        t[Opcode::StyZPX as usize] = Instruction::new("STY", Self::sty, AddrMode::ZPX, 4);
        t[Opcode::StyABS as usize] = Instruction::new("STY", Self::sty, AddrMode::ABS, 4);
        // * JSR Instructions
        t[Opcode::JsrABS as usize] = Instruction::new("JSR", Self::jsr, AddrMode::ABS, 6);
        // * RTS Instructions
        t[Opcode::RtsIMP as usize] = Instruction::new("RTS", Self::rts, AddrMode::IMP, 6);
        // * JMP Instructions
        t[Opcode::JmpABS as usize] = Instruction::new("JMP", Self::jmp, AddrMode::ABS, 3);
        t[Opcode::JmpIND as usize] = Instruction::new("JMP", Self::jmp, AddrMode::IND, 5);
        // * TSX Instructions
        t[Opcode::TsxIMP as usize] = Instruction::new("TSX", Self::tsx, AddrMode::IMP, 2);
        // * TXS Instructions
        t[Opcode::TxsIMP as usize] = Instruction::new("TXS", Self::txs, AddrMode::IMP, 2);
        // * PHA Instructions
        t[Opcode::PhaIMP as usize] = Instruction::new("PHA", Self::pha, AddrMode::IMP, 3);
        // * PHP Instructions
        t[Opcode::PhpIMP as usize] = Instruction::new("PHP", Self::php, AddrMode::IMP, 3);
        // * PLA Instructions
        t[Opcode::PlaIMP as usize] = Instruction::new("PLA", Self::pla, AddrMode::IMP, 4);
        // * PLP Instructions
        t[Opcode::PlpIMP as usize] = Instruction::new("PLP", Self::plp, AddrMode::IMP, 4);
        // * AND Instructions
        t[Opcode::AndIMM as usize] = Instruction::new("AND", Self::and, AddrMode::IMM, 2);
        t[Opcode::AndZPG as usize] = Instruction::new("AND", Self::and, AddrMode::ZPG, 3);
        t[Opcode::AndZPX as usize] = Instruction::new("AND", Self::and, AddrMode::ZPX, 4);
        t[Opcode::AndABS as usize] = Instruction::new("AND", Self::and, AddrMode::ABS, 4);
        t[Opcode::AndABX as usize] = Instruction::new("AND", Self::and, AddrMode::ABX, 4);
        t[Opcode::AndABY as usize] = Instruction::new("AND", Self::and, AddrMode::ABY, 4);
        t[Opcode::AndIDX as usize] = Instruction::new("AND", Self::and, AddrMode::IDX, 6);
        t[Opcode::AndIDY as usize] = Instruction::new("AND", Self::and, AddrMode::IDY, 5);
        // * EOR Instructions
        t[Opcode::EorIMM as usize] = Instruction::new("EOR", Self::eor, AddrMode::IMM, 2);
        t[Opcode::EorZPG as usize] = Instruction::new("EOR", Self::eor, AddrMode::ZPG, 3);
        t[Opcode::EorZPX as usize] = Instruction::new("EOR", Self::eor, AddrMode::ZPX, 4);
        t[Opcode::EorABS as usize] = Instruction::new("EOR", Self::eor, AddrMode::ABS, 4);
        t[Opcode::EorABX as usize] = Instruction::new("EOR", Self::eor, AddrMode::ABX, 4);
        t[Opcode::EorABY as usize] = Instruction::new("EOR", Self::eor, AddrMode::ABY, 4);
        t[Opcode::EorIDX as usize] = Instruction::new("EOR", Self::eor, AddrMode::IDX, 6);
        t[Opcode::EorIDY as usize] = Instruction::new("EOR", Self::eor, AddrMode::IDY, 5);
        // * ORA Instructions
        t[Opcode::OraIMM as usize] = Instruction::new("ORA", Self::ora, AddrMode::IMM, 2);
        t[Opcode::OraZPG as usize] = Instruction::new("ORA", Self::ora, AddrMode::ZPG, 3);
        t[Opcode::OraZPX as usize] = Instruction::new("ORA", Self::ora, AddrMode::ZPX, 4);
        t[Opcode::OraABS as usize] = Instruction::new("ORA", Self::ora, AddrMode::ABS, 4);
        t[Opcode::OraABX as usize] = Instruction::new("ORA", Self::ora, AddrMode::ABX, 4);
        t[Opcode::OraABY as usize] = Instruction::new("ORA", Self::ora, AddrMode::ABY, 4);
        t[Opcode::OraIDX as usize] = Instruction::new("ORA", Self::ora, AddrMode::IDX, 6);
        t[Opcode::OraIDY as usize] = Instruction::new("ORA", Self::ora, AddrMode::IDY, 5);
        // * BIT Instructions
        t[Opcode::BitZPG as usize] = Instruction::new("BIT", Self::bit, AddrMode::ZPG, 3);
        t[Opcode::BitABS as usize] = Instruction::new("BIT", Self::bit, AddrMode::ABS, 4);

        // * TAX Instructions
        t[Opcode::TaxIMP as usize] = Instruction::new("TAX", Self::tax, AddrMode::IMP, 2);
        // * TAY Instructions
        t[Opcode::TayIMP as usize] = Instruction::new("TAY", Self::tay, AddrMode::IMP, 2);
        // * TXA Instructions
        t[Opcode::TxaIMP as usize] = Instruction::new("TXA", Self::txa, AddrMode::IMP, 2);
        // * TYA Instructions
        t[Opcode::TyaIMP as usize] = Instruction::new("TYA", Self::tya, AddrMode::IMP, 2);
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

    fn lda(&mut self) -> Byte {
        self.fetch();
        self.a = self.fetched;
        self.lda_set_status();
        1
    }
    fn ldx(&mut self) -> Byte {
        self.fetch();
        self.x = self.fetched;
        self.ldx_set_status();
        1
    }
    fn ldy(&mut self) -> Byte {
        self.fetch();
        self.y = self.fetched;
        self.ldy_set_status();
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
        self.ldx_set_status();
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
        self.lda_set_status();
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
        self.lda_set_status();
        1
    }
    fn eor(&mut self) -> Byte {
        self.fetch();
        self.a = self.a ^ self.fetched;
        self.lda_set_status();
        1
    }
    fn ora(&mut self) -> Byte {
        self.fetch();
        self.a = self.a | self.fetched;
        self.lda_set_status();
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
        self.ldx_set_status();
        0
    }
    fn tay(&mut self) -> Byte {
        self.y = self.a;
        self.ldy_set_status();
        0
    }
    fn txa(&mut self) -> Byte {
        self.a = self.x;
        self.lda_set_status();
        0
    }
    fn tya(&mut self) -> Byte {
        self.a = self.y;
        self.lda_set_status();
        0
    }
    fn _tmp(&mut self) -> Byte {
        0
    }

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
            AddrMode::XXX => self.xxx(),
        }
    }
}
