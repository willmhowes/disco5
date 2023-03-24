// #[allow(non_camel_case_types)]
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub mod cpu;
pub mod opcode_map;

use crate::computer::cpu::*;
use crate::computer::opcode_map::map_byte_to_instruction;

const MEMORY_SIZE: usize = 0x10000;

#[derive(Debug, Copy, Clone)]
pub enum AddressingMode {
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
pub enum Instruction {
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
    cpu.step_pc();
    memory[index as usize]
}

#[derive(Debug)]
pub struct Computer {
    pub cpu: CPU,
    pub memory: [u8; MEMORY_SIZE],
    pub flags: StatusRegister,
    pub clock: u64,
}

impl Default for Computer {
    fn default() -> Computer {
        Computer {
            cpu: CPU {
                ..Default::default()
            },
            memory: [0; MEMORY_SIZE],
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
        let memory = &mut self.memory[memory_entry_point..];
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
            let instruction = fetch_instruction(&self.memory, &mut self.cpu);
            let (instruction, minimum_ticks) = map_byte_to_instruction(instruction);
            if loud {
                println!("NEXT: {:?}, minimum {:?} ticks", instruction, minimum_ticks);
                println!("--------------------");
            }
            // let mut line = String::new();
            // let b1 = std::io::stdin().read_line(&mut line).unwrap();
            self.process_instruction(instruction, minimum_ticks);

            if exit_condition(self.cpu.pc) == true {
                println!("SUCCESS");
                println!("CLOCK = {}", self.clock);
                println!("PC    = 0x{:0>4x}", self.cpu.pc);
                break;
            }
        }
    }

    fn set_status_nz(&mut self, test_val: u8) {
        self.cpu.p.z = if test_val == 0 { true } else { false };
        // 0x80 = 0b1000_0000 (i.e. a negative number under two-complement encoding)
        self.cpu.p.n = if test_val & 0x80 == 0x80 { true } else { false };
    }

