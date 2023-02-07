// Disco5.rs
// 6502 hexdump Decoder
// Author: Will Howes

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
// use std::str::FromStr;
use strum_macros::FromRepr;

const MEMORY_SIZE: usize = 1000;

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
    /// invalid MEMORY HANDLER
    #[default]
    Invalid,
}

/// Type for storing CPU fields
#[derive(Copy, Clone)]
struct CPU {
    a: u8,
    x: u8,
    y: u8,
    z: u8,
    pc: u16,
}

impl CPU {
    /// steps pc to next position
    fn step(&mut self) {
        self.pc = self.pc + 1;
    }

    /// processes 6502 instructions stored in enum data type Instruction
    fn process_instruction(&mut self, instruction: Instruction, memory: &mut [u8]) {
        match instruction {
            Instruction::I0xa2 => {
                let new_x = lb(memory, self);
                self.x = new_x;
            }
            Instruction::I0xa0 => {
                let new_y = lb(memory, self);
                self.y = new_y;
            }
            Instruction::I0x94 => {
                let zpg = lb(memory, self);
                memory[usize::from(zpg + self.x)] = self.y;
            }
            Instruction::I0xe8 => {
                self.x += 1;
            }
            Instruction::I0x88 => {
                self.y -= 1;
            }
            Instruction::I0xc0 => {
                let test_val = lb(memory, self);
                if self.y == test_val {
                    self.z = 1;
                } else {
                    self.z = 0;
                }
            }
            Instruction::I0xd0 => {
                let offset: u8 = lb(memory, self);
                let offset: i8 = offset as i8;
                let mut negative = false;
                if offset.is_negative() {
                    negative = true;
                }
                let offset = offset.abs();
                let offset = offset as u16;
                if self.z == 0 && negative == false {
                    self.pc += u16::from(offset);
                } else if self.z == 0 && negative == true {
                    self.pc -= u16::from(offset);
                }
            }
            Instruction::Invalid => (),
        }
    }
}

struct Computer {
    cpu: CPU,
    memory: [u8; MEMORY_SIZE],
}

/// loads instruction at address of pc, increments pc
fn lb(memory: &[u8], cpu: &mut CPU) -> u8 {
    let index = cpu.pc;
    cpu.step();
    memory[index as usize]
}

fn load_program(computer: &mut Computer) -> io::Result<()> {
    let memory = &mut computer.memory;
    let cpu = &mut computer.cpu;
    // Load file contents into a buffer
    let f = File::open("countdown.txt")?;
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

fn run_program(computer: &mut Computer) {
    let memory = &mut computer.memory;
    let cpu = &mut computer.cpu;

    while usize::from(cpu.pc) < memory.len() {
        let instruction = lb(memory, cpu);
        let instruction = usize::from(instruction);
        let instruction = Instruction::from_repr(instruction);
        let instruction = instruction.unwrap_or_default();
        cpu.process_instruction(instruction, memory);
    }
}

fn main() {
    let mut computer = Computer {
        cpu: CPU {
            a: 0,
            x: 0,
            y: 0,
            z: 0,
            pc: 0,
        },
        memory: [0; MEMORY_SIZE],
    };

    let program = load_program(&mut computer);
    program.unwrap(); // verify that program loaded

    println!("BEFORE: 0600: {:?}", &computer.memory[600..616]);
    println!("BEFORE: 0016: {:?}", &computer.memory[16..32]);

    run_program(&mut computer);
    println!("AFTER : 0016: {:?}", &computer.memory[16..32]);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_match_message_call() {
//         let mut state = State {
//             quit: false,
//             position: Point { x: 0, y: 0 },
//             color: (0, 0, 0),
//         };
//         state.process(Message::ChangeColor((255, 0, 255)));
//         state.process(Message::Echo(String::from("hello world")));
//         state.process(Message::Move(Point { x: 10, y: 15 }));
//         state.process(Message::Quit);

//         assert_eq!(state.color, (255, 0, 255));
//         assert_eq!(state.position.x, 10);
//         assert_eq!(state.position.y, 15);
//         assert_eq!(state.quit, true);
//     }
// }
