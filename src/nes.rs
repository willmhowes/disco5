use core::time;
// #[allow(non_camel_case_types)]
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, SeekFrom};
use std::thread;
use std::time::Instant;

use speedy2d::image::{ImageDataType, ImageSmoothingMode};
use speedy2d::shape::Rectangle;
use speedy2d::window::{WindowHandler, WindowHelper};
use speedy2d::Graphics2D;

pub mod bus;
pub mod cpu;
pub mod cpu_structs;
pub mod ppu;
pub mod ppu_structs;

use crate::nes::bus::Bus;
use crate::nes::cpu::CPU;
use crate::nes::cpu_structs::{decode_instruction, Instruction};
use crate::nes::ppu::FRAME_BUFFER_SIZE;
use crate::nes::ppu_structs::PPUCTRL;

const PPU_SCANLINES_PER_FRAME: u64 = 262;
const PPU_CYCLES_PER_SCANLINES: u64 = 341;
const PPU_CYCLES_PER_FRAME: u64 = PPU_SCANLINES_PER_FRAME * PPU_CYCLES_PER_SCANLINES;

const CPU_CYCLES_PER_FRAME: u64 = PPU_CYCLES_PER_FRAME / 3;
const LENGTH_OF_FRAME: f64 = 1.0 / 60.0;

const LOUD: bool = false;

#[derive(Debug, Default)]
pub struct NES {
    pub cpu: CPU,
    pub address_space: Bus,
}

fn byte_dump(memory: &[u8]) {
    let mut i = 0;
    let mut line_count = 0;
    for byte in memory {
        if i == 0 {
            print!("{line_count:0>7x} :");
        }
        if i < 15 {
            print!(" {byte:0>2x}");
            i += 1;
        } else {
            println!(" {byte:0>2x}");
            i = 0;
            line_count += 16;
        }
    }
}