    /// returns the address and whether or not a page was crossed
    fn resolve_address_fetch(&mut self, am: AddressingMode) -> (u16, bool) {
        match am {
            AddressingMode::Accumulator
            | AddressingMode::Implied
            | AddressingMode::Immediate
            | AddressingMode::Relative => {
                panic!("Attempted to fetch an AddressingMode that is intended to be handled on a per instruction basis")
            }
            AddressingMode::Absolute => {
                let lo = fetch_instruction(&self.memory, &mut self.cpu);
                let hi = fetch_instruction(&self.memory, &mut self.cpu);
                let address = (u16::from(hi) << 8) + u16::from(lo);
                (address, false)
            }
            AddressingMode::AbsoluteX => {
                let lo = fetch_instruction(&self.memory, &mut self.cpu);
                let hi = fetch_instruction(&self.memory, &mut self.cpu);
                let address = (u16::from(hi) << 8) + u16::from(lo);
                let address_plus_x = address.wrapping_add(u16::from(self.cpu.x));
                // bitmask the high 8 bits and compare. If they are different,
                // then a page boundary has been crossed
                let boundary_crossed = (address & 0xff00) != (address_plus_x & 0xff00);
                (address_plus_x, boundary_crossed)
            }
            AddressingMode::AbsoluteY => {
                let lo = fetch_instruction(&self.memory, &mut self.cpu);
                let hi = fetch_instruction(&self.memory, &mut self.cpu);
                let address = (u16::from(hi) << 8) + u16::from(lo);
                let address_plus_y = address.wrapping_add(u16::from(self.cpu.y));
                // bitmask the high 8 bits and compare. If they are different,
                // then a page boundary has been crossed
                let boundary_crossed = (address & 0xff00) != (address_plus_y & 0xff00);
                (address_plus_y, boundary_crossed)
            }
            AddressingMode::Indirect => {
                let lo = fetch_instruction(&self.memory, &mut self.cpu);
                let hi = fetch_instruction(&self.memory, &mut self.cpu);
                let address = (u16::from(hi) << 8) + u16::from(lo);

                let lo = self.memory[usize::from(address)];
                // The indirect jump instruction does not increment the page
                // address when the indirect pointer crosses a page boundary.
                // JMP ($xxFF) will fetch the address from $xxFF and $xx00.
                // https://www.pagetable.com/c64ref/6502/?tab=3
                let address = if address & 0x00ff == 0x00ff {
                    address & 0xff00
                } else {
                    address + 1
                };
                let hi = self.memory[usize::from(address)];
                let address = (u16::from(hi) << 8) + u16::from(lo);
                (address, false)
            }
            AddressingMode::IndirectX => {
                let zpg = fetch_instruction(&self.memory, &mut self.cpu);
                let lo = zpg.wrapping_add(self.cpu.x);
                let hi: u8 = 0x00;
                let address = (u16::from(hi) << 8) + u16::from(lo);

                let lo = self.memory[usize::from(address)];
                // IndirectX wraps around the zeropage
                let hi = self.memory[usize::from(address + 1) % 256];
                let address = (u16::from(hi) << 8) + u16::from(lo);
                (address, false)
            }
            AddressingMode::IndirectY => {
                let lo = fetch_instruction(&self.memory, &mut self.cpu);
                let hi: u8 = 0x00;
                let address = (u16::from(hi) << 8) + u16::from(lo);

                let lo = self.memory[usize::from(address)];
                let hi = self.memory[usize::from(address.wrapping_add(1))];
                let address = (u16::from(hi) << 8) + u16::from(lo);
                let address_plus_y = address.wrapping_add(u16::from(self.cpu.y));
                // bitmask the high 8 bits and compare. If they are different,
                // then a page boundary has been crossed
                let boundary_crossed = (address & 0xff00) != (address_plus_y & 0xff00);
                (address_plus_y, boundary_crossed)
            }
            AddressingMode::ZeroPage => {
                let lo = fetch_instruction(&self.memory, &mut self.cpu);
                let hi: u8 = 0x00;
                let address = (u16::from(hi) << 8) + u16::from(lo);
                (address, false)
            }
            AddressingMode::ZeroPageX => {
                let zpg = fetch_instruction(&self.memory, &mut self.cpu);
                let lo = zpg.wrapping_add(self.cpu.x);
                let hi: u8 = 0x00;
                let address = (u16::from(hi) << 8) + u16::from(lo);
                (address, false)
            }
            AddressingMode::ZeroPageY => {
                let zpg = fetch_instruction(&self.memory, &mut self.cpu);
                let lo = zpg.wrapping_add(self.cpu.y);
                let hi: u8 = 0x00;
                let address = (u16::from(hi) << 8) + u16::from(lo);
                (address, false)
            }
        }
    }

    fn adc_logic(&mut self, addend_1: u8) {
        let addend_2 = self.cpu.a;
        let carry = if self.cpu.p.c == true { 1 } else { 0 };
        let result = addend_1.wrapping_add(addend_2).wrapping_add(carry);
        self.cpu.a = result;
        self.cpu.p.c = if u16::from(addend_1) + u16::from(addend_2) + u16::from(carry) > 255 {
            true
        } else {
            false
        };
        self.cpu.p.v = if (addend_1 ^ result) & (addend_2 ^ result) & 0x80 == 0x00 {
            false
        } else {
            true
        };
        self.set_status_nz(self.cpu.a);
    }

    /// returns whether or not a page was crossed
    fn branch_if(&mut self, condition: bool) -> bool {
        let offset = fetch_instruction(&self.memory, &mut self.cpu);
        let offset: i16 = i16::from(offset as i8);
        let mut negative = false;
        if offset.is_negative() {
            negative = true;
        }
        let offset = offset.abs();
        let offset = offset as u16;
        let mut pc_update: u16 = self.cpu.pc;
        if condition && negative == false {
            pc_update += u16::from(offset);
        } else if condition && negative == true {
            pc_update -= u16::from(offset);
        }
        // bitmask the high 8 bits and compare. If they are different,
        // then a page boundary has been crossed
        let boundary_crossed = (self.cpu.pc & 0xff00) != (pc_update & 0xff00);
        self.cpu.pc = pc_update;
        boundary_crossed
    }

    fn push_stack(&mut self, byte: u8) {
        let address = (u16::from(0x01_u8) << 8) + u16::from(self.cpu.sp);
        self.memory[usize::from(address)] = byte;
        self.cpu.sp = self.cpu.sp.wrapping_sub(1);
    }

