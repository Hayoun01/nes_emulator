use crate::bus::Byte;

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
    /// | 0x45 | 2 | 3 |
    EorZPG = 0x45,
    /// ### Exclusive OR Zero Page X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x55 | 2 | 4 |
    EorZPX = 0x55,
    /// ### Exclusive OR Absolute
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x4D | 3 | 4 |
    EorABS = 0x4D,
    /// ### Exclusive OR Absolute X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x5D | 3 | 4 (+1 if page crossed) |
    EorABX = 0x5D,
    /// ### Exclusive OR Absolute Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x59 | 3 | 4 (+1 if page crossed) |
    EorABY = 0x59,
    /// ### Exclusive OR Indexed Indirect X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x41 | 2 | 6 |
    EorIDX = 0x41,
    /// ### Exclusive OR Indirect Indexed Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x51 | 2 | 5 (+1 if page crossed) |
    EorIDY = 0x51,
    // * [ORA] Logical Inclusive OR
    /// ### Logical Inclusive OR Immediate
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x09 | 2 | 2 |
    OraIMM = 0x09,
    /// ### Logical Inclusive OR Zero Page
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x05 | 2 | 3 |
    OraZPG = 0x05,
    /// ### Logical Inclusive OR Zero Page X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x15 | 2 | 4 |
    OraZPX = 0x15,
    /// ### Logical Inclusive OR Absolute
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x0D | 3 | 4 |
    OraABS = 0x0D,
    /// ### Logical Inclusive OR Absolute X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x1D | 3 | 4 (+1 if page crossed) |
    OraABX = 0x1D,
    /// ### Logical Inclusive OR Absolute Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x19 | 3 | 4 (+1 if page crossed) |
    OraABY = 0x19,
    /// ### Logical Inclusive OR Indexed Indirect X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x01 | 2 | 6 |
    OraIDX = 0x01,
    /// ### Logical Inclusive OR Indirect Indexed Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x11 | 2 | 5 (+1 if page crossed) |
    OraIDY = 0x11,
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
    // * [INC] Increment Memory
    /// ### Increment Memory Zero Page
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xE6 | 2 | 5 |
    IncZPG = 0xE6,
    /// ### Increment Memory Zero Page X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xF6 | 2 | 6 |
    IncZPX = 0xF6,
    /// ### Increment Memory Absolute
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xEE | 3 | 6 |
    IncABS = 0xEE,
    /// ### Increment Memory Absolute X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xFE | 3 | 7 |
    IncABX = 0xFE,
    // * [INX] Increment X Register
    /// ### Increment X Register
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xE8 | 1 | 2 |
    InxIMP = 0xE8,
    // * [INY] Increment Y Register
    /// ### Increment Y Register
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xC8 | 1 | 2 |
    InyIMP = 0xC8,
    // * [DEC] Decrement Memory
    /// ### Decrement Memory Zero Page
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xC6 | 2 | 5 |
    DecZPG = 0xC6,
    /// ### Decrement Memory Zero Page X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xD6 | 2 | 6 |
    DecZPX = 0xD6,
    /// ### Decrement Memory Absolute
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xCE | 3 | 6 |
    DecABS = 0xCE,
    /// ### Decrement Memory Absolute X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xDE | 3 | 7 |
    DecABX = 0xDE,
    // * [DEX] Decrement X Register
    /// ### Decrement X Register
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xCA | 1 | 2 |
    DexIMP = 0xCA,
    // * [DEY] Decrement X Register
    /// ### Decrement X Register
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x88 | 1 | 2 |
    DeyIMP = 0x88,
    // * [BEQ] Branch if Equal
    /// ### Branch if Equal
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xF0 | 2 | 2 (+1 if branch succeeds) |
    /// | | | (+2 if to a new page) |
    BeqREL = 0xF0,
    // * [BNE] Branch if Not Equal
    /// ### Branch if Not Equal
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xD0 | 2 | 2 (+1 if branch succeeds) |
    /// | | | (+2 if to a new page) |
    BneREL = 0xD0,
    // * [BCC] Branch if Carry Clear
    /// ### Branch if Carry Clear
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x90 | 2 | 2 (+1 if branch succeeds) |
    /// | | | (+2 if to a new page) |
    BccREL = 0x90,
    // * [BCS] Branch if Carry Set
    /// ### Branch if Carry Set
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xB0 | 2 | 2 (+1 if branch succeeds) |
    /// | | | (+2 if to a new page) |
    BcsREL = 0xB0,
    // * [BMI] Branch if Minus
    /// ### Branch if Minus
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x30 | 2 | 2 (+1 if branch succeeds) |
    /// | | | (+2 if to a new page) |
    BmiREL = 0x30,
    // * [BPL] Branch if Positive
    /// ### Branch if Positive
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x10 | 2 | 2 (+1 if branch succeeds) |
    /// | | | (+2 if to a new page) |
    BplREL = 0x10,
    // * [BVS] Branch if Overflow Set
    /// ### Branch if Overflow Set
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x70 | 2 | 2 (+1 if branch succeeds) |
    /// | | | (+2 if to a new page) |
    BvsREL = 0x70,
    // * [BVC] Branch if Overflow Clear
    /// ### Branch if Overflow Clear
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x50 | 2 | 2 (+1 if branch succeeds) |
    /// | | | (+2 if to a new page) |
    BvcREL = 0x50,
    // * [CLC] Clear Carry Flag
    /// ### Clear Carry Flag
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x18 | 1 | 2 |
    ClcIMP = 0x18,
    // * [CLD] Clear Decimal Mode
    /// ### Clear Decimal Mode
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xD8 | 1 | 2 |
    CldIMP = 0xD8,
    // * [CLI] Clear Interrupt Disable
    /// ### Clear Interrupt Disable
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x58 | 1 | 2 |
    CliIMP = 0x58,
    // * [CLV] Clear Overflow Flag
    /// ### Clear Overflow Flag
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xB8 | 1 | 2 |
    ClvIMP = 0xB8,
    // * [SEC] Set Carry Flag
    /// ### Set Carry Flag
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x38 | 1 | 2 |
    SecIMP = 0x38,
    // * [SED] Set Decimal Flag
    /// ### Set Decimal Flag
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xF8 | 1 | 2 |
    SedIMP = 0xF8,
    // * [SEI] Set Interrupt Disable
    /// ### Set Interrupt Disable
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x78 | 1 | 2 |
    SeiIMP = 0x78,
    // * [ADC] Add with Carry
    /// ### Add with Carry Immediate
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x69 | 2 | 2 |
    AdcIMM = 0x69,
    /// ### Add with Carry Zero Page
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x65 | 2 | 3 |
    AdcZPG = 0x65,
    /// ### Add with Carry Zero Page X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x75 | 2 | 4 |
    AdcZPX = 0x75,
    /// ### Add with Carry Absolute
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x6D | 3 | 4 |
    AdcABS = 0x6D,
    /// ### Add with Carry Absolute X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x7D | 3 | 4 (+1 if page crossed) |
    AdcABX = 0x7D,
    /// ### Add with Carry Absolute Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x79 | 3 | 4 (+1 if page crossed) |
    AdcABY = 0x79,
    /// ### Add with Carry Indexed Indirect X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x61 | 2 | 6 |
    AdcIDX = 0x61,
    /// ### Add with Carry Indirect Indexed Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0x71 | 2 | 5 (+1 if page crossed) |
    AdcIDY = 0x71,
    // * [SBC] Subtract with Carry
    /// ### Subtract with Carry Immediate
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xE9 | 2 | 2 |
    SbcIMM = 0xE9,
    /// ### Subtract with Carry Zero Page
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xE5 | 2 | 3 |
    SbcZPG = 0xE5,
    /// ### Subtract with Carry Zero Page X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xF5 | 2 | 4 |
    SbcZPX = 0xF5,
    /// ### Subtract with Carry Absolute
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xED | 3 | 4 |
    SbcABS = 0xED,
    /// ### Subtract with Carry Absolute X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xFD | 3 | 4 (+1 if page crossed) |
    SbcABX = 0xFD,
    /// ### Subtract with Carry Absolute Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xF9 | 3 | 4 (+1 if page crossed) |
    SbcABY = 0xF9,
    /// ### Subtract with Carry Indexed Indirect X
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xE1 | 2 | 6 |
    SbcIDX = 0xE1,
    /// ### Subtract with Carry Indirect Indexed Y
    /// | Opcode | Bytes | Cycles |
    /// |--------|-------|--------|
    /// | 0xF1 | 2 | 5 (+1 if page crossed) |
    SbcIDY = 0xF1,
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
            // * [INC]
            x if x == Self::IncZPG as Byte => Ok(Self::IncZPG),
            x if x == Self::IncZPX as Byte => Ok(Self::IncZPX),
            x if x == Self::IncABS as Byte => Ok(Self::IncABS),
            x if x == Self::IncABX as Byte => Ok(Self::IncABX),
            // * [INX]
            x if x == Self::InxIMP as Byte => Ok(Self::InxIMP),
            // * [INY]
            x if x == Self::InyIMP as Byte => Ok(Self::InyIMP),
            // * [DEC]
            x if x == Self::DecZPG as Byte => Ok(Self::DecZPG),
            x if x == Self::DecZPX as Byte => Ok(Self::DecZPX),
            x if x == Self::DecABS as Byte => Ok(Self::DecABS),
            x if x == Self::DecABX as Byte => Ok(Self::DecABX),
            // * [DEX]
            x if x == Self::DexIMP as Byte => Ok(Self::DexIMP),
            // * [DEY]
            x if x == Self::DeyIMP as Byte => Ok(Self::DeyIMP),
            // * [BEQ]
            x if x == Self::BeqREL as Byte => Ok(Self::BeqREL),
            // * [BNE]
            x if x == Self::BneREL as Byte => Ok(Self::BneREL),
            // * [BCC]
            x if x == Self::BccREL as Byte => Ok(Self::BccREL),
            // * [BCS]
            x if x == Self::BcsREL as Byte => Ok(Self::BcsREL),
            // * [BMI]
            x if x == Self::BmiREL as Byte => Ok(Self::BmiREL),
            // * [BPL]
            x if x == Self::BplREL as Byte => Ok(Self::BplREL),
            // * [BVS]
            x if x == Self::BvsREL as Byte => Ok(Self::BvsREL),
            // * [BVC]
            x if x == Self::BvcREL as Byte => Ok(Self::BvcREL),
            // * [CLC]
            x if x == Self::ClcIMP as Byte => Ok(Self::ClcIMP),
            // * [CLD]
            x if x == Self::CldIMP as Byte => Ok(Self::CldIMP),
            // * [CLI]
            x if x == Self::CliIMP as Byte => Ok(Self::CliIMP),
            // * [CLV]
            x if x == Self::ClvIMP as Byte => Ok(Self::ClvIMP),
            // * [SEC]
            x if x == Self::SecIMP as Byte => Ok(Self::SecIMP),
            // * [SED]
            x if x == Self::SedIMP as Byte => Ok(Self::SedIMP),
            // * [SEI]
            x if x == Self::SeiIMP as Byte => Ok(Self::SeiIMP),
            // * [ADC]
            x if x == Self::AdcIMM as Byte => Ok(Self::AdcIMM),
            x if x == Self::AdcZPG as Byte => Ok(Self::AdcZPG),
            x if x == Self::AdcZPX as Byte => Ok(Self::AdcZPX),
            x if x == Self::AdcABS as Byte => Ok(Self::AdcABS),
            x if x == Self::AdcABX as Byte => Ok(Self::AdcABX),
            x if x == Self::AdcABY as Byte => Ok(Self::AdcABY),
            x if x == Self::AdcIDX as Byte => Ok(Self::AdcIDX),
            x if x == Self::AdcIDY as Byte => Ok(Self::AdcIDY),
            // * [SBC]
            x if x == Self::SbcIMM as Byte => Ok(Self::SbcIMM),
            x if x == Self::SbcZPG as Byte => Ok(Self::SbcZPG),
            x if x == Self::SbcZPX as Byte => Ok(Self::SbcZPX),
            x if x == Self::SbcABS as Byte => Ok(Self::SbcABS),
            x if x == Self::SbcABX as Byte => Ok(Self::SbcABX),
            x if x == Self::SbcABY as Byte => Ok(Self::SbcABY),
            x if x == Self::SbcIDX as Byte => Ok(Self::SbcIDX),
            x if x == Self::SbcIDY as Byte => Ok(Self::SbcIDY),
            _ => Err("unknown CPU instruction"),
        }
    }
}

impl From<Opcode> for Byte {
    fn from(val: Opcode) -> Self {
        val as Byte
    }
}
