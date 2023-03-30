#[derive(Debug, Copy, Clone)]
pub enum AddressingMode {
    Accumulator,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    Implied,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
}

#[derive(Debug)]
pub enum Instruction {
    /// add with carry
    ADC(AddressingMode),
    /// and (with accumulator)
    AND(AddressingMode),
    /// arithmetic shift left
    ASL(AddressingMode),
    /// branch on carry clear
    BCC(AddressingMode),
    /// branch on carry set
    BCS(AddressingMode),
    /// branch on equal (zero set)
    BEQ(AddressingMode),
    /// bit test
    BIT(AddressingMode),
    /// branch on minus (negative set)
    BMI(AddressingMode),
    /// branch on not equal (zero clear)
    BNE(AddressingMode),
    /// branch on plus (negative clear)
    BPL(AddressingMode),
    /// break / interrupt
    BRK(AddressingMode),
    /// branch on overflow clear
    BVC(AddressingMode),
    /// branch on overflow set
    BVS(AddressingMode),
    /// clear carry
    CLC(AddressingMode),
    /// clear decimal
    CLD(AddressingMode),
    /// clear interrupt disable
    CLI(AddressingMode),
    /// clear overflow
    CLV(AddressingMode),
    /// compare (with accumulator)
    CMP(AddressingMode),
    /// compare with X
    CPX(AddressingMode),
    /// compare with Y
    CPY(AddressingMode),
    /// decrement
    DEC(AddressingMode),
    /// decrement X
    DEX(AddressingMode),
    /// decrement Y
    DEY(AddressingMode),
    /// exclusive or (with accumulator)
    EOR(AddressingMode),
    /// increment
    INC(AddressingMode),
    /// increment X
    INX(AddressingMode),
    /// increment Y
    INY(AddressingMode),
    /// jump
    JMP(AddressingMode),
    /// jump subroutine
    JSR(AddressingMode),
    /// load accumulator
    LDA(AddressingMode),
    /// load X
    LDX(AddressingMode),
    /// load Y
    LDY(AddressingMode),
    /// logical shift right
    LSR(AddressingMode),
    /// no operation
    NOP(AddressingMode),
    /// or with accumulator
    ORA(AddressingMode),
    /// push accumulator
    PHA(AddressingMode),
    /// push processor status (SR)
    PHP(AddressingMode),
    /// pull accumulator
    PLA(AddressingMode),
    /// pull processor status (SR)
    PLP(AddressingMode),
    /// rotate left
    ROL(AddressingMode),
    /// rotate right
    ROR(AddressingMode),
    /// return from interrupt
    RTI(AddressingMode),
    /// return from subroutine
    RTS(AddressingMode),
    /// subtract with carry
    SBC(AddressingMode),
    /// set carry
    SEC(AddressingMode),
    /// set decimal
    SED(AddressingMode),
    /// set interrupt disable
    SEI(AddressingMode),
    /// store accumulator
    STA(AddressingMode),
    /// store X
    STX(AddressingMode),
    /// store Y
    STY(AddressingMode),
    /// transfer accumulator to X
    TAX(AddressingMode),
    /// transfer accumulator to Y
    TAY(AddressingMode),
    /// transfer stack pointer to X
    TSX(AddressingMode),
    /// transfer X to accumulator
    TXA(AddressingMode),
    /// transfer X to stack pointer
    TXS(AddressingMode),
    /// transfer Y to accumulator
    TYA(AddressingMode),
    Invalid(u8),
}

