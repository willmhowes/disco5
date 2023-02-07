// Disco5.rs
// 6502 hexdump Decoder
// Author: Will Howes

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

/// Type for storing CPU registers
#[derive(Copy, Clone)]
struct Registers {
    a: u8,
    x: u8,
    y: u8,
    z: u8,
    pc: u16,
}

impl Registers {
    /// steps pc to next position
    fn step(&mut self) {
        self.pc = self.pc + 1;
    }
}

// enum Instruction {
//     0xa2
// }

/// loads instruction at address of pc, increments pc
fn lb(memory: &[u8], registers: &mut Registers) -> u8 {
    let index = registers.pc;
    registers.step();
    memory[index as usize]
}

fn load_program(memory: &mut [u8], registers: &mut Registers) -> io::Result<()> {
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

        if registers.pc == 0 {
            registers.pc = loc;
        };

        // Write instructions to memory
        println!("LINE : {}", registers.pc);
        for hex in &hexdump[1..] {
            // println!("{} || {:b}", hex, u8::from_str_radix(hex, 16).unwrap());
            memory[usize::from(loc)] = u8::from_str_radix(hex, 16).unwrap();
            loc += 1;
        }
    }

    Ok(())
}

fn run_program(memory: &mut [u8], mut registers: Registers) -> Registers {
    while registers.pc < 1000 {
        let instruction = lb(&memory, &mut registers);
        match instruction {
            // LDX #
            0xa2 => {
                let new_x = lb(&memory, &mut registers);
                registers.x = new_x;
            }
            // LDY #
            0xa0 => {
                let new_y = lb(&memory, &mut registers);
                registers.y = new_y;
            }
            // STY zpg,X
            0x94 => {
                let zpg = lb(&memory, &mut registers);
                memory[usize::from(zpg + registers.x)] = registers.y;
            }
            // INX impl
            0xe8 => {
                registers.x += 1;
            }
            // DEY impl
            0x88 => {
                registers.y -= 1;
            }
            // CPY #
            0xc0 => {
                let test_val = lb(&memory, &mut registers);
                if registers.y == test_val {
                    registers.z = 1;
                } else {
                    registers.z = 0;
                }
            }
            // BNE rel
            0xd0 => {
                let offset: u8 = lb(&memory, &mut registers);
                let offset: i8 = offset as i8;
                let mut negative = false;
                if offset.is_negative() {
                    negative = true;
                }
                let offset = offset.abs();
                let offset = offset as u16;
                if registers.z == 0 && negative == false {
                    registers.pc += u16::from(offset);
                } else if registers.z == 0 && negative == true {
                    registers.pc -= u16::from(offset);
                }
            }
            0x00 => {
                ()
            }

            _ => panic!("Unexpected instruction {} at position {} in memory", instruction, registers.pc)
            // _ => (),
        }
        // println!("Register PC: {}", registers.pc);
        // println!("Instruction: {}", instruction);
    }
    // println!("Register PC: {}", registers.pc.unwrap());

    registers
}

fn main() {
    let mut registers = Registers {
        a: 0,
        x: 0,
        y: 0,
        z: 0,
        pc: 0,
    };

    // initialize memory array to all zeroes
    let mut memory: [u8; 1000] = [0; 1000];

    let program = load_program(&mut memory, &mut registers);
    program.unwrap(); // verify that program loaded

    println!("0600: {:?}", &memory[600..616]);
    println!("0016: {:?}", &memory[16..32]);

    let registers = run_program(&mut memory, registers);
    println!("Register x after run_program: {}", registers.x);

    println!("0016: {:?}", &memory[16..32]);
}

// enums3.rs
// Address all the TODOs to make the tests pass!
// Execute `rustlings hint enums3` or use the `hint` watch subcommand for a hint.

// enum Message {
//     ChangeColor((u8,u8,u8)),
//     Echo(String),
//     Move(Point),
//     Quit
// }

// struct Point {
//     x: u8,
//     y: u8,
// }

// struct State {
//     color: (u8, u8, u8),
//     position: Point,
//     quit: bool,
// }

// impl State {
//     fn change_color(&mut self, color: (u8, u8, u8)) {
//         self.color = color;
//     }

//     fn quit(&mut self) {
//         self.quit = true;
//     }

//     fn echo(&self, s: String) {
//         println!("{}", s);
//     }

//     fn move_position(&mut self, p: Point) {
//         self.position = p;
//     }

//     fn process(&mut self, message: Message) {
//         match message {
//             Message::ChangeColor((x,y,z)) => self.change_color((x,y,z)),
//             Message::Echo(s) => self.echo(s),
//             Message::Move(p) => self.move_position(p),
//             Message::Quit => self.quit(),
//         }
//     }
// }

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