    fn pop_stack(&mut self) -> u8 {
        self.cpu.sp = self.cpu.sp.wrapping_add(1);
        let address = (u16::from(0x01_u8) << 8) + u16::from(self.cpu.sp);
        self.memory[usize::from(address)]
    }

    fn process_instruction(&mut self, instruction: Instruction, minimum_ticks: u8) {
        let mut num_ticks: u8 = minimum_ticks;
        match instruction {
            Instruction::ADC(am) => match am {
                AddressingMode::Absolute
                | AddressingMode::AbsoluteX
                | AddressingMode::AbsoluteY
                | AddressingMode::IndirectX
                | AddressingMode::IndirectY
                | AddressingMode::ZeroPage
                | AddressingMode::ZeroPageX => {
                    let (address, boundary_crossed) = self.resolve_address_fetch(am);
                    let addend = self.memory[usize::from(address)];
                    self.adc_logic(addend);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                }
                AddressingMode::Immediate => {
                    let immediate = fetch_instruction(&self.memory, &mut self.cpu);
                    self.adc_logic(immediate);
                }
                _ => {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            },
            Instruction::AND(am) => {
                match am {
                    AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::AbsoluteY
                    | AddressingMode::IndirectX
                    | AddressingMode::IndirectY
                    | AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am);
                        let value = self.memory[usize::from(address)];
                        self.cpu.a = self.cpu.a & value;
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                    }
                    AddressingMode::Immediate => {
                        let immediate = fetch_instruction(&self.memory, &mut self.cpu);
                        self.cpu.a = self.cpu.a & immediate;
                    }
                    _ => {
                        panic!("Attempted to execute instruction with invalid AddressingMode");
                    }
                };
                self.set_status_nz(self.cpu.a);
            }
            Instruction::ASL(am) => {
                let shift_result: u8;
                match am {
                    AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am);
                        let value = self.memory[usize::from(address)];
                        self.cpu.p.c = if value & 0x80 == 0x80 { true } else { false };
                        shift_result = self.cpu.a << 1;
                        self.memory[usize::from(address)] = shift_result;
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                    }
                    AddressingMode::Accumulator => {
                        self.cpu.p.c = if self.cpu.a & 0x80 == 0x80 {
                            true
                        } else {
                            false
                        };
                        self.cpu.a = self.cpu.a << 1;
                        shift_result = self.cpu.a;
                    }
                    _ => {
                        panic!("Attempted to execute instruction with invalid AddressingMode");
                    }
                };
                self.set_status_nz(shift_result);
            }
            Instruction::BCC(am) => {
                if let AddressingMode::Relative = am {
                    let condition = self.cpu.p.c == false;
                    let boundary_crossed = self.branch_if(condition);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    if condition == true {
                        num_ticks += 1;
                    }
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::BCS(am) => {
                if let AddressingMode::Relative = am {
                    let condition = self.cpu.p.c == true;
                    let boundary_crossed = self.branch_if(condition);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    if condition == true {
                        num_ticks += 1;
                    }
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::BEQ(am) => {
                if let AddressingMode::Relative = am {
                    let condition = self.cpu.p.z == true;
                    let boundary_crossed = self.branch_if(condition);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    if condition == true {
                        num_ticks += 1;
                    }
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::BIT(am) => {
                match am {
                    AddressingMode::Absolute | AddressingMode::ZeroPage => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am);
                        let value = self.memory[usize::from(address)];
                        let result = self.cpu.a & value;
                        // v register <- bit 6 of value
                        self.cpu.p.v = if value & 0x40 == 0x40 { true } else { false };
                        // n register <- bit 7 of value
                        self.cpu.p.n = if value & 0x80 == 0x80 { true } else { false };
                        self.cpu.p.z = if result == 0 { true } else { false };
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                    }
                    _ => {
                        panic!("Attempted to execute instruction with invalid AddressingMode");
                    }
                };
            }
            Instruction::BMI(am) => {
                if let AddressingMode::Relative = am {
                    let condition = self.cpu.p.n == true;
                    let boundary_crossed = self.branch_if(condition);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    if condition == true {
                        num_ticks += 1;
                    }
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::BNE(am) => {
                if let AddressingMode::Relative = am {
                    let condition = self.cpu.p.z == false;
                    let boundary_crossed = self.branch_if(condition);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    if condition == true {
                        num_ticks += 1;
                    }
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::BPL(am) => {
                if let AddressingMode::Relative = am {
                    let condition = self.cpu.p.n == false;
                    let boundary_crossed = self.branch_if(condition);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    if condition == true {
                        num_ticks += 1;
                    }
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::BRK(am) => {
                if let AddressingMode::Implied = am {
                    // BRK stores the location of the pc+2 in the stack, even though
                    // BRK is a 1 byte instruction. Our program increments PC
                    // when reading an instruction so the PC is already incremented by
                    // one. Thus, we add store pc+1 in the stack, which is equal to the
                    // third byte as intended.
                    let to_be_pushed = self.cpu.pc + 1;
                    let lo = to_be_pushed as u8;
                    let hi = (to_be_pushed >> 8) as u8;
                    self.push_stack(hi);
                    self.push_stack(lo);

                    // store self.cpu.p on stack with a set b flag
                    let b: u8 = 0b0001_0000;
                    let p = self.cpu.p.to_byte() | b;

                    self.push_stack(p);

                    // fetch address of interrupt handler
                    let lo = self.memory[0xfffe];
                    let hi = self.memory[0xffff];
                    let address = (u16::from(hi) << 8) + u16::from(lo);
                    self.cpu.pc = address;

                    // set interrupt disable flag
                    self.cpu.p.i = true;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::BVC(am) => {
                if let AddressingMode::Relative = am {
                    let condition = self.cpu.p.v == false;
                    let boundary_crossed = self.branch_if(condition);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    if condition == true {
                        num_ticks += 1;
                    }
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::BVS(am) => {
                if let AddressingMode::Relative = am {
                    let condition = self.cpu.p.v == true;
                    let boundary_crossed = self.branch_if(condition);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    if condition == true {
                        num_ticks += 1;
                    }
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::CLC(am) => {
                if let AddressingMode::Implied = am {
                    self.cpu.p.c = false;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::CLD(am) => {
                if let AddressingMode::Implied = am {
                    self.cpu.p.d = false;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::CLI(am) => {
                if let AddressingMode::Implied = am {
                    self.cpu.p.i = false;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::CLV(am) => {
                if let AddressingMode::Implied = am {
                    self.cpu.p.v = false;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::CMP(am) => {
                let test_val: u8;
                match am {
                    AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::AbsoluteY
                    | AddressingMode::IndirectX
                    | AddressingMode::IndirectY
                    | AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am);
                        test_val = self.memory[usize::from(address)];
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                    }
                    AddressingMode::Immediate => {
                        test_val = fetch_instruction(&self.memory, &mut self.cpu);
                    }
                    _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
                }
                self.cpu.p.c = if self.cpu.a >= test_val { true } else { false };
                self.set_status_nz(self.cpu.a.wrapping_sub(test_val));
            }
            Instruction::CPX(am) => {
                let test_val: u8;
                match am {
                    AddressingMode::Absolute | AddressingMode::ZeroPage => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        test_val = self.memory[usize::from(address)];
                    }
                    AddressingMode::Immediate => {
                        test_val = fetch_instruction(&self.memory, &mut self.cpu);
                    }
                    _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
                }
                self.cpu.p.c = if self.cpu.x >= test_val { true } else { false };
                self.set_status_nz(self.cpu.x.wrapping_sub(test_val));
            }
            Instruction::CPY(am) => {
                let test_val: u8;
                match am {
                    AddressingMode::Absolute | AddressingMode::ZeroPage => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        test_val = self.memory[usize::from(address)];
                    }
                    AddressingMode::Immediate => {
                        test_val = fetch_instruction(&self.memory, &mut self.cpu);
                    }
                    _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
                }
                self.cpu.p.c = if self.cpu.y >= test_val { true } else { false };
                self.set_status_nz(self.cpu.y.wrapping_sub(test_val));
            }
            Instruction::DEC(am) => match am {
                AddressingMode::Absolute
                | AddressingMode::AbsoluteX
                | AddressingMode::ZeroPage
                | AddressingMode::ZeroPageX => {
                    let (address, boundary_crossed) = self.resolve_address_fetch(am);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    let mut to_modify = self.memory[usize::from(address)];
                    to_modify = to_modify.wrapping_sub(1);
                    self.memory[usize::from(address)] = to_modify;
                    self.set_status_nz(to_modify);
                }
                _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
            },
            Instruction::DEX(am) => {
                if let AddressingMode::Implied = am {
                    self.cpu.x = self.cpu.x.wrapping_sub(1);
                    self.set_status_nz(self.cpu.x);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::DEY(am) => {
                if let AddressingMode::Implied = am {
                    self.cpu.y = self.cpu.y.wrapping_sub(1);
                    self.set_status_nz(self.cpu.y);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::EOR(am) => {
                match am {
                    AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::AbsoluteY
                    | AddressingMode::IndirectX
                    | AddressingMode::IndirectY
                    | AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        let value = self.memory[usize::from(address)];
                        self.cpu.a = self.cpu.a ^ value;
                    }
                    AddressingMode::Immediate => {
                        let immediate = fetch_instruction(&self.memory, &mut self.cpu);
                        self.cpu.a = self.cpu.a ^ immediate;
                    }
                    _ => {
                        panic!("Attempted to execute instruction with invalid AddressingMode");
                    }
                };
                self.set_status_nz(self.cpu.a);
            }
            Instruction::INC(am) => match am {
                AddressingMode::Absolute
                | AddressingMode::AbsoluteX
                | AddressingMode::ZeroPage
                | AddressingMode::ZeroPageX => {
                    let (address, boundary_crossed) = self.resolve_address_fetch(am);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    let mut to_modify = self.memory[usize::from(address)];
                    to_modify = to_modify.wrapping_add(1);
                    self.memory[usize::from(address)] = to_modify;
                    self.set_status_nz(to_modify);
                }
                _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
            },
            Instruction::INX(am) => {
                if let AddressingMode::Implied = am {
                    self.cpu.x = self.cpu.x.wrapping_add(1);
                    self.set_status_nz(self.cpu.x);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::INY(am) => {
                if let AddressingMode::Implied = am {
                    self.cpu.y = self.cpu.y.wrapping_add(1);
                    self.set_status_nz(self.cpu.y);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::JMP(am) => {
                if let AddressingMode::Absolute | AddressingMode::Indirect = am {
                    let (address, boundary_crossed) = self.resolve_address_fetch(am);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    self.cpu.pc = address;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::JSR(am) => {
                if let AddressingMode::Absolute = am {
                    // JSR stores the location of the last byte in the instruction.
                    // JSR is a 3 byte instruction, and our program increments PC
                    // when reading an instruction so the PC is already pointing at
                    // the second byte. Thus, we add store pc+1 in the stack, which is
                    // equal to the third byte as intended.
                    let to_be_pushed = self.cpu.pc + 1;
                    let (address, boundary_crossed) = self.resolve_address_fetch(am);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    let lo = to_be_pushed as u8;
                    let hi = (to_be_pushed >> 8) as u8;
                    self.push_stack(hi);
                    self.push_stack(lo);
                    self.cpu.pc = address;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::LDA(am) => {
                match am {
                    AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::AbsoluteY
                    | AddressingMode::IndirectX
                    | AddressingMode::IndirectY
                    | AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        self.cpu.a = self.memory[usize::from(address)];
                    }
                    AddressingMode::Immediate => {
                        self.cpu.a = fetch_instruction(&self.memory, &mut self.cpu);
                    }
                    _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
                }
                self.set_status_nz(self.cpu.a);
            }
            Instruction::LDX(am) => {
                match am {
                    AddressingMode::Absolute
                    | AddressingMode::AbsoluteY
                    | AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageY => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        self.cpu.x = self.memory[usize::from(address)];
                    }
                    AddressingMode::Immediate => {
                        self.cpu.x = fetch_instruction(&self.memory, &mut self.cpu);
                    }
                    _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
                }
                self.set_status_nz(self.cpu.x);
            }
            Instruction::LDY(am) => {
                match am {
                    AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        self.cpu.y = self.memory[usize::from(address)];
                    }
                    AddressingMode::Immediate => {
                        self.cpu.y = fetch_instruction(&self.memory, &mut self.cpu);
                    }
                    _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
                }
                self.set_status_nz(self.cpu.y);
            }
            Instruction::LSR(am) => {
                let shift_result: u8;
                match am {
                    AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        let value = self.memory[usize::from(address)];
                        self.cpu.p.c = if value & 0x01 == 0x01 { true } else { false };
                        shift_result = self.cpu.a >> 1;
                        self.memory[usize::from(address)] = shift_result;
                    }
                    AddressingMode::Accumulator => {
                        self.cpu.p.c = if self.cpu.a & 0x01 == 0x01 {
                            true
                        } else {
                            false
                        };
                        self.cpu.a = self.cpu.a >> 1;
                        shift_result = self.cpu.a;
                    }
                    _ => {
                        panic!("Attempted to execute instruction with invalid AddressingMode");
                    }
                };
                self.set_status_nz(shift_result);
            }
            Instruction::NOP(am) => {
                if let AddressingMode::Implied = am {
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::ORA(am) => {
                match am {
                    AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::AbsoluteY
                    | AddressingMode::IndirectX
                    | AddressingMode::IndirectY
                    | AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        let value = self.memory[usize::from(address)];
                        self.cpu.a = self.cpu.a | value;
                    }
                    AddressingMode::Immediate => {
                        let immediate = fetch_instruction(&self.memory, &mut self.cpu);
                        self.cpu.a = self.cpu.a | immediate;
                    }
                    _ => {
                        panic!("Attempted to execute instruction with invalid AddressingMode");
                    }
                };
                self.set_status_nz(self.cpu.a);
            }
            Instruction::PHA(am) => {
                if let AddressingMode::Implied = am {
                    self.push_stack(self.cpu.a);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::PHP(am) => {
                if let AddressingMode::Implied = am {
                    let b: u8 = 0b0001_0000;
                    let p = self.cpu.p.to_byte() | b;
                    self.push_stack(p);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::PLA(am) => {
                if let AddressingMode::Implied = am {
                    self.cpu.a = self.pop_stack();
                    self.set_status_nz(self.cpu.a);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::PLP(am) => {
                if let AddressingMode::Implied = am {
                    // bits 4 and 5 are ignored
                    let p = self.pop_stack() & 0b1100_1111;
                    self.cpu.p.set_from_byte(p)
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::ROL(am) => {
                let shift_result: u8;
                match am {
                    AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        let mut value = self.memory[usize::from(address)];
                        let tail = self.cpu.p.c;
                        self.cpu.p.c = if value & 0x80 == 0x80 { true } else { false };
                        value = self.cpu.a << 1;
                        shift_result = if tail == true { value | 0x01 } else { value };
                        self.memory[usize::from(address)] = shift_result;
                    }
                    AddressingMode::Accumulator => {
                        let tail = self.cpu.p.c;
                        self.cpu.p.c = if self.cpu.a & 0x80 == 0x80 {
                            true
                        } else {
                            false
                        };
                        self.cpu.a = self.cpu.a << 1;
                        self.cpu.a = if tail == true {
                            self.cpu.a | 0x01
                        } else {
                            self.cpu.a
                        };
                        shift_result = self.cpu.a;
                    }
                    _ => {
                        panic!("Attempted to execute instruction with invalid AddressingMode");
                    }
                };
                self.set_status_nz(shift_result);
            }
            Instruction::ROR(am) => {
                let shift_result: u8;
                match am {
                    AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        let mut value = self.memory[usize::from(address)];
                        let tail = self.cpu.p.c;
                        self.cpu.p.c = if value & 0x01 == 0x01 { true } else { false };
                        value = self.cpu.a >> 1;
                        shift_result = if tail == true { value | 0x80 } else { value };
                        self.memory[usize::from(address)] = shift_result;
                    }
                    AddressingMode::Accumulator => {
                        let tail = self.cpu.p.c;
                        self.cpu.p.c = if self.cpu.a & 0x01 == 0x01 {
                            true
                        } else {
                            false
                        };
                        self.cpu.a = self.cpu.a >> 1;
                        self.cpu.a = if tail == true {
                            self.cpu.a | 0x80
                        } else {
                            self.cpu.a
                        };
                        shift_result = self.cpu.a;
                    }
                    _ => {
                        panic!("Attempted to execute instruction with invalid AddressingMode");
                    }
                };
                self.set_status_nz(shift_result);
            }
            Instruction::RTI(am) => {
                if let AddressingMode::Implied = am {
                    // bits 4 and 5 are ignored
                    let p = self.pop_stack() & 0b1100_1111;
                    self.cpu.p.set_from_byte(p);

                    let lo = self.pop_stack();
                    let hi = self.pop_stack();
                    let address = (u16::from(hi) << 8) + u16::from(lo);
                    self.cpu.pc = address;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::RTS(am) => {
                if let AddressingMode::Implied = am {
                    let lo = self.pop_stack();
                    let hi = self.pop_stack();
                    let address = (u16::from(hi) << 8) + u16::from(lo);
                    self.cpu.pc = address.wrapping_add(1);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::SBC(am) => match am {
                AddressingMode::Absolute
                | AddressingMode::AbsoluteX
                | AddressingMode::AbsoluteY
                | AddressingMode::IndirectX
                | AddressingMode::IndirectY
                | AddressingMode::ZeroPage
                | AddressingMode::ZeroPageX => {
                    let (address, boundary_crossed) = self.resolve_address_fetch(am);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    let complement = !self.memory[usize::from(address)];
                    self.adc_logic(complement);
                }
                AddressingMode::Immediate => {
                    let immediate = fetch_instruction(&self.memory, &mut self.cpu);
                    self.adc_logic(!(immediate as u8));
                }
                _ => {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            },
            Instruction::SEC(am) => {
                if let AddressingMode::Implied = am {
                    self.cpu.p.c = true;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::SED(am) => {
                if let AddressingMode::Implied = am {
                    self.cpu.p.d = true;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::SEI(am) => {
                if let AddressingMode::Implied = am {
                    self.cpu.p.i = true;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::STA(am) => match am {
                AddressingMode::Absolute
                | AddressingMode::AbsoluteX
                | AddressingMode::AbsoluteY
                | AddressingMode::IndirectX
                | AddressingMode::IndirectY
                | AddressingMode::ZeroPage
                | AddressingMode::ZeroPageX => {
                    let (address, boundary_crossed) = self.resolve_address_fetch(am);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    self.memory[usize::from(address)] = self.cpu.a;
                }
                _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
            },
            Instruction::STX(am) => match am {
                AddressingMode::Absolute | AddressingMode::ZeroPage | AddressingMode::ZeroPageY => {
                    let (address, boundary_crossed) = self.resolve_address_fetch(am);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    self.memory[usize::from(address)] = self.cpu.x;
                }
                _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
            },
            Instruction::STY(am) => match am {
                AddressingMode::Absolute | AddressingMode::ZeroPage | AddressingMode::ZeroPageX => {
                    let (address, boundary_crossed) = self.resolve_address_fetch(am);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    self.memory[usize::from(address)] = self.cpu.y;
                }
                _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
            },
            Instruction::TAX(am) => {
                if let AddressingMode::Implied = am {
                    self.cpu.x = self.cpu.a;
                    self.set_status_nz(self.cpu.x);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::TAY(am) => {
                if let AddressingMode::Implied = am {
                    self.cpu.y = self.cpu.a;
                    self.set_status_nz(self.cpu.y);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::TSX(am) => {
                if let AddressingMode::Implied = am {
                    self.cpu.x = self.cpu.sp;
                    self.set_status_nz(self.cpu.x);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::TXA(am) => {
                if let AddressingMode::Implied = am {
                    self.cpu.a = self.cpu.x;
                    self.set_status_nz(self.cpu.a);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::TXS(am) => {
                if let AddressingMode::Implied = am {
                    self.cpu.sp = self.cpu.x;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::TYA(am) => {
                if let AddressingMode::Implied = am {
                    self.cpu.a = self.cpu.y;
                    self.set_status_nz(self.cpu.a);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::Invalid => panic!("Attempted to execute invalid instruction"),
        }
        self.tick(num_ticks);
    }
}
