// #[allow(non_camel_case_types)]
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub mod bus;
pub mod cpu;
pub mod cpu_structs;
pub mod ppu;

use crate::computer::bus::Bus;
use crate::computer::cpu::{StatusRegister, CPU};
use crate::computer::cpu_structs::map_byte_to_instruction;

#[derive(Debug)]
pub struct Computer {
    pub cpu: CPU,
    pub memory: Bus,
    pub flags: StatusRegister,
    pub clock: u64,
}

impl Default for Computer {
    fn default() -> Computer {
        Computer {
            cpu: CPU {
                ..Default::default()
            },
            memory: Default::default(),
            flags: Default::default(),
            clock: Default::default(),
        }
    }
}

impl Computer {
    pub fn tick(&mut self, num: u8) {
        self.clock += u64::from(num);
    }

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
            println!("WRITING TO LINE {}", cpu.pc);
            for hex in &hexdump[1..] {
                memory[usize::from(loc)] = u8::from_str_radix(hex, 16).unwrap();
                loc += 1;
            }
        }

        Ok(())
    }

    pub fn load_program_from_hex(
        &mut self,
        filename: &str,
        memory_entry_point: usize,
        pc: u16,
    ) -> io::Result<()> {
        let memory = &mut self.memory.bytes[memory_entry_point..];
        let cpu = &mut self.cpu;

        // Load file contents into memory array
        let f = File::open(filename)?;
        let mut f = BufReader::new(f);
        let bytes_read = f.read(memory)?;
        println!("{bytes_read} bytes read");

        cpu.pc = pc;

        Ok(())
    }

    pub fn run_program(&mut self, loud: bool, exit_condition: fn(u16) -> bool) {
        loop {
            if loud {
                println!("--------------------");
                println!("Clock = {}", self.clock);
                self.cpu.print_state();
            }
            let instruction = self.cpu.fetch_instruction(&self.memory);
            let (instruction, minimum_ticks) = map_byte_to_instruction(instruction);
            if loud {
                println!("NEXT: {:?}, minimum {:?} ticks", instruction, minimum_ticks);
                println!("--------------------");
            }
            // let mut line = String::new();
            // let b1 = std::io::stdin().read_line(&mut line).unwrap();
            let ticks = self
                .cpu
                .process_instruction(instruction, minimum_ticks, &mut self.memory);

            self.tick(ticks);

            if exit_condition(self.cpu.pc) == true {
                println!("SUCCESS");
                println!("CLOCK = {}", self.clock);
                println!("PC    = 0x{:0>4x}", self.cpu.pc);
                break;
            }
        }
    }
}
