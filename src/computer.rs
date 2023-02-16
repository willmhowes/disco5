#[allow(non_camel_case_types)]
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::ops::Add;
use strum_macros::FromRepr;

pub mod cpu;

use crate::computer::cpu::*;

const MEMORY_SIZE: usize = 0xffff;

#[derive(Debug, Copy, Clone)]
enum AddressingMode {
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

#[derive(Debug, Default)]
enum Instruction {
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
    #[default]
    Invalid,
}

/// loads instruction at address of pc, increments pc
fn fetch_instruction(memory: &[u8], cpu: &mut CPU) -> u8 {
    let index = cpu.pc;
    cpu.step();
    memory[index as usize]
}

#[derive(Debug)]
pub struct Computer {
    pub cpu: CPU,
    pub memory: [u8; MEMORY_SIZE],
    pub flags: StatusRegister,
}

impl Default for Computer {
    fn default() -> Computer {
        Computer {
            cpu: CPU {
                ..Default::default()
            },
            memory: [0; MEMORY_SIZE],
            flags: Default::default(),
        }
    }
}

impl Computer {
    pub fn load_program(&mut self, filename: &str) -> io::Result<()> {
        let memory = &mut self.memory;
        let cpu = &mut self.cpu;
        // Load file contents into a buffer
        let f = File::open(filename)?;
        let f = BufReader::new(f);

        // Iterate through each line in file
        // Currently only supports one line
        for line in f.lines() {
            let line = line?;
            let hexdump: Vec<&str> = line.split(' ').collect();

            // Identify location of code in memory
            let loc_length = hexdump[0].chars().count();
            let loc = &hexdump[0][0..loc_length - 1];
            let mut loc: u16 = loc.parse().unwrap();

            if cpu.pc == 0 {
                cpu.pc = loc;
            };

            // Write instructions to memory
            println!("LINE : {}", cpu.pc);
            for hex in &hexdump[1..] {
                memory[usize::from(loc)] = u8::from_str_radix(hex, 16).unwrap();
                loc += 1;
            }
        }

        Ok(())
    }

    pub fn run_program(&mut self) {
        while usize::from(self.cpu.pc) < self.memory.len() {
            let instruction = fetch_instruction(&self.memory, &mut self.cpu);
            let instruction = usize::from(instruction);
            let instruction = match instruction {
                0x6d => Instruction::ADC(AddressingMode::Absolute),
                0x7d => Instruction::ADC(AddressingMode::AbsoluteX),
                0x79 => Instruction::ADC(AddressingMode::AbsoluteY),
                0x69 => Instruction::ADC(AddressingMode::Immediate),
                0x61 => Instruction::ADC(AddressingMode::IndirectX),
                0x71 => Instruction::ADC(AddressingMode::IndirectY),
                0x65 => Instruction::ADC(AddressingMode::ZeroPage),
                0x75 => Instruction::ADC(AddressingMode::ZeroPageX),

                0xe9 => Instruction::SBC(AddressingMode::Immediate),

                0xa2 => Instruction::LDX(AddressingMode::Immediate),

                0xa0 => Instruction::LDY(AddressingMode::Immediate),

                0x94 => Instruction::STY(AddressingMode::ZeroPageX),

                0xe8 => Instruction::INX(AddressingMode::Implied),

                0x88 => Instruction::DEY(AddressingMode::Implied),

                0xc0 => Instruction::CPY(AddressingMode::Immediate),

                0xd0 => Instruction::BNE(AddressingMode::Relative),

                _ => Instruction::Invalid,
            };
            self.process_instruction(instruction);
        }
    }

    fn set_status_nz(&mut self, test_val: u8) {
        self.cpu.status.z = if test_val == 0 { true } else { false };
        // 0x80 = 0b1000_0000 (i.e. a negative number under two-complement encoding)
        self.cpu.status.n = if test_val & 0x80 == 0x80 { true } else { false };
    }

