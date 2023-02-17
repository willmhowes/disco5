use crate::computer::{Instruction, AddressingMode};

pub fn map_byte_to_instruction(byte: u8) -> Instruction {
    match byte {
        0x6d => Instruction::ADC(AddressingMode::Absolute),
        0x7d => Instruction::ADC(AddressingMode::AbsoluteX),
        0x79 => Instruction::ADC(AddressingMode::AbsoluteY),
        0x69 => Instruction::ADC(AddressingMode::Immediate),
        0x61 => Instruction::ADC(AddressingMode::IndirectX),
        0x71 => Instruction::ADC(AddressingMode::IndirectY),
        0x65 => Instruction::ADC(AddressingMode::ZeroPage),
        0x75 => Instruction::ADC(AddressingMode::ZeroPageX),

        0x2d => Instruction::AND(AddressingMode::Absolute),
        0x3d => Instruction::AND(AddressingMode::AbsoluteX),
        0x39 => Instruction::AND(AddressingMode::AbsoluteY),
        0x29 => Instruction::AND(AddressingMode::Immediate),
        0x21 => Instruction::AND(AddressingMode::IndirectX),
        0x31 => Instruction::AND(AddressingMode::IndirectY),
        0x25 => Instruction::AND(AddressingMode::ZeroPage),
        0x35 => Instruction::AND(AddressingMode::ZeroPageX),

        0x0a => Instruction::ASL(AddressingMode::Accumulator),
        0x0e => Instruction::ASL(AddressingMode::Absolute),
        0x1e => Instruction::ASL(AddressingMode::AbsoluteX),
        0x06 => Instruction::ASL(AddressingMode::ZeroPage),
        0x16 => Instruction::ASL(AddressingMode::ZeroPageX),

        0x90 => Instruction::BCC(AddressingMode::Relative),

        0xb0 => Instruction::BCS(AddressingMode::Relative),

        0xf0 => Instruction::BEQ(AddressingMode::Relative),

        0x2c => Instruction::BIT(AddressingMode::Absolute),
        0x24 => Instruction::BIT(AddressingMode::ZeroPage),

        0x30 => Instruction::BMI(AddressingMode::Relative),

        0xd0 => Instruction::BNE(AddressingMode::Relative),

        0x10 => Instruction::BPL(AddressingMode::Relative),

        0x00 => Instruction::BRK(AddressingMode::Implied),

        0x50 => Instruction::BVC(AddressingMode::Relative),

        0x70 => Instruction::BVS(AddressingMode::Relative),

        0x18 => Instruction::CLC(AddressingMode::Implied),

        0xd8 => Instruction::CLD(AddressingMode::Implied),

        0x58 => Instruction::CLI(AddressingMode::Implied),

        0xb8 => Instruction::CLV(AddressingMode::Implied),

        0xcd => Instruction::CMP(AddressingMode::Absolute),
        0xdd => Instruction::CMP(AddressingMode::AbsoluteX),
        0xd9 => Instruction::CMP(AddressingMode::AbsoluteY),
        0xc9 => Instruction::CMP(AddressingMode::Immediate),
        0xc1 => Instruction::CMP(AddressingMode::IndirectX),
        0xd1 => Instruction::CMP(AddressingMode::IndirectY),
        0xc5 => Instruction::CMP(AddressingMode::ZeroPage),
        0xd5 => Instruction::CMP(AddressingMode::ZeroPageX),

        0xec => Instruction::CPX(AddressingMode::Absolute),
        0xe0 => Instruction::CPX(AddressingMode::Immediate),
        0xe4 => Instruction::CPX(AddressingMode::ZeroPage),

        0xcc => Instruction::CPY(AddressingMode::Absolute),
        0xc0 => Instruction::CPY(AddressingMode::Immediate),
        0xc4 => Instruction::CPY(AddressingMode::ZeroPage),

        0xce => Instruction::DEC(AddressingMode::Absolute),
        0xde => Instruction::DEC(AddressingMode::AbsoluteX),
        0xc6 => Instruction::DEC(AddressingMode::ZeroPage),
        0xd6 => Instruction::DEC(AddressingMode::ZeroPageX),

        0xca => Instruction::DEX(AddressingMode::Implied),

        0x88 => Instruction::DEY(AddressingMode::Implied),

        0x4d => Instruction::EOR(AddressingMode::Absolute),
        0x5d => Instruction::EOR(AddressingMode::AbsoluteX),
        0x59 => Instruction::EOR(AddressingMode::AbsoluteY),
        0x49 => Instruction::EOR(AddressingMode::Immediate),
        0x41 => Instruction::EOR(AddressingMode::IndirectX),
        0x51 => Instruction::EOR(AddressingMode::IndirectY),
        0x45 => Instruction::EOR(AddressingMode::ZeroPage),
        0x55 => Instruction::EOR(AddressingMode::ZeroPageX),

        0xee => Instruction::INC(AddressingMode::Absolute),
        0xfe => Instruction::INC(AddressingMode::AbsoluteX),
        0xe6 => Instruction::INC(AddressingMode::ZeroPage),
        0xf6 => Instruction::INC(AddressingMode::ZeroPageX),

        0xe8 => Instruction::INX(AddressingMode::Implied),

        0xc8 => Instruction::INY(AddressingMode::Implied),

        0x4c => Instruction::JMP(AddressingMode::Absolute),
        0x6c => Instruction::JMP(AddressingMode::Indirect),

        0x20 => Instruction::JSR(AddressingMode::Absolute),

        0xad => Instruction::LDA(AddressingMode::Absolute),
        0xbd => Instruction::LDA(AddressingMode::AbsoluteX),
        0xb9 => Instruction::LDA(AddressingMode::AbsoluteY),
        0xa9 => Instruction::LDA(AddressingMode::Immediate),
        0xa1 => Instruction::LDA(AddressingMode::IndirectX),
        0xb1 => Instruction::LDA(AddressingMode::IndirectY),
        0xa5 => Instruction::LDA(AddressingMode::ZeroPage),
        0xb5 => Instruction::LDA(AddressingMode::ZeroPageX),

        0xae => Instruction::LDX(AddressingMode::Absolute),
        0xbe => Instruction::LDX(AddressingMode::AbsoluteY),
        0xa2 => Instruction::LDX(AddressingMode::Immediate),
        0xa6 => Instruction::LDX(AddressingMode::ZeroPage),
        0xb6 => Instruction::LDX(AddressingMode::ZeroPageY),

        0xac => Instruction::LDY(AddressingMode::Absolute),
        0xbc => Instruction::LDY(AddressingMode::AbsoluteX),
        0xa0 => Instruction::LDY(AddressingMode::Immediate),
        0xa4 => Instruction::LDY(AddressingMode::ZeroPage),
        0xb4 => Instruction::LDY(AddressingMode::ZeroPageX),

        0x4e => Instruction::LSR(AddressingMode::Absolute),
        0x5e => Instruction::LSR(AddressingMode::AbsoluteX),
        0x4a => Instruction::LSR(AddressingMode::Accumulator),
        0x46 => Instruction::LSR(AddressingMode::ZeroPage),
        0x56 => Instruction::LSR(AddressingMode::ZeroPageX),

        0xea => Instruction::NOP(AddressingMode::Implied),

        0x0d => Instruction::ORA(AddressingMode::Absolute),
        0x1d => Instruction::ORA(AddressingMode::AbsoluteX),
        0x19 => Instruction::ORA(AddressingMode::AbsoluteY),
        0x09 => Instruction::ORA(AddressingMode::Immediate),
        0x01 => Instruction::ORA(AddressingMode::IndirectX),
        0x11 => Instruction::ORA(AddressingMode::IndirectY),
        0x05 => Instruction::ORA(AddressingMode::ZeroPage),
        0x15 => Instruction::ORA(AddressingMode::ZeroPageX),

        0x48 => Instruction::PHA(AddressingMode::Implied),

        0x08 => Instruction::PHP(AddressingMode::Implied),

        0x68 => Instruction::PLA(AddressingMode::Implied),

        0x28 => Instruction::PLP(AddressingMode::Implied),

        0x2e => Instruction::ROL(AddressingMode::Absolute),
        0x3e => Instruction::ROL(AddressingMode::AbsoluteX),
        0x2a => Instruction::ROL(AddressingMode::Accumulator),
        0x26 => Instruction::ROL(AddressingMode::ZeroPage),
        0x36 => Instruction::ROL(AddressingMode::ZeroPageX),

        0x6e => Instruction::ROR(AddressingMode::Absolute),
        0x7e => Instruction::ROR(AddressingMode::AbsoluteX),
        0x6a => Instruction::ROR(AddressingMode::Accumulator),
        0x66 => Instruction::ROR(AddressingMode::ZeroPage),
        0x76 => Instruction::ROR(AddressingMode::ZeroPageX),

        0x40 => Instruction::RTI(AddressingMode::Implied),

        0x60 => Instruction::RTS(AddressingMode::Implied),

        0xed => Instruction::SBC(AddressingMode::Absolute),
        0xfd => Instruction::SBC(AddressingMode::AbsoluteX),
        0xf9 => Instruction::SBC(AddressingMode::AbsoluteY),
        0xe9 => Instruction::SBC(AddressingMode::Immediate),
        0xe1 => Instruction::SBC(AddressingMode::IndirectX),
        0xf1 => Instruction::SBC(AddressingMode::IndirectY),
        0xe5 => Instruction::SBC(AddressingMode::ZeroPage),
        0xf5 => Instruction::SBC(AddressingMode::ZeroPageX),

        0x38 => Instruction::SEC(AddressingMode::Implied),

        0xf8 => Instruction::SED(AddressingMode::Implied),

        0x78 => Instruction::SEI(AddressingMode::Implied),

        0x8d => Instruction::STA(AddressingMode::Absolute),
        0x9d => Instruction::STA(AddressingMode::AbsoluteX),
        0x99 => Instruction::STA(AddressingMode::AbsoluteY),
        0x81 => Instruction::STA(AddressingMode::IndirectX),
        0x91 => Instruction::STA(AddressingMode::IndirectY),
        0x85 => Instruction::STA(AddressingMode::ZeroPage),
        0x95 => Instruction::STA(AddressingMode::ZeroPageX),

        0x8e => Instruction::STX(AddressingMode::Absolute),
        0x86 => Instruction::STX(AddressingMode::ZeroPage),
        0x96 => Instruction::STX(AddressingMode::ZeroPageY),

        0x8c => Instruction::STY(AddressingMode::Absolute),
        0x84 => Instruction::STY(AddressingMode::ZeroPage),
        0x94 => Instruction::STY(AddressingMode::ZeroPageX),

        0xaa => Instruction::TAX(AddressingMode::Implied),

        0xa8 => Instruction::TAY(AddressingMode::Implied),

        0xba => Instruction::TSX(AddressingMode::Implied),

        0x8a => Instruction::TXA(AddressingMode::Implied),

        0x9a => Instruction::TXS(AddressingMode::Implied),

        0x98 => Instruction::TYA(AddressingMode::Implied),

        _ => Instruction::Invalid,
    }
}
