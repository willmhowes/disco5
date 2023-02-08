// Disco5.rs
// 6502 hexdump Decoder
// Author: Will Howes

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
// use std::str::FromStr;
use strum_macros::FromRepr;

const MEMORY_SIZE: usize = 0xffff;

/// Enum for each 6502 instruction
#[derive(Debug, FromRepr, Default)]
enum Instruction {
    /// LDX #
    I0xa2 = 0xa2,
    /// LDY #
    I0xa0 = 0xa0,
    /// STY zpg,X
    I0x94 = 0x94,
    /// INX impl
    I0xe8 = 0xe8,
    /// DEY impl
    I0x88 = 0x88,
    /// CPY #
    I0xc0 = 0xc0,
    /// BNE rel
    I0xd0 = 0xd0,
    /// invalid instruction catch-all
    #[default]
    Invalid = 0x100,
}

/// Type for storing CPU fields
#[derive(Copy, Clone, Default)]
struct CPU {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,
}

#[derive(Default)]

struct Flags {
    n: bool,
    v: bool,
    b: bool,
    d: bool,
    i: bool,
    z: bool,
    c: bool,
}

impl CPU {
    /// steps pc to next position
    fn step(&mut self) {
        self.pc = self.pc + 1;
    }
}

struct Computer {
    cpu: CPU,
    memory: [u8; MEMORY_SIZE],
    flags: Flags,
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
    fn load_program(&mut self, filename: &str) -> io::Result<()> {
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

    fn run_program(&mut self) {
        while usize::from(self.cpu.pc) < self.memory.len() {
            let instruction = lb(&self.memory, &mut self.cpu);
            let instruction = usize::from(instruction);
            let instruction = Instruction::from_repr(instruction);
            let instruction = instruction.unwrap_or_default();
            self.process_instruction(instruction);
        }
    }

    /// processes 6502 instructions stored in enum data type Instruction
    fn process_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::I0xa2 => {
                let new_x = lb(&self.memory, &mut self.cpu);
                self.cpu.x = new_x;
            }
            Instruction::I0xa0 => {
                let new_y = lb(&self.memory, &mut self.cpu);
                self.cpu.y = new_y;
            }
            Instruction::I0x94 => {
                let zpg = usize::from(lb(&self.memory, &mut self.cpu));
                let x = usize::from(self.cpu.x);
                self.memory[(zpg + x) % 255] = self.cpu.y;
            }
            Instruction::I0xe8 => {
                self.cpu.x += 1;
            }
            Instruction::I0x88 => {
                self.cpu.y -= 1;
                // if self.cpu.y == 0 {
                //     self.flags.z = true;
                // }
            }
            Instruction::I0xc0 => {
                let test_val = lb(&self.memory, &mut self.cpu);
                if self.cpu.y == test_val {
                    self.flags.z = true;
                } else {
                    self.flags.z = false;
                }
            }
            Instruction::I0xd0 => {
                let offset: u8 = lb(&self.memory, &mut self.cpu);
                let offset: i8 = offset as i8;
                let mut negative = false;
                if offset.is_negative() {
                    negative = true;
                }
                let offset = offset.abs();
                let offset = offset as u16;
                if self.flags.z == false && negative == false {
                    self.cpu.pc += u16::from(offset);
                } else if self.flags.z == false && negative == true {
                    self.cpu.pc -= u16::from(offset);
                }
            }
            Instruction::Invalid => (),
        }
    }
}

/// loads instruction at address of pc, increments pc
fn lb(memory: &[u8], cpu: &mut CPU) -> u8 {
    let index = cpu.pc;
    cpu.step();
    memory[index as usize]
}

fn main() {
    let mut computer: Computer = Default::default();

    computer
        .load_program(&String::from("countdown.txt"))
        .unwrap(); // NOTE: verifies that program loaded without errors

    println!("BEFORE: 0600: {:?}", &computer.memory[600..616]);
    println!("BEFORE: 0016: {:?}", &computer.memory[16..32]);

    computer.run_program();
    println!("AFTER : 0016: {:?}", &computer.memory[16..32]);
}

// #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_0xa2() {
        let mut computer: Computer = Default::default();

        computer.memory[0] = 5;
        computer.process_instruction(Instruction::I0xa2);
        assert_eq!(computer.cpu.x, 5);

        let mut computer: Computer = Default::default();
        computer.memory[0] = 0xf4;
        computer.process_instruction(Instruction::I0xa2);
        assert_eq!(computer.cpu.x, 0xf4);
    }

    #[test]
    fn test_instruction_0xa0() {
        let mut computer: Computer = Default::default();
        computer.memory[0] = 5;
        computer.process_instruction(Instruction::I0xa0);
        assert_eq!(computer.cpu.y, 5);

        let mut computer: Computer = Default::default();
        computer.memory[0] = 0xf4;
        computer.process_instruction(Instruction::I0xa0);
        assert_eq!(computer.cpu.y, 244);
    }

    #[test]
    fn test_instruction_0x94() {
        let mut computer: Computer = Default::default();
        computer.memory[0] = 0x05;
        computer.cpu.x = 0x00;
        computer.cpu.y = 0xff;
        computer.process_instruction(Instruction::I0x94);
        assert_eq!(computer.cpu.y, computer.memory[0x05]);

        // tests whether zero-index + x wraps around
        let mut computer: Computer = Default::default();
        computer.memory[0] = 0xf4;
        computer.cpu.x = 0xf4;
        computer.cpu.y = 0x10;
        computer.process_instruction(Instruction::I0x94);
        assert_eq!(computer.cpu.y, computer.memory[233]);
    }

    #[test]
    fn test_instruction_0xe8() {
        let mut computer: Computer = Default::default();
        computer.cpu.x = 0x00;
        let original_x = computer.cpu.x;
        computer.process_instruction(Instruction::I0xe8);
        assert_eq!(computer.cpu.x, original_x + 1);

        let mut computer: Computer = Default::default();
        computer.cpu.x = 0xb8;
        let original_x = computer.cpu.x;
        computer.process_instruction(Instruction::I0xe8);
        assert_eq!(computer.cpu.x, original_x + 1);
    }

    // TODO: implement flags and improve test
    #[test]
    fn test_instruction_0x88() {
        let mut computer: Computer = Default::default();
        computer.cpu.y = 0x01;
        let original_y = computer.cpu.y;
        computer.process_instruction(Instruction::I0x88);
        assert_eq!(computer.cpu.y, original_y - 1);

        let mut computer: Computer = Default::default();
        computer.cpu.y = 0xb8;
        let original_y = computer.cpu.y;
        computer.process_instruction(Instruction::I0x88);
        assert_eq!(computer.cpu.y, original_y - 1);
    }

    #[test]
    fn test_instruction_0xc0() {
        let mut computer: Computer = Default::default();
        computer.memory[0] = 0xa1;
        computer.cpu.y = 0xa1;
        computer.process_instruction(Instruction::I0xc0);
        assert_eq!(computer.flags.z, true);

        let mut computer: Computer = Default::default();
        computer.memory[0] = 0xb1;
        computer.cpu.y = 0xa1;
        computer.process_instruction(Instruction::I0xc0);
        assert_eq!(computer.flags.z, false);
    }
}