pub fn map_byte_to_instruction(byte: u8) -> (Instruction, u8) {
    match byte {
        0x6d => (Instruction::ADC(AddressingMode::Absolute), 4),
        0x7d => (Instruction::ADC(AddressingMode::AbsoluteX), 4),
        0x79 => (Instruction::ADC(AddressingMode::AbsoluteY), 4),
        0x69 => (Instruction::ADC(AddressingMode::Immediate), 2),
        0x61 => (Instruction::ADC(AddressingMode::IndirectX), 6),
        0x71 => (Instruction::ADC(AddressingMode::IndirectY), 5),
        0x65 => (Instruction::ADC(AddressingMode::ZeroPage), 3),
        0x75 => (Instruction::ADC(AddressingMode::ZeroPageX), 4),

        0x2d => (Instruction::AND(AddressingMode::Absolute), 4),
        0x3d => (Instruction::AND(AddressingMode::AbsoluteX), 4),
        0x39 => (Instruction::AND(AddressingMode::AbsoluteY), 4),
        0x29 => (Instruction::AND(AddressingMode::Immediate), 2),
        0x21 => (Instruction::AND(AddressingMode::IndirectX), 6),
        0x31 => (Instruction::AND(AddressingMode::IndirectY), 5),
        0x25 => (Instruction::AND(AddressingMode::ZeroPage), 3),
        0x35 => (Instruction::AND(AddressingMode::ZeroPageX), 4),

        0x0a => (Instruction::ASL(AddressingMode::Accumulator), 2),
        0x0e => (Instruction::ASL(AddressingMode::Absolute), 6),
        0x1e => (Instruction::ASL(AddressingMode::AbsoluteX), 7),
        0x06 => (Instruction::ASL(AddressingMode::ZeroPage), 5),
        0x16 => (Instruction::ASL(AddressingMode::ZeroPageX), 6),

        0x90 => (Instruction::BCC(AddressingMode::Relative), 2),

        0xb0 => (Instruction::BCS(AddressingMode::Relative), 2),

        0xf0 => (Instruction::BEQ(AddressingMode::Relative), 2),

        0x2c => (Instruction::BIT(AddressingMode::Absolute), 4),
        0x24 => (Instruction::BIT(AddressingMode::ZeroPage), 3),

        0x30 => (Instruction::BMI(AddressingMode::Relative), 2),

        0xd0 => (Instruction::BNE(AddressingMode::Relative), 2),

        0x10 => (Instruction::BPL(AddressingMode::Relative), 2),

        0x00 => (Instruction::BRK(AddressingMode::Implied), 7),

        0x50 => (Instruction::BVC(AddressingMode::Relative), 2),

        0x70 => (Instruction::BVS(AddressingMode::Relative), 2),

        0x18 => (Instruction::CLC(AddressingMode::Implied), 2),

        0xd8 => (Instruction::CLD(AddressingMode::Implied), 2),

        0x58 => (Instruction::CLI(AddressingMode::Implied), 2),

        0xb8 => (Instruction::CLV(AddressingMode::Implied), 2),

        0xcd => (Instruction::CMP(AddressingMode::Absolute), 4),
        0xdd => (Instruction::CMP(AddressingMode::AbsoluteX), 4),
        0xd9 => (Instruction::CMP(AddressingMode::AbsoluteY), 4),
        0xc9 => (Instruction::CMP(AddressingMode::Immediate), 2),
        0xc1 => (Instruction::CMP(AddressingMode::IndirectX), 6),
        0xd1 => (Instruction::CMP(AddressingMode::IndirectY), 5),
        0xc5 => (Instruction::CMP(AddressingMode::ZeroPage), 3),
        0xd5 => (Instruction::CMP(AddressingMode::ZeroPageX), 4),

        0xec => (Instruction::CPX(AddressingMode::Absolute), 4),
        0xe0 => (Instruction::CPX(AddressingMode::Immediate), 2),
        0xe4 => (Instruction::CPX(AddressingMode::ZeroPage), 3),

        0xcc => (Instruction::CPY(AddressingMode::Absolute), 4),
        0xc0 => (Instruction::CPY(AddressingMode::Immediate), 2),
        0xc4 => (Instruction::CPY(AddressingMode::ZeroPage), 3),

        0xce => (Instruction::DEC(AddressingMode::Absolute), 6),
        0xde => (Instruction::DEC(AddressingMode::AbsoluteX), 7),
        0xc6 => (Instruction::DEC(AddressingMode::ZeroPage), 5),
        0xd6 => (Instruction::DEC(AddressingMode::ZeroPageX), 6),

        0xca => (Instruction::DEX(AddressingMode::Implied), 2),

        0x88 => (Instruction::DEY(AddressingMode::Implied), 2),

        0x4d => (Instruction::EOR(AddressingMode::Absolute), 4),
        0x5d => (Instruction::EOR(AddressingMode::AbsoluteX), 4),
        0x59 => (Instruction::EOR(AddressingMode::AbsoluteY), 4),
        0x49 => (Instruction::EOR(AddressingMode::Immediate), 2),
        0x41 => (Instruction::EOR(AddressingMode::IndirectX), 6),
        0x51 => (Instruction::EOR(AddressingMode::IndirectY), 5),
        0x45 => (Instruction::EOR(AddressingMode::ZeroPage), 3),
        0x55 => (Instruction::EOR(AddressingMode::ZeroPageX), 4),

        0xee => (Instruction::INC(AddressingMode::Absolute), 6),
        0xfe => (Instruction::INC(AddressingMode::AbsoluteX), 7),
        0xe6 => (Instruction::INC(AddressingMode::ZeroPage), 5),
        0xf6 => (Instruction::INC(AddressingMode::ZeroPageX), 6),

        0xe8 => (Instruction::INX(AddressingMode::Implied), 2),

        0xc8 => (Instruction::INY(AddressingMode::Implied), 2),

        0x4c => (Instruction::JMP(AddressingMode::Absolute), 3),
        0x6c => (Instruction::JMP(AddressingMode::Indirect), 5),

        0x20 => (Instruction::JSR(AddressingMode::Absolute), 6),

        0xad => (Instruction::LDA(AddressingMode::Absolute), 4),
        0xbd => (Instruction::LDA(AddressingMode::AbsoluteX), 4),
        0xb9 => (Instruction::LDA(AddressingMode::AbsoluteY), 4),
        0xa9 => (Instruction::LDA(AddressingMode::Immediate), 2),
        0xa1 => (Instruction::LDA(AddressingMode::IndirectX), 6),
        0xb1 => (Instruction::LDA(AddressingMode::IndirectY), 5),
        0xa5 => (Instruction::LDA(AddressingMode::ZeroPage), 3),
        0xb5 => (Instruction::LDA(AddressingMode::ZeroPageX), 4),

        0xae => (Instruction::LDX(AddressingMode::Absolute), 4),
        0xbe => (Instruction::LDX(AddressingMode::AbsoluteY), 4),
        0xa2 => (Instruction::LDX(AddressingMode::Immediate), 2),
        0xa6 => (Instruction::LDX(AddressingMode::ZeroPage), 3),
        0xb6 => (Instruction::LDX(AddressingMode::ZeroPageY), 4),

        0xac => (Instruction::LDY(AddressingMode::Absolute), 4),
        0xbc => (Instruction::LDY(AddressingMode::AbsoluteX), 4),
        0xa0 => (Instruction::LDY(AddressingMode::Immediate), 2),
        0xa4 => (Instruction::LDY(AddressingMode::ZeroPage), 3),
        0xb4 => (Instruction::LDY(AddressingMode::ZeroPageX), 4),

        0x4e => (Instruction::LSR(AddressingMode::Absolute), 6),
        0x5e => (Instruction::LSR(AddressingMode::AbsoluteX), 7),
        0x4a => (Instruction::LSR(AddressingMode::Accumulator), 2),
        0x46 => (Instruction::LSR(AddressingMode::ZeroPage), 5),
        0x56 => (Instruction::LSR(AddressingMode::ZeroPageX), 6),

        0xea => (Instruction::NOP(AddressingMode::Implied), 2),

        0x0d => (Instruction::ORA(AddressingMode::Absolute), 4),
        0x1d => (Instruction::ORA(AddressingMode::AbsoluteX), 4),
        0x19 => (Instruction::ORA(AddressingMode::AbsoluteY), 4),
        0x09 => (Instruction::ORA(AddressingMode::Immediate), 2),
        0x01 => (Instruction::ORA(AddressingMode::IndirectX), 6),
        0x11 => (Instruction::ORA(AddressingMode::IndirectY), 5),
        0x05 => (Instruction::ORA(AddressingMode::ZeroPage), 3),
        0x15 => (Instruction::ORA(AddressingMode::ZeroPageX), 4),

        0x48 => (Instruction::PHA(AddressingMode::Implied), 3),

        0x08 => (Instruction::PHP(AddressingMode::Implied), 3),

        0x68 => (Instruction::PLA(AddressingMode::Implied), 4),

        0x28 => (Instruction::PLP(AddressingMode::Implied), 4),

        0x2e => (Instruction::ROL(AddressingMode::Absolute), 6),
        0x3e => (Instruction::ROL(AddressingMode::AbsoluteX), 7),
        0x2a => (Instruction::ROL(AddressingMode::Accumulator), 2),
        0x26 => (Instruction::ROL(AddressingMode::ZeroPage), 5),
        0x36 => (Instruction::ROL(AddressingMode::ZeroPageX), 6),

        0x6e => (Instruction::ROR(AddressingMode::Absolute), 6),
        0x7e => (Instruction::ROR(AddressingMode::AbsoluteX), 7),
        0x6a => (Instruction::ROR(AddressingMode::Accumulator), 2),
        0x66 => (Instruction::ROR(AddressingMode::ZeroPage), 5),
        0x76 => (Instruction::ROR(AddressingMode::ZeroPageX), 6),

        0x40 => (Instruction::RTI(AddressingMode::Implied), 6),

        0x60 => (Instruction::RTS(AddressingMode::Implied), 6),

        0xed => (Instruction::SBC(AddressingMode::Absolute), 4),
        0xfd => (Instruction::SBC(AddressingMode::AbsoluteX), 4),
        0xf9 => (Instruction::SBC(AddressingMode::AbsoluteY), 4),
        0xe9 => (Instruction::SBC(AddressingMode::Immediate), 2),
        0xe1 => (Instruction::SBC(AddressingMode::IndirectX), 6),
        0xf1 => (Instruction::SBC(AddressingMode::IndirectY), 5),
        0xe5 => (Instruction::SBC(AddressingMode::ZeroPage), 3),
        0xf5 => (Instruction::SBC(AddressingMode::ZeroPageX), 4),

        0x38 => (Instruction::SEC(AddressingMode::Implied), 2),

        0xf8 => (Instruction::SED(AddressingMode::Implied), 2),

        0x78 => (Instruction::SEI(AddressingMode::Implied), 2),

        0x8d => (Instruction::STA(AddressingMode::Absolute), 4),
        0x9d => (Instruction::STA(AddressingMode::AbsoluteX), 5),
        0x99 => (Instruction::STA(AddressingMode::AbsoluteY), 5),
        0x81 => (Instruction::STA(AddressingMode::IndirectX), 6),
        0x91 => (Instruction::STA(AddressingMode::IndirectY), 6),
        0x85 => (Instruction::STA(AddressingMode::ZeroPage), 3),
        0x95 => (Instruction::STA(AddressingMode::ZeroPageX), 4),

        0x8e => (Instruction::STX(AddressingMode::Absolute), 4),
        0x86 => (Instruction::STX(AddressingMode::ZeroPage), 3),
        0x96 => (Instruction::STX(AddressingMode::ZeroPageY), 4),

        0x8c => (Instruction::STY(AddressingMode::Absolute), 4),
        0x84 => (Instruction::STY(AddressingMode::ZeroPage), 3),
        0x94 => (Instruction::STY(AddressingMode::ZeroPageX), 4),

        0xaa => (Instruction::TAX(AddressingMode::Implied), 2),

        0xa8 => (Instruction::TAY(AddressingMode::Implied), 2),

        0xba => (Instruction::TSX(AddressingMode::Implied), 2),

        0x8a => (Instruction::TXA(AddressingMode::Implied), 2),

        0x9a => (Instruction::TXS(AddressingMode::Implied), 2),

        0x98 => (Instruction::TYA(AddressingMode::Implied), 2),

        _ => (Instruction::Invalid(byte), 0),
    }
}
