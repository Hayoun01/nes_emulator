use crate::bus::Byte;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
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
            _ => Err("unknown CPU instruction"),
        }
    }
}

impl From<Instruction> for Byte {
    fn from(val: Instruction) -> Self {
        val as Byte
    }
}
