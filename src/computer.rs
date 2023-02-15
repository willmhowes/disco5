#[allow(non_camel_case_types)]
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use strum_macros::FromRepr;

pub mod cpu;

use crate::computer::cpu::*;

const MEMORY_SIZE: usize = 0xffff;

/// Enum for each 6502 instruction
#[derive(Debug, FromRepr, Default)]
enum Instruction {
    // ADCimm = 0x69,
    LDXimm = 0xa2,
    LDYimm = 0xa0,
    STYzpgx = 0x94,
    INXimpl = 0xe8,
    DEYimpl = 0x88,
    CPYimm = 0xc0,
    BNErel = 0xd0,
    /// catch-all invalid instruction
    #[default]
    Invalid = 0x100, // larger than any possible 6502 instruction
}

/// loads instruction at address of pc, increments pc
fn lb(memory: &[u8], cpu: &mut CPU) -> u8 {
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
            let instruction = lb(&self.memory, &mut self.cpu);
            let instruction = usize::from(instruction);
            let instruction = Instruction::from_repr(instruction);
            let instruction = instruction.unwrap_or_default();
            self.process_instruction(instruction);
        }
    }

    fn set_status_nz(&mut self, test_val: u8) {
        self.cpu.status.z = if test_val == 0 { true } else { false };
        // 0x80 = 0b1000_0000
        self.cpu.status.n = if test_val & 0x80 == 0x80 { true } else { false };
    }

    /// processes 6502 instructions stored in enum data type Instruction
    fn process_instruction(&mut self, instruction: Instruction) {
        match instruction {
            // Instruction::ADCimm => {
            //     let addend = lb(&self.memory, &mut self.cpu);

            // }
            Instruction::LDXimm => {
                self.cpu.x = lb(&self.memory, &mut self.cpu);
                self.set_status_nz(self.cpu.x);
            }
            Instruction::LDYimm => {
                self.cpu.y = lb(&self.memory, &mut self.cpu);
                self.set_status_nz(self.cpu.y);
            }
            Instruction::STYzpgx => {
                let zpg = lb(&self.memory, &mut self.cpu);
                let index = zpg.wrapping_add(self.cpu.x);
                let index: usize = usize::from(index);
                self.memory[index] = self.cpu.y;
            }
            Instruction::INXimpl => {
                self.cpu.x = self.cpu.x.wrapping_add(1);
                self.set_status_nz(self.cpu.x);
            }
            Instruction::DEYimpl => {
                self.cpu.y = self.cpu.y.wrapping_sub(1);
                self.set_status_nz(self.cpu.y);
            }
            Instruction::CPYimm => {
                let test_val = lb(&self.memory, &mut self.cpu);
                self.cpu.status.c = if self.cpu.y >= test_val { true } else { false };
                self.set_status_nz(self.cpu.y.wrapping_sub(test_val));
            }
            Instruction::BNErel => {
                let offset: u8 = lb(&self.memory, &mut self.cpu);
                let offset: i8 = offset as i8;
                let mut negative = false;
                if offset.is_negative() {
                    negative = true;
                }
                let offset = offset.abs();
                let offset = offset as u16;
                if self.cpu.status.z == false && negative == false {
                    self.cpu.pc += u16::from(offset);
                } else if self.cpu.status.z == false && negative == true {
                    self.cpu.pc -= u16::from(offset);
                }
            }
            Instruction::Invalid => (),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ldx_imm() {
        let mut computer: Computer = Default::default();
        computer.memory[0] = 5;
        computer.process_instruction(Instruction::LDXimm);
        assert_eq!(computer.cpu.x, 5);
        assert_eq!(computer.cpu.status.z, false);
        assert_eq!(computer.cpu.status.n, false);

        let mut computer: Computer = Default::default();
        computer.memory[0] = 0xf4;
        computer.process_instruction(Instruction::LDXimm);
        assert_eq!(computer.cpu.x, 0xf4);
        assert_eq!(computer.cpu.status.z, false);
        assert_eq!(computer.cpu.status.n, true);

        let mut computer: Computer = Default::default();
        computer.process_instruction(Instruction::LDXimm);
        assert_eq!(computer.cpu.x, 0x00);
        assert_eq!(computer.cpu.status.z, true);
        assert_eq!(computer.cpu.status.n, false);
    }

    #[test]
    fn test_ldy_imm() {
        let mut computer: Computer = Default::default();
        computer.memory[0] = 5;
        computer.process_instruction(Instruction::LDYimm);
        assert_eq!(computer.cpu.y, 5);
        assert_eq!(computer.cpu.status.z, false);
        assert_eq!(computer.cpu.status.n, false);

        let mut computer: Computer = Default::default();
        computer.memory[0] = 0xf4;
        computer.process_instruction(Instruction::LDYimm);
        assert_eq!(computer.cpu.y, 244);
        assert_eq!(computer.cpu.status.z, false);
        assert_eq!(computer.cpu.status.n, true);

        let mut computer: Computer = Default::default();
        computer.process_instruction(Instruction::LDYimm);
        assert_eq!(computer.cpu.x, 0x00);
        assert_eq!(computer.cpu.status.z, true);
        assert_eq!(computer.cpu.status.n, false);
    }

    #[test]
    fn test_sty_zpgx() {
        let mut computer: Computer = Default::default();
        computer.memory[0] = 0x05;
        computer.cpu.x = 0x00;
        computer.cpu.y = 0xff;
        computer.process_instruction(Instruction::STYzpgx);
        assert_eq!(computer.cpu.y, computer.memory[0x05]);

        // tests whether zero-index + x wraps around
        let mut computer: Computer = Default::default();
        computer.memory[0] = 0xf4;
        computer.cpu.x = 0xf4;
        computer.cpu.y = 0x10;
        computer.process_instruction(Instruction::STYzpgx);
        assert_eq!(computer.cpu.y, computer.memory[232]);
    }

    #[test]
    fn test_inx_impl() {
        let mut computer: Computer = Default::default();
        computer.cpu.x = 0x00;
        let original_x = computer.cpu.x;
        computer.process_instruction(Instruction::INXimpl);
        assert_eq!(computer.cpu.x, original_x + 1);
        assert_eq!(computer.cpu.status.n, false);
        assert_eq!(computer.cpu.status.z, false);

        let mut computer: Computer = Default::default();
        computer.cpu.x = 0xf0;
        let original_x = computer.cpu.x;
        computer.process_instruction(Instruction::INXimpl);
        assert_eq!(computer.cpu.x, original_x + 1);
        assert_eq!(computer.cpu.status.n, true);
        assert_eq!(computer.cpu.status.z, false);

        let mut computer: Computer = Default::default();
        computer.cpu.x = 0xff;
        computer.process_instruction(Instruction::INXimpl);
        assert_eq!(computer.cpu.x, 0);
        assert_eq!(computer.cpu.status.n, false);
        assert_eq!(computer.cpu.status.z, true);
    }

    // TODO: implement flags and improve test
    #[test]
    fn test_dey_impl() {
        let mut computer: Computer = Default::default();
        computer.cpu.y = 0x01;
        let original_y = computer.cpu.y;
        computer.process_instruction(Instruction::DEYimpl);
        assert_eq!(computer.cpu.y, original_y - 1);
        assert_eq!(computer.cpu.status.n, false);
        assert_eq!(computer.cpu.status.z, true);

        let mut computer: Computer = Default::default();
        computer.cpu.y = 0x18;
        let original_y = computer.cpu.y;
        computer.process_instruction(Instruction::DEYimpl);
        assert_eq!(computer.cpu.y, original_y - 1);
        assert_eq!(computer.cpu.status.n, false);
        assert_eq!(computer.cpu.status.z, false);

        let mut computer: Computer = Default::default();
        computer.cpu.y = 0x00;
        computer.process_instruction(Instruction::DEYimpl);
        assert_eq!(computer.cpu.y, 0xff);
        assert_eq!(computer.cpu.status.n, true);
        assert_eq!(computer.cpu.status.z, false);
    }

    #[test]
    fn test_cpy_imm() {
        let mut computer: Computer = Default::default();
        computer.memory[0] = 0xa1;
        computer.cpu.y = 0xa1;
        computer.process_instruction(Instruction::CPYimm);
        assert_eq!(computer.cpu.status.n, false);
        assert_eq!(computer.cpu.status.z, true);
        assert_eq!(computer.cpu.status.c, true);

        let mut computer: Computer = Default::default();
        computer.memory[0] = 0xa1;
        computer.cpu.y = 0x10;
        computer.process_instruction(Instruction::CPYimm);
        assert_eq!(computer.cpu.status.n, false);
        assert_eq!(computer.cpu.status.z, false);
        assert_eq!(computer.cpu.status.c, false);

        let mut computer: Computer = Default::default();
        computer.memory[0] = 0x20;
        computer.cpu.y = 0x10;
        computer.process_instruction(Instruction::CPYimm);
        assert_eq!(computer.cpu.status.n, true);
        assert_eq!(computer.cpu.status.z, false);
        assert_eq!(computer.cpu.status.c, false);

        let mut computer: Computer = Default::default();
        computer.memory[0] = 0x10;
        computer.cpu.y = 0xa1;
        computer.process_instruction(Instruction::CPYimm);
        assert_eq!(computer.cpu.status.n, true);
        assert_eq!(computer.cpu.status.z, false);
        assert_eq!(computer.cpu.status.c, true);
    }

    #[test]
    fn test_bne_rel() {
        // backward jump
        let mut computer: Computer = Default::default();
        computer.memory[10] = 0xf5;
        computer.cpu.pc = 10;
        computer.memory[0] = 0xaa;
        computer.process_instruction(Instruction::BNErel);
        assert_eq!(lb(&computer.memory, &mut computer.cpu), 0xaa);

        // forward jump
        let mut computer: Computer = Default::default();
        computer.memory[10] = 0x04;
        computer.cpu.pc = 10;
        computer.memory[15] = 0xaa;
        computer.process_instruction(Instruction::BNErel);
        assert_eq!(lb(&computer.memory, &mut computer.cpu), 0xaa);
    }
}
