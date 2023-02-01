// Disco5.rs
// 6502 hexdump Decoder
// Author: Will Howes

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

/// Type for storing CPU registers
struct Registers {
    a: Option<usize>,
    x: Option<usize>,
    y: Option<usize>,
    z: Option<usize>,
    pc: Option<usize>,
}

/// loads instruction at address of pc, increments pc
fn lb(memory: &mut [u8], pc: usize) -> (u8, usize) {
    (memory[pc], pc+1)
}

fn main() -> io::Result<()> {
    let mut registers = Registers {
        a: Some(0),
        x: Some(0),
        y: Some(0),
        z: Some(0),
        pc: None,
    };

    // initialize memory array to all zeroes
    let mut memory: [u8; 1000] = [0; 1000];
    // memory[16] = 2;

    let f = File::open("countdown.txt")?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line?;
        let hexdump: Vec<&str> = line.split(' ').collect();

        // Identify location of code in memory
        let loc_length = hexdump[0].chars().count();
        let loc = &hexdump[0][0..loc_length - 1];
        let mut loc: usize = loc.parse().unwrap();
        if registers.pc == None {
            registers.pc = Some(loc as i32);
        };

        // Write instructions to memory
        println!("LINE : {}",registers.pc.unwrap());
        for hex in &hexdump[1..] {
            println!("{} || {:b}", hex, u8::from_str_radix(hex, 16).unwrap());
            memory[loc] = u8::from_str_radix(hex, 16).unwrap();
            loc+=1;
        }

        // Inspect memory
        println!("0600: {:?}", &memory[600..616]);
        println!("0010: {:?}", &memory[16..32]);
    }

    Ok(())

    // another_function();
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