impl NES {
    pub fn load_asm_6502js(&mut self, filename: &str) -> io::Result<()> {
        let memory = &mut self.address_space;
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

    pub fn load_asm_as65(
        &mut self,
        filename: &str,
        memory_entry_point: usize,
        pc: u16,
    ) -> io::Result<()> {
        let memory = &mut self.address_space.bytes[memory_entry_point..];

        // Load file contents into memory array
        let f = File::open(filename)?;
        let mut f = BufReader::new(f);
        let bytes_read = f.read(memory)?;
        println!("{bytes_read} bytes read");

        self.cpu.pc = pc;

        Ok(())
    }

    fn process_ines_header(memory: &[u8]) {
        println!("--------------------");
        println!("| Header Bytes     |");
        println!("--------------------");
        println!(
            "| 0   | {:0>8b}   | {}",
            memory[0],
            if memory[0] == 0x4e {
                "valid"
            } else {
                "invalid"
            }
        );
        println!(
            "| 1   | {:0>8b}   | {}",
            memory[1],
            if memory[1] == 0x45 {
                "valid"
            } else {
                "invalid"
            }
        );
        println!(
            "| 2   | {:0>8b}   | {}",
            memory[2],
            if memory[2] == 0x53 {
                "valid"
            } else {
                "invalid"
            }
        );
        println!(
            "| 3   | {:0>8b}   | {}",
            memory[3],
            if memory[3] == 0x1a {
                "valid"
            } else {
                "invalid"
            }
        );
        println!("--------------------");
        println!(
            "| 4   | {:0>8b}   | PRG ROM = 16 KB * {}",
            memory[4], memory[4]
        );
        println!(
            "| 5   | {:0>8b}   | CHR ROM = 8 KB * {}",
            memory[5], memory[5]
        );
        println!("--------------------");
        println!("| 6   | {:0>8b}   |", memory[6]);
        let six = format!("{:0>8b}", memory[6]);
        let six = six.as_bytes();
        println!("| 6.0 | {}   |", six[0] as char);
        println!("--------------------");
    }

    pub fn load_nrom_128(&mut self, filename: &str, memory_entry_point: usize) -> io::Result<()> {
        // Load file contents into memory array
        let f = File::open(filename)?;
        let mut f = BufReader::new(f);
        f.seek(SeekFrom::Start(16))?;

        let cpu_memory_0 =
            &mut self.address_space.bytes[memory_entry_point..memory_entry_point + 0x4000];
        f.read_exact(cpu_memory_0)?;

        f.seek(SeekFrom::Start(16))?;
        let cpu_memory_1 =
            &mut self.address_space.bytes[memory_entry_point + 0x4000..memory_entry_point + 0x8000];
        f.read_exact(cpu_memory_1)?;

        // This should be the only time the PPU's memory is directly addressed
        let ppu_memory = &mut self.address_space.ppu.address_space[..0x2000];
        f.read_exact(ppu_memory)?;

        let lo = self.address_space.bytes[0xfffc];
        let hi = self.address_space.bytes[0xfffd];
        let address = (u16::from(hi) << 8) + u16::from(lo);

        self.cpu.pc = address;

        Ok(())
    }

    pub fn run_cpu_program(&mut self, loud: bool, exit_condition: fn(u16) -> bool) {
        while exit_condition(self.cpu.pc) == false {
            if loud {
                println!("--------------------");
                println!("Clock = {}", self.cpu.clock);
                self.cpu.print_state();
            }
            let instruction = self.cpu.fetch_instruction(&self.address_space);
            let (instruction, minimum_ticks) = decode_instruction(instruction);
            if loud {
                println!("NEXT: {:?}, minimum {:?} ticks", instruction, minimum_ticks);
                println!("--------------------");
            }

            let _ =
                self.cpu
                    .execute_instruction(instruction, minimum_ticks, &mut self.address_space);
        }
        println!("SUCCESS");
        println!("CLOCK = {}", self.cpu.clock);
        println!("PC    = 0x{:0>4x}", self.cpu.pc);
    }
}

impl WindowHandler for NES {
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        let mut cpu_clockspeed_manager = Instant::now();
        loop {
            if LOUD {
                println!("--------------------");
                println!("Clock = {}", self.cpu.clock);
                self.cpu.print_state();
            }
            let instruction = self.cpu.fetch_instruction(&self.address_space);
            let (instruction, minimum_ticks) = decode_instruction(instruction);
            if LOUD {
                println!("NEXT: {:?}, minimum {:?} ticks", instruction, minimum_ticks);
                println!("--------------------");
            }
            let ticks =
                self.cpu
                    .execute_instruction(instruction, minimum_ticks, &mut self.address_space);
            self.cpu.time_since_last_frame += u64::from(ticks);

            if self.cpu.time_since_last_frame >= CPU_CYCLES_PER_FRAME {
                // TODO: Adjust how frame sleeping works, probably going to be end up sleeping
                // for too long the way it currently is

                // let elapsed_time = cpu_clockspeed_manager.elapsed().as_secs_f64();
                // if elapsed_time < LENGTH_OF_FRAME {
                //     let time_to_sleep =
                //         time::Duration::from_secs_f64(LENGTH_OF_FRAME - elapsed_time);
                //         if LOUD {
                //             println!("---- SLEEPING FOR {:?} ----", time_to_sleep);
                //         }
                //     thread::sleep(time_to_sleep);
                // }
                self.cpu.time_since_last_frame = 0;
                cpu_clockspeed_manager = Instant::now();

                if self.address_space.ppu.ppu_ctrl & PPUCTRL::GEN_NMI.bits()
                    == PPUCTRL::GEN_NMI.bits()
                {
                    let buffer: [(u8, u8, u8); FRAME_BUFFER_SIZE] =
                        self.address_space.ppu.render_frame();

                    // uncomment to pause when entering NMI
                    // println!("---- NMI ----");
                    // let mut line = String::new();
                    // let b1 = std::io::stdin().read_line(&mut line).unwrap();

                    let mut new_buffer: [u8; FRAME_BUFFER_SIZE * 3] = [0; FRAME_BUFFER_SIZE * 3];

                    let mut j = 0;
                    for i in 0..FRAME_BUFFER_SIZE {
                        let (x, y, z) = buffer[i];
                        new_buffer[j] = x;
                        j += 1;
                        new_buffer[j] = y;
                        j += 1;
                        new_buffer[j] = z;
                        j += 1;
                    }

                    let frame = graphics
                        .create_image_from_raw_pixels(
                            ImageDataType::RGB,
                            ImageSmoothingMode::NearestNeighbor,
                            (256, 240),
                            &new_buffer,
                        )
                        .unwrap();

                    graphics.draw_rectangle_image(
                        Rectangle::from_tuples((0.0, 0.0), (1024.0, 960.0)),
                        &frame,
                    );

                    let instruction = Instruction::NMI;
                    let ticks =
                        self.cpu
                            .execute_instruction(instruction, 7, &mut self.address_space);
                    self.cpu.time_since_last_frame += u64::from(ticks);
                    break;
                }
            }
        }
        helper.request_redraw();
    }
}
