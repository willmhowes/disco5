// Disco5.rs
// 6502 hexdump Decoder
// Author: Will Howes

use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

// enum SysVars {
//     A,
//     X,
//     Y,
//     Z,
//     PC,
// }


fn main() -> io::Result<()> {
    // let pc = None;

    // initialize memory array to all zeroes
    let mut memory: [i32; 1000] = [0; 1000];
    memory[16] = 2;
    println!("0010: {:?}", &memory[16..32]);

    let f = File::open("countdown.txt")?;
    let f = BufReader::new(f);

    for line in f.lines() {
        // println!("{}", line.unwrap());
        let good = line?;
        let hexdump: Vec<&str> = good.split(' ').collect();
        for hex in hexdump {
            println!("{hex}")
        }
        // let loc = int(hexdump[0][:-1])
        // if pc == None:
        //     pc = loc
        // # write instructions to memory
        // for instr in hexdump[1:]:
        //     memory[loc] = int(instr, 16)
        //     loc += 1
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