    fn resolve_addressing_mode(&mut self, am: AddressingMode) -> Option<u16> {
        match am {
            AddressingMode::Accumulator | AddressingMode::Implied => None,
            AddressingMode::Absolute => {
                let lo = fetch_instruction(&self.memory, &mut self.cpu);
                let hi = fetch_instruction(&self.memory, &mut self.cpu);
                Some((u16::from(hi) << 2) + u16::from(lo))
            }
            AddressingMode::AbsoluteX => todo!(),
            AddressingMode::AbsoluteY => todo!(),
            AddressingMode::Immediate => {
                Some(u16::from(fetch_instruction(&self.memory, &mut self.cpu)))
            }
            AddressingMode::Indirect => todo!(),
            AddressingMode::IndirectX => todo!(),
            AddressingMode::IndirectY => todo!(),
            AddressingMode::Relative => todo!(),
            AddressingMode::ZeroPage => todo!(),
            AddressingMode::ZeroPageX => todo!(),
            AddressingMode::ZeroPageY => todo!(),
        }
    }

    fn adc_logic(&mut self, addend_1: u8) {
        let addend_2 = self.cpu.a;
        let carry = if self.cpu.status.c == true { 1 } else { 0 };
        let result = addend_1.wrapping_add(addend_2).wrapping_add(carry);
        self.cpu.a = result;
        self.cpu.status.c = if addend_1 >= result { true } else { false };
        self.cpu.status.v = if (addend_1 ^ result) & (addend_2 ^ result) & 0x80 == 0x00 {
            false
        } else {
            true
        };
        self.set_status_nz(self.cpu.a);
    }

    fn process_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADC(am) => {
                let address = self.resolve_addressing_mode(am);
                let address = address.unwrap();
                if let AddressingMode::Immediate = am {
                    self.adc_logic(address as u8);
                } else {
                    let addend = self.memory[usize::from(address)];
                    self.adc_logic(addend);
                }
            }
            Instruction::SBC(am) => {
                let address = self.resolve_addressing_mode(am);
                let address = address.unwrap();
                if let AddressingMode::Immediate = am {
                    self.adc_logic(!(address as u8));
                } else {
                    let complement = !self.memory[usize::from(address)];
                    self.adc_logic(complement);
                }
            }
            // Opcode::BNE(am) => Instruction::BNE(AddressingMode::Relative(
            //     fetch_instruction(&self.memory, &mut self.cpu),
            // )),
            // Opcode::CPY(am) => Instruction::CPY(AddressingMode::Immediate(
            //     fetch_instruction(&self.memory, &mut self.cpu),
            // )),
            // Opcode::DEY(am) => Instruction::DEY(AddressingMode::Implied),
            // Opcode::LDX(am) => Instruction::LDX(AddressingMode::Immediate(
            //     fetch_instruction(&self.memory, &mut self.cpu),
            // )),
            // Opcode::LDY(am) => Instruction::LDY(AddressingMode::Immediate(
            //     fetch_instruction(&self.memory, &mut self.cpu),
            // )),
            // Opcode::INXi(am) => Instruction::INX(AddressingMode::Implied),
            // Opcode::SBC(am) => Instruction::SBC(AddressingMode::Immediate(
            //     fetch_instruction(&self.memory, &mut self.cpu),
            // )),
            // Opcode::STY(am) => Instruction::STY(AddressingMode::ZeroPageX(
            //     fetch_instruction(&self.memory, &mut self.cpu),
            // )),
            Instruction::Invalid => (),
            _ => todo!(),
        }
    }

    // fn process_instruction_old(&mut self, instruction: Instruction) {
    //         match instruction {
    //             Instruction::ADC(addend_1) => {
    //             }
    //             Instruction::SBC(addend_1) => {
    //                 self.process_instruction(Instruction::ADC(!addend_1));
    //             }
    //             Instruction::LDX => {
    //                 self.cpu.x = fetch_instruction(&self.memory, &mut self.cpu);
    //                 self.set_status_nz(self.cpu.x);
    //             }
    //             Instruction::LDY => {
    //                 self.cpu.y = fetch_instruction(&self.memory, &mut self.cpu);
    //                 self.set_status_nz(self.cpu.y);
    //             }
    //             Instruction::STY => {
    //                 let zpg = fetch_instruction(&self.memory, &mut self.cpu);
    //                 let index = zpg.wrapping_add(self.cpu.x);
    //                 let index: usize = usize::from(index);
    //                 self.memory[index] = self.cpu.y;
    //             }
    //             Instruction::INX => {
    //                 self.cpu.x = self.cpu.x.wrapping_add(1);
    //                 self.set_status_nz(self.cpu.x);
    //             }
    //             Instruction::DEY => {
    //                 self.cpu.y = self.cpu.y.wrapping_sub(1);
    //                 self.set_status_nz(self.cpu.y);
    //             }
    //             Instruction::CPY => {
    //                 let test_val = fetch_instruction(&self.memory, &mut self.cpu);
    //                 self.cpu.status.c = if self.cpu.y >= test_val { true } else { false };
    //                 self.set_status_nz(self.cpu.y.wrapping_sub(test_val));
    //             }
    //             Instruction::BNE => {
    //                 let offset: u8 = fetch_instruction(&self.memory, &mut self.cpu);
    //                 let offset: i8 = offset as i8;
    //                 let mut negative = false;
    //                 if offset.is_negative() {
    //                     negative = true;
    //                 }
    //                 let offset = offset.abs();
    //                 let offset = offset as u16;
    //                 if self.cpu.status.z == false && negative == false {
    //                     self.cpu.pc += u16::from(offset);
    //                 } else if self.cpu.status.z == false && negative == true {
    //                     self.cpu.pc -= u16::from(offset);
    //                 }
    //             }
    //             Instruction::Invalid => (),
    //         }
    //     }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_adc_imm() {
        let mut computer: Computer = Default::default();
        computer.memory[0] = 0x0a;
        computer.cpu.a = 0x05;
        computer.cpu.status.c = true;
        computer.process_instruction(Instruction::ADC(AddressingMode::Immediate));
        assert_eq!(computer.cpu.a, 0x10);
        assert_eq!(computer.cpu.status.z, false);
        assert_eq!(computer.cpu.status.n, false);
        assert_eq!(computer.cpu.status.c, false);
        assert_eq!(computer.cpu.status.v, false);

        let mut computer: Computer = Default::default();
        computer.memory[0] = 0xff;
        computer.cpu.a = 0x10;
        computer.process_instruction(Instruction::ADC(AddressingMode::Immediate));
        assert_eq!(computer.cpu.a, 0x0f);
        assert_eq!(computer.cpu.status.z, false);
        assert_eq!(computer.cpu.status.n, false);
        assert_eq!(computer.cpu.status.c, true);
        assert_eq!(computer.cpu.status.v, false);

        let mut computer: Computer = Default::default();
        computer.memory[0] = 0xff;
        computer.cpu.a = 0x01;
        computer.process_instruction(Instruction::ADC(AddressingMode::Immediate));
        assert_eq!(computer.cpu.a, 0x00);
        assert_eq!(computer.cpu.status.z, true);
        assert_eq!(computer.cpu.status.n, false);
        assert_eq!(computer.cpu.status.c, true);
        assert_eq!(computer.cpu.status.v, false);

        let mut computer: Computer = Default::default();
        computer.memory[0] = 0xa0;
        computer.cpu.a = 0x05;
        computer.process_instruction(Instruction::ADC(AddressingMode::Immediate));
        assert_eq!(computer.cpu.a, 0xa5);
        assert_eq!(computer.cpu.status.z, false);
        assert_eq!(computer.cpu.status.n, true);
        assert_eq!(computer.cpu.status.c, false);
        assert_eq!(computer.cpu.status.v, false);

        let mut computer: Computer = Default::default();
        computer.memory[0] = 0x9c;
        computer.cpu.a = 0x9c;
        computer.process_instruction(Instruction::ADC(AddressingMode::Immediate));
        assert_eq!(computer.cpu.a, 0x38);
        assert_eq!(computer.cpu.status.z, false);
        assert_eq!(computer.cpu.status.n, false);
        assert_eq!(computer.cpu.status.c, true);
        assert_eq!(computer.cpu.status.v, true);

        let mut computer: Computer = Default::default();
        computer.memory[0] = 0x50;
        computer.cpu.a = 0x50;
        computer.process_instruction(Instruction::ADC(AddressingMode::Immediate));
        assert_eq!(computer.cpu.a, 0xa0);
        assert_eq!(computer.cpu.status.z, false);
        assert_eq!(computer.cpu.status.n, true);
        assert_eq!(computer.cpu.status.c, false);
        assert_eq!(computer.cpu.status.v, true);
    }

    #[test]
    fn test_sbc_imm() {
        let mut computer: Computer = Default::default();
        computer.memory[0] = 6;
        computer.cpu.a = 10;
        computer.cpu.status.c = true;
        computer.process_instruction(Instruction::SBC(AddressingMode::Immediate));
        assert_eq!(computer.cpu.a, 4);
        assert_eq!(computer.cpu.status.z, false);
        assert_eq!(computer.cpu.status.n, false);
        assert_eq!(computer.cpu.status.c, true);
        assert_eq!(computer.cpu.status.v, false);

        let mut computer: Computer = Default::default();
        computer.memory[0] = 0x10;
        computer.cpu.a = 0xff;
        computer.cpu.status.c = false;
        computer.process_instruction(Instruction::SBC(AddressingMode::Immediate));
        assert_eq!(computer.cpu.a, 0xee);
        assert_eq!(computer.cpu.status.z, false);
        assert_eq!(computer.cpu.status.n, true);
        assert_eq!(computer.cpu.status.c, true);
        assert_eq!(computer.cpu.status.v, false);

        let mut computer: Computer = Default::default();
        computer.memory[0] = 0xff;
        computer.cpu.a = 0xff;
        computer.cpu.status.c = true;
        computer.process_instruction(Instruction::SBC(AddressingMode::Immediate));
        assert_eq!(computer.cpu.a, 0x00);
        assert_eq!(computer.cpu.status.z, true);
        assert_eq!(computer.cpu.status.n, false);
        assert_eq!(computer.cpu.status.c, true);
        assert_eq!(computer.cpu.status.v, false);

        let mut computer: Computer = Default::default();
        computer.memory[0] = 0xb0;
        computer.cpu.a = 0x50;
        computer.cpu.status.c = true;
        computer.process_instruction(Instruction::SBC(AddressingMode::Immediate));
        assert_eq!(computer.cpu.a, 0xa0);
        assert_eq!(computer.cpu.status.z, false);
        assert_eq!(computer.cpu.status.n, true);
        assert_eq!(computer.cpu.status.c, false);
        assert_eq!(computer.cpu.status.v, true);

        let mut computer: Computer = Default::default();
        computer.memory[0] = 0x70;
        computer.cpu.a = 0xd0;
        computer.cpu.status.c = true;
        computer.process_instruction(Instruction::SBC(AddressingMode::Immediate));
        assert_eq!(computer.cpu.a, 0x60);
        assert_eq!(computer.cpu.status.z, false);
        assert_eq!(computer.cpu.status.n, false);
        assert_eq!(computer.cpu.status.c, true);
        assert_eq!(computer.cpu.status.v, true);
    }

    // #[test]
    // fn test_ldx_imm() {
    //     let mut computer: Computer = Default::default();
    //     computer.memory[0] = 5;
    //     computer.process_instruction(Instruction::LDX(AddressingMode::Immediate));
    //     assert_eq!(computer.cpu.x, 5);
    //     assert_eq!(computer.cpu.status.z, false);
    //     assert_eq!(computer.cpu.status.n, false);

    //     let mut computer: Computer = Default::default();
    //     computer.memory[0] = 0xf4;
    //     computer.process_instruction(Instruction::LDX(AddressingMode::Immediate));
    //     assert_eq!(computer.cpu.x, 0xf4);
    //     assert_eq!(computer.cpu.status.z, false);
    //     assert_eq!(computer.cpu.status.n, true);

    //     let mut computer: Computer = Default::default();
    //     computer.process_instruction(Instruction::LDX(AddressingMode::Immediate));
    //     assert_eq!(computer.cpu.x, 0x00);
    //     assert_eq!(computer.cpu.status.z, true);
    //     assert_eq!(computer.cpu.status.n, false);
    // }

    // #[test]
    // fn test_ldy_imm() {
    //     let mut computer: Computer = Default::default();
    //     computer.memory[0] = 5;
    //     computer.process_instruction(InstructionOpcode::LDYimm);
    //     assert_eq!(computer.cpu.y, 5);
    //     assert_eq!(computer.cpu.status.z, false);
    //     assert_eq!(computer.cpu.status.n, false);

    //     let mut computer: Computer = Default::default();
    //     computer.memory[0] = 0xf4;
    //     computer.process_instruction(InstructionOpcode::LDYimm);
    //     assert_eq!(computer.cpu.y, 244);
    //     assert_eq!(computer.cpu.status.z, false);
    //     assert_eq!(computer.cpu.status.n, true);

    //     let mut computer: Computer = Default::default();
    //     computer.process_instruction(InstructionOpcode::LDYimm);
    //     assert_eq!(computer.cpu.x, 0x00);
    //     assert_eq!(computer.cpu.status.z, true);
    //     assert_eq!(computer.cpu.status.n, false);
    // }

    // #[test]
    // fn test_sty_zpgx() {
    //     let mut computer: Computer = Default::default();
    //     computer.memory[0] = 0x05;
    //     computer.cpu.x = 0x00;
    //     computer.cpu.y = 0xff;
    //     computer.process_instruction(InstructionOpcode::STYzpgx);
    //     assert_eq!(computer.cpu.y, computer.memory[0x05]);

    //     // tests whether zero-index + x wraps around
    //     let mut computer: Computer = Default::default();
    //     computer.memory[0] = 0xf4;
    //     computer.cpu.x = 0xf4;
    //     computer.cpu.y = 0x10;
    //     computer.process_instruction(InstructionOpcode::STYzpgx);
    //     assert_eq!(computer.cpu.y, computer.memory[232]);
    // }

    // #[test]
    // fn test_inx_impl() {
    //     let mut computer: Computer = Default::default();
    //     computer.cpu.x = 0x00;
    //     let original_x = computer.cpu.x;
    //     computer.process_instruction(InstructionOpcode::INXimpl);
    //     assert_eq!(computer.cpu.x, original_x + 1);
    //     assert_eq!(computer.cpu.status.n, false);
    //     assert_eq!(computer.cpu.status.z, false);

    //     let mut computer: Computer = Default::default();
    //     computer.cpu.x = 0xf0;
    //     let original_x = computer.cpu.x;
    //     computer.process_instruction(InstructionOpcode::INXimpl);
    //     assert_eq!(computer.cpu.x, original_x + 1);
    //     assert_eq!(computer.cpu.status.n, true);
    //     assert_eq!(computer.cpu.status.z, false);

    //     let mut computer: Computer = Default::default();
    //     computer.cpu.x = 0xff;
    //     computer.process_instruction(InstructionOpcode::INXimpl);
    //     assert_eq!(computer.cpu.x, 0);
    //     assert_eq!(computer.cpu.status.n, false);
    //     assert_eq!(computer.cpu.status.z, true);
    // }

    // // TODO: implement flags and improve test
    // #[test]
    // fn test_dey_impl() {
    //     let mut computer: Computer = Default::default();
    //     computer.cpu.y = 0x01;
    //     let original_y = computer.cpu.y;
    //     computer.process_instruction(InstructionOpcode::DEYimpl);
    //     assert_eq!(computer.cpu.y, original_y - 1);
    //     assert_eq!(computer.cpu.status.n, false);
    //     assert_eq!(computer.cpu.status.z, true);

    //     let mut computer: Computer = Default::default();
    //     computer.cpu.y = 0x18;
    //     let original_y = computer.cpu.y;
    //     computer.process_instruction(InstructionOpcode::DEYimpl);
    //     assert_eq!(computer.cpu.y, original_y - 1);
    //     assert_eq!(computer.cpu.status.n, false);
    //     assert_eq!(computer.cpu.status.z, false);

    //     let mut computer: Computer = Default::default();
    //     computer.cpu.y = 0x00;
    //     computer.process_instruction(InstructionOpcode::DEYimpl);
    //     assert_eq!(computer.cpu.y, 0xff);
    //     assert_eq!(computer.cpu.status.n, true);
    //     assert_eq!(computer.cpu.status.z, false);
    // }

    // #[test]
    // fn test_cpy_imm() {
    //     let mut computer: Computer = Default::default();
    //     computer.memory[0] = 0xa1;
    //     computer.cpu.y = 0xa1;
    //     computer.process_instruction(InstructionOpcode::CPYimm);
    //     assert_eq!(computer.cpu.status.n, false);
    //     assert_eq!(computer.cpu.status.z, true);
    //     assert_eq!(computer.cpu.status.c, true);

    //     let mut computer: Computer = Default::default();
    //     computer.memory[0] = 0xa1;
    //     computer.cpu.y = 0x10;
    //     computer.process_instruction(InstructionOpcode::CPYimm);
    //     assert_eq!(computer.cpu.status.n, false);
    //     assert_eq!(computer.cpu.status.z, false);
    //     assert_eq!(computer.cpu.status.c, false);

    //     let mut computer: Computer = Default::default();
    //     computer.memory[0] = 0x20;
    //     computer.cpu.y = 0x10;
    //     computer.process_instruction(InstructionOpcode::CPYimm);
    //     assert_eq!(computer.cpu.status.n, true);
    //     assert_eq!(computer.cpu.status.z, false);
    //     assert_eq!(computer.cpu.status.c, false);

    //     let mut computer: Computer = Default::default();
    //     computer.memory[0] = 0x10;
    //     computer.cpu.y = 0xa1;
    //     computer.process_instruction(InstructionOpcode::CPYimm);
    //     assert_eq!(computer.cpu.status.n, true);
    //     assert_eq!(computer.cpu.status.z, false);
    //     assert_eq!(computer.cpu.status.c, true);
    // }

    // #[test]
    // fn test_bne_rel() {
    //     // backward jump
    //     let mut computer: Computer = Default::default();
    //     computer.memory[10] = 0xf5;
    //     computer.cpu.pc = 10;
    //     computer.memory[0] = 0xaa;
    //     computer.process_instruction(InstructionOpcode::BNErel);
    //     assert_eq!(fetch_instruction(&computer.memory, &mut computer.cpu), 0xaa);

    //     // forward jump
    //     let mut computer: Computer = Default::default();
    //     computer.memory[10] = 0x04;
    //     computer.cpu.pc = 10;
    //     computer.memory[15] = 0xaa;
    //     computer.process_instruction(InstructionOpcode::BNErel);
    //     assert_eq!(fetch_instruction(&computer.memory, &mut computer.cpu), 0xaa);
    // }
}
