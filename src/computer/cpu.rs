use crate::computer::bus::Bus;
use crate::computer::cpu_structs::{AddressingMode, Instruction};

/// Type for storing CPU registers as fields
#[derive(Copy, Clone, Default, Debug)]
pub struct CPU {
    /// accumulator register
    pub a: u8,
    /// index register x
    pub x: u8,
    /// index register y
    pub y: u8,
    /// stack pointer register
    pub sp: u8,
    /// program counter register
    pub pc: u16,
    /// status register
    pub p: StatusRegister,
    /// read/write pin. low is write
    pub rw: ReadWrite,
    /// interrupt pin
    pub irq: bool,
    /// non-maskable interrupt pin
    pub nmi: bool,
    pub clock: u64,
}

impl CPU {
    pub fn tick(&mut self, num: u8) {
        self.clock += u64::from(num);
    }

    pub fn print_state(&self) {
        // println!("--------------------");
        println!("A  = 0b{:0>8b}, X = {}, Y = {}", self.a, self.x, self.y);
        println!("P  =   NV_BDIZC");
        println!("     0b{:0>8b}", self.p.to_byte());
        println!("PC = 0x{:0>4x}", self.pc);
        println!("SP = {}", self.sp);
        // println!("--------------------");
    }

    /// steps pc to next position
    pub fn step_pc(&mut self) {
        self.pc = self.pc + 1;
    }

    /// loads instruction at address of pc, increments pc
    pub fn fetch_instruction(&mut self, memory: &Bus) -> u8 {
        let index = self.pc;
        self.step_pc();
        memory[index as usize]
    }

    /// returns the address and whether or not a page was crossed
    pub fn resolve_address_fetch(&mut self, am: AddressingMode, memory: &Bus) -> (u16, bool) {
        let output = {
            match am {
                AddressingMode::Accumulator
                | AddressingMode::Implied
                | AddressingMode::Immediate
                | AddressingMode::Relative => {
                    panic!("Attempted to fetch an AddressingMode that is intended to be handled on a per instruction basis")
                }
                AddressingMode::Absolute => {
                    let lo = self.fetch_instruction(memory);
                    let hi = self.fetch_instruction(memory);
                    let address = (u16::from(hi) << 8) + u16::from(lo);
                    (address, false)
                }
                AddressingMode::AbsoluteX => {
                    let lo = self.fetch_instruction(memory);
                    let hi = self.fetch_instruction(memory);
                    let address = (u16::from(hi) << 8) + u16::from(lo);
                    let address_plus_x = address.wrapping_add(u16::from(self.x));
                    // bitmask the high 8 bits and compare. If they are different,
                    // then a page boundary has been crossed
                    let boundary_crossed = (address & 0xff00) != (address_plus_x & 0xff00);
                    (address_plus_x, boundary_crossed)
                }
                AddressingMode::AbsoluteY => {
                    let lo = self.fetch_instruction(memory);
                    let hi = self.fetch_instruction(memory);
                    let address = (u16::from(hi) << 8) + u16::from(lo);
                    let address_plus_y = address.wrapping_add(u16::from(self.y));
                    // bitmask the high 8 bits and compare. If they are different,
                    // then a page boundary has been crossed
                    let boundary_crossed = (address & 0xff00) != (address_plus_y & 0xff00);
                    (address_plus_y, boundary_crossed)
                }
                AddressingMode::Indirect => {
                    let lo = self.fetch_instruction(memory);
                    let hi = self.fetch_instruction(memory);
                    let address = (u16::from(hi) << 8) + u16::from(lo);

                    let lo = memory[usize::from(address)];
                    // The indirect jump instruction does not increment the page
                    // address when the indirect pointer crosses a page boundary.
                    // JMP ($xxFF) will fetch the address from $xxFF and $xx00.
                    // https://www.pagetable.com/c64ref/6502/?tab=3
                    let address = if address & 0x00ff == 0x00ff {
                        address & 0xff00
                    } else {
                        address + 1
                    };
                    let hi = memory[usize::from(address)];
                    let address = (u16::from(hi) << 8) + u16::from(lo);
                    (address, false)
                }
                AddressingMode::IndirectX => {
                    let zpg = self.fetch_instruction(memory);
                    let lo = zpg.wrapping_add(self.x);
                    let hi: u8 = 0x00;
                    let address = (u16::from(hi) << 8) + u16::from(lo);

                    let lo = memory[usize::from(address)];
                    // IndirectX wraps around the zeropage
                    let hi = memory[usize::from(address + 1) % 256];
                    let address = (u16::from(hi) << 8) + u16::from(lo);
                    (address, false)
                }
                AddressingMode::IndirectY => {
                    let lo = self.fetch_instruction(memory);
                    let hi: u8 = 0x00;
                    let address = (u16::from(hi) << 8) + u16::from(lo);

                    let lo = memory[usize::from(address)];
                    let hi = memory[usize::from(address.wrapping_add(1))];
                    let address = (u16::from(hi) << 8) + u16::from(lo);
                    let address_plus_y = address.wrapping_add(u16::from(self.y));
                    // bitmask the high 8 bits and compare. If they are different,
                    // then a page boundary has been crossed
                    let boundary_crossed = (address & 0xff00) != (address_plus_y & 0xff00);
                    (address_plus_y, boundary_crossed)
                }
                AddressingMode::ZeroPage => {
                    let lo = self.fetch_instruction(memory);
                    let hi: u8 = 0x00;
                    let address = (u16::from(hi) << 8) + u16::from(lo);
                    (address, false)
                }
                AddressingMode::ZeroPageX => {
                    let zpg = self.fetch_instruction(memory);
                    let lo = zpg.wrapping_add(self.x);
                    let hi: u8 = 0x00;
                    let address = (u16::from(hi) << 8) + u16::from(lo);
                    (address, false)
                }
                AddressingMode::ZeroPageY => {
                    let zpg = self.fetch_instruction(memory);
                    let lo = zpg.wrapping_add(self.y);
                    let hi: u8 = 0x00;
                    let address = (u16::from(hi) << 8) + u16::from(lo);
                    (address, false)
                }
            }
        };
        // if     output.0 == 0x2000
        //     || output.0 == 0x2001
        //     || output.0 == 0x2002
        //     || output.0 == 0x2003
        //     || output.0 == 0x2004
        //     || output.0 == 0x2005
        //     || output.0 == 0x2006
        //     || output.0 == 0x2007
        // {
        //     println!("HIT PPU REGISTER = 0x{:x}", output.0);
        //     let mut line = String::new();
        //     let b1 = std::io::stdin().read_line(&mut line).unwrap();
        // }
        output
    }

    fn set_status_nz(&mut self, test_val: u8) {
        self.p.z = if test_val == 0 { true } else { false };
        // 0x80 = 0b1000_0000 (i.e. a negative number under two-complement encoding)
        self.p.n = if test_val & 0x80 == 0x80 { true } else { false };
    }

    fn adc_logic(&mut self, addend_1: u8) {
        let addend_2 = self.a;
        let carry = if self.p.c == true { 1 } else { 0 };
        let result = addend_1.wrapping_add(addend_2).wrapping_add(carry);
        self.a = result;
        self.p.c = if u16::from(addend_1) + u16::from(addend_2) + u16::from(carry) > 255 {
            true
        } else {
            false
        };
        self.p.v = if (addend_1 ^ result) & (addend_2 ^ result) & 0x80 == 0x00 {
            false
        } else {
            true
        };
        self.set_status_nz(self.a);
    }

    /// returns whether or not a page was crossed
    fn branch_if(&mut self, condition: bool, memory: &Bus) -> bool {
        let offset = self.fetch_instruction(memory);
        let offset: i16 = i16::from(offset as i8);
        let mut negative = false;
        if offset.is_negative() {
            negative = true;
        }
        let offset = offset.abs();
        let offset = offset as u16;
        let mut pc_update: u16 = self.pc;
        if condition && negative == false {
            pc_update += u16::from(offset);
        } else if condition && negative == true {
            pc_update -= u16::from(offset);
        }
        // bitmask the high 8 bits and compare. If they are different,
        // then a page boundary has been crossed
        let boundary_crossed = (self.pc & 0xff00) != (pc_update & 0xff00);
        self.pc = pc_update;
        boundary_crossed
    }

    fn push_stack(&mut self, byte: u8, memory: &mut Bus) {
        let address = (u16::from(0x01_u8) << 8) + u16::from(self.sp);
        memory[usize::from(address)] = byte;
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pop_stack(&mut self, memory: &Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        let address = (u16::from(0x01_u8) << 8) + u16::from(self.sp);
        memory[usize::from(address)]
    }

    pub fn process_instruction(
        &mut self,
        instruction: Instruction,
        minimum_ticks: u8,
        memory: &mut Bus,
    ) -> u8 {
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
                    let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                    let addend = memory[usize::from(address)];
                    self.adc_logic(addend);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                }
                AddressingMode::Immediate => {
                    let immediate = self.fetch_instruction(&memory);
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
                        let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                        let value = memory[usize::from(address)];
                        self.a = self.a & value;
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                    }
                    AddressingMode::Immediate => {
                        let immediate = self.fetch_instruction(memory);
                        self.a = self.a & immediate;
                    }
                    _ => {
                        panic!("Attempted to execute instruction with invalid AddressingMode");
                    }
                };
                self.set_status_nz(self.a);
            }
            Instruction::ASL(am) => {
                let shift_result: u8;
                match am {
                    AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                        let value = memory[usize::from(address)];
                        self.p.c = if value & 0x80 == 0x80 { true } else { false };
                        shift_result = self.a << 1;
                        memory[usize::from(address)] = shift_result;
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                    }
                    AddressingMode::Accumulator => {
                        self.p.c = if self.a & 0x80 == 0x80 { true } else { false };
                        self.a = self.a << 1;
                        shift_result = self.a;
                    }
                    _ => {
                        panic!("Attempted to execute instruction with invalid AddressingMode");
                    }
                };
                self.set_status_nz(shift_result);
            }
            Instruction::BCC(am) => {
                if let AddressingMode::Relative = am {
                    let condition = self.p.c == false;
                    let boundary_crossed = self.branch_if(condition, memory);
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
                    let condition = self.p.c == true;
                    let boundary_crossed = self.branch_if(condition, memory);
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
                    let condition = self.p.z == true;
                    let boundary_crossed = self.branch_if(condition, memory);
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
                        let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                        let value = memory[usize::from(address)];
                        let result = self.a & value;
                        // v register <- bit 6 of value
                        self.p.v = if value & 0x40 == 0x40 { true } else { false };
                        // n register <- bit 7 of value
                        self.p.n = if value & 0x80 == 0x80 { true } else { false };
                        self.p.z = if result == 0 { true } else { false };
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
                    let condition = self.p.n == true;
                    let boundary_crossed = self.branch_if(condition, memory);
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
                    let condition = self.p.z == false;
                    let boundary_crossed = self.branch_if(condition, memory);
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
                    let condition = self.p.n == false;
                    let boundary_crossed = self.branch_if(condition, memory);
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
                    let to_be_pushed = self.pc + 1;
                    let lo = to_be_pushed as u8;
                    let hi = (to_be_pushed >> 8) as u8;
                    self.push_stack(hi, memory);
                    self.push_stack(lo, memory);

                    // store self.p on stack with a set b flag
                    let b: u8 = 0b0001_0000;
                    let p = self.p.to_byte() | b;

                    self.push_stack(p, memory);

                    // fetch address of interrupt handler
                    let lo = memory[0xfffe];
                    let hi = memory[0xffff];
                    let address = (u16::from(hi) << 8) + u16::from(lo);
                    self.pc = address;

                    // set interrupt disable flag
                    self.p.i = true;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::BVC(am) => {
                if let AddressingMode::Relative = am {
                    let condition = self.p.v == false;
                    let boundary_crossed = self.branch_if(condition, memory);
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
                    let condition = self.p.v == true;
                    let boundary_crossed = self.branch_if(condition, memory);
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
                    self.p.c = false;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::CLD(am) => {
                if let AddressingMode::Implied = am {
                    self.p.d = false;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::CLI(am) => {
                if let AddressingMode::Implied = am {
                    self.p.i = false;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::CLV(am) => {
                if let AddressingMode::Implied = am {
                    self.p.v = false;
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
                        let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                        test_val = memory[usize::from(address)];
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                    }
                    AddressingMode::Immediate => {
                        test_val = self.fetch_instruction(memory);
                    }
                    _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
                }
                self.p.c = if self.a >= test_val { true } else { false };
                self.set_status_nz(self.a.wrapping_sub(test_val));
            }
            Instruction::CPX(am) => {
                let test_val: u8;
                match am {
                    AddressingMode::Absolute | AddressingMode::ZeroPage => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        test_val = memory[usize::from(address)];
                    }
                    AddressingMode::Immediate => {
                        test_val = self.fetch_instruction(memory);
                    }
                    _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
                }
                self.p.c = if self.x >= test_val { true } else { false };
                self.set_status_nz(self.x.wrapping_sub(test_val));
            }
            Instruction::CPY(am) => {
                let test_val: u8;
                match am {
                    AddressingMode::Absolute | AddressingMode::ZeroPage => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        test_val = memory[usize::from(address)];
                    }
                    AddressingMode::Immediate => {
                        test_val = self.fetch_instruction(memory);
                    }
                    _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
                }
                self.p.c = if self.y >= test_val { true } else { false };
                self.set_status_nz(self.y.wrapping_sub(test_val));
            }
            Instruction::DEC(am) => match am {
                AddressingMode::Absolute
                | AddressingMode::AbsoluteX
                | AddressingMode::ZeroPage
                | AddressingMode::ZeroPageX => {
                    let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    let mut to_modify = memory[usize::from(address)];
                    to_modify = to_modify.wrapping_sub(1);
                    memory[usize::from(address)] = to_modify;
                    self.set_status_nz(to_modify);
                }
                _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
            },
            Instruction::DEX(am) => {
                if let AddressingMode::Implied = am {
                    self.x = self.x.wrapping_sub(1);
                    self.set_status_nz(self.x);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::DEY(am) => {
                if let AddressingMode::Implied = am {
                    self.y = self.y.wrapping_sub(1);
                    self.set_status_nz(self.y);
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
                        let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        let value = memory[usize::from(address)];
                        self.a = self.a ^ value;
                    }
                    AddressingMode::Immediate => {
                        let immediate = self.fetch_instruction(memory);
                        self.a = self.a ^ immediate;
                    }
                    _ => {
                        panic!("Attempted to execute instruction with invalid AddressingMode");
                    }
                };
                self.set_status_nz(self.a);
            }
            Instruction::INC(am) => match am {
                AddressingMode::Absolute
                | AddressingMode::AbsoluteX
                | AddressingMode::ZeroPage
                | AddressingMode::ZeroPageX => {
                    let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    let mut to_modify = memory[usize::from(address)];
                    to_modify = to_modify.wrapping_add(1);
                    memory[usize::from(address)] = to_modify;
                    self.set_status_nz(to_modify);
                }
                _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
            },
            Instruction::INX(am) => {
                if let AddressingMode::Implied = am {
                    self.x = self.x.wrapping_add(1);
                    self.set_status_nz(self.x);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::INY(am) => {
                if let AddressingMode::Implied = am {
                    self.y = self.y.wrapping_add(1);
                    self.set_status_nz(self.y);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::JMP(am) => {
                if let AddressingMode::Absolute | AddressingMode::Indirect = am {
                    let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    self.pc = address;
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
                    let to_be_pushed = self.pc + 1;
                    let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    let lo = to_be_pushed as u8;
                    let hi = (to_be_pushed >> 8) as u8;
                    self.push_stack(hi, memory);
                    self.push_stack(lo, memory);
                    self.pc = address;
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
                        let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        self.a = memory[usize::from(address)];
                    }
                    AddressingMode::Immediate => {
                        self.a = self.fetch_instruction(memory);
                    }
                    _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
                }
                self.set_status_nz(self.a);
            }
            Instruction::LDX(am) => {
                match am {
                    AddressingMode::Absolute
                    | AddressingMode::AbsoluteY
                    | AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageY => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        self.x = memory[usize::from(address)];
                    }
                    AddressingMode::Immediate => {
                        self.x = self.fetch_instruction(memory);
                    }
                    _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
                }
                self.set_status_nz(self.x);
            }
            Instruction::LDY(am) => {
                match am {
                    AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        self.y = memory[usize::from(address)];
                    }
                    AddressingMode::Immediate => {
                        self.y = self.fetch_instruction(memory);
                    }
                    _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
                }
                self.set_status_nz(self.y);
            }
            Instruction::LSR(am) => {
                let shift_result: u8;
                match am {
                    AddressingMode::Absolute
                    | AddressingMode::AbsoluteX
                    | AddressingMode::ZeroPage
                    | AddressingMode::ZeroPageX => {
                        let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        let value = memory[usize::from(address)];
                        self.p.c = if value & 0x01 == 0x01 { true } else { false };
                        shift_result = self.a >> 1;
                        memory[usize::from(address)] = shift_result;
                    }
                    AddressingMode::Accumulator => {
                        self.p.c = if self.a & 0x01 == 0x01 { true } else { false };
                        self.a = self.a >> 1;
                        shift_result = self.a;
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
                        let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        let value = memory[usize::from(address)];
                        self.a = self.a | value;
                    }
                    AddressingMode::Immediate => {
                        let immediate = self.fetch_instruction(memory);
                        self.a = self.a | immediate;
                    }
                    _ => {
                        panic!("Attempted to execute instruction with invalid AddressingMode");
                    }
                };
                self.set_status_nz(self.a);
            }
            Instruction::PHA(am) => {
                if let AddressingMode::Implied = am {
                    self.push_stack(self.a, memory);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::PHP(am) => {
                if let AddressingMode::Implied = am {
                    let b: u8 = 0b0001_0000;
                    let p = self.p.to_byte() | b;
                    self.push_stack(p, memory);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::PLA(am) => {
                if let AddressingMode::Implied = am {
                    self.a = self.pop_stack(memory);
                    self.set_status_nz(self.a);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::PLP(am) => {
                if let AddressingMode::Implied = am {
                    // bits 4 and 5 are ignored
                    let p = self.pop_stack(memory) & 0b1100_1111;
                    self.p.set_from_byte(p)
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
                        let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        let mut value = memory[usize::from(address)];
                        let tail = self.p.c;
                        self.p.c = if value & 0x80 == 0x80 { true } else { false };
                        value = self.a << 1;
                        shift_result = if tail == true { value | 0x01 } else { value };
                        memory[usize::from(address)] = shift_result;
                    }
                    AddressingMode::Accumulator => {
                        let tail = self.p.c;
                        self.p.c = if self.a & 0x80 == 0x80 { true } else { false };
                        self.a = self.a << 1;
                        self.a = if tail == true { self.a | 0x01 } else { self.a };
                        shift_result = self.a;
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
                        let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                        if boundary_crossed == true {
                            num_ticks += 1;
                        }
                        let mut value = memory[usize::from(address)];
                        let tail = self.p.c;
                        self.p.c = if value & 0x01 == 0x01 { true } else { false };
                        value = self.a >> 1;
                        shift_result = if tail == true { value | 0x80 } else { value };
                        memory[usize::from(address)] = shift_result;
                    }
                    AddressingMode::Accumulator => {
                        let tail = self.p.c;
                        self.p.c = if self.a & 0x01 == 0x01 { true } else { false };
                        self.a = self.a >> 1;
                        self.a = if tail == true { self.a | 0x80 } else { self.a };
                        shift_result = self.a;
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
                    let p = self.pop_stack(memory) & 0b1100_1111;
                    self.p.set_from_byte(p);

                    let lo = self.pop_stack(memory);
                    let hi = self.pop_stack(memory);
                    let address = (u16::from(hi) << 8) + u16::from(lo);
                    self.pc = address;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::RTS(am) => {
                if let AddressingMode::Implied = am {
                    let lo = self.pop_stack(memory);
                    let hi = self.pop_stack(memory);
                    let address = (u16::from(hi) << 8) + u16::from(lo);
                    self.pc = address.wrapping_add(1);
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
                    let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    let complement = !memory[usize::from(address)];
                    self.adc_logic(complement);
                }
                AddressingMode::Immediate => {
                    let immediate = self.fetch_instruction(memory);
                    self.adc_logic(!(immediate as u8));
                }
                _ => {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            },
            Instruction::SEC(am) => {
                if let AddressingMode::Implied = am {
                    self.p.c = true;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::SED(am) => {
                if let AddressingMode::Implied = am {
                    self.p.d = true;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::SEI(am) => {
                if let AddressingMode::Implied = am {
                    self.p.i = true;
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
                    let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    memory[usize::from(address)] = self.a;
                }
                _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
            },
            Instruction::STX(am) => match am {
                AddressingMode::Absolute | AddressingMode::ZeroPage | AddressingMode::ZeroPageY => {
                    let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    memory[usize::from(address)] = self.x;
                }
                _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
            },
            Instruction::STY(am) => match am {
                AddressingMode::Absolute | AddressingMode::ZeroPage | AddressingMode::ZeroPageX => {
                    let (address, boundary_crossed) = self.resolve_address_fetch(am, memory);
                    if boundary_crossed == true {
                        num_ticks += 1;
                    }
                    memory[usize::from(address)] = self.y;
                }
                _ => panic!("Attempted to execute instruction with invalid AddressingMode"),
            },
            Instruction::TAX(am) => {
                if let AddressingMode::Implied = am {
                    self.x = self.a;
                    self.set_status_nz(self.x);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::TAY(am) => {
                if let AddressingMode::Implied = am {
                    self.y = self.a;
                    self.set_status_nz(self.y);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::TSX(am) => {
                if let AddressingMode::Implied = am {
                    self.x = self.sp;
                    self.set_status_nz(self.x);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::TXA(am) => {
                if let AddressingMode::Implied = am {
                    self.a = self.x;
                    self.set_status_nz(self.a);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::TXS(am) => {
                if let AddressingMode::Implied = am {
                    self.sp = self.x;
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::TYA(am) => {
                if let AddressingMode::Implied = am {
                    self.a = self.y;
                    self.set_status_nz(self.a);
                } else {
                    panic!("Attempted to execute instruction with invalid AddressingMode");
                }
            }
            Instruction::Invalid(byte) => panic!(
                "Attempted to execute undocumented instruction : 0x{:x}",
                byte
            ),
        }
        self.tick(num_ticks);
        num_ticks
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub enum ReadWrite {
    Write,
    #[default]
    Read,
}

/// Type for storing the flags of the status register as fields
#[derive(Copy, Clone, Default, Debug)]
pub struct StatusRegister {
    /// negative flag
    pub n: bool,
    /// overflow flag
    pub v: bool,
    /// brk flag
    pub b: bool,
    /// bcd flag
    pub d: bool,
    /// interrupt disable flag
    pub i: bool,
    /// zero flag
    pub z: bool,
    /// carry flag
    pub c: bool,
}

/// bitflag representation of the N flag
const N: u8 = 0b1000_0000;
/// bitflag representation of the V flag
const V: u8 = 0b0100_0000;
/// bitflag representation of the B flag
const B: u8 = 0b0001_0000;
/// bitflag representation of the D flag
const D: u8 = 0b0000_1000;
/// bitflag representation of the I flag
const I: u8 = 0b0000_0100;
/// bitflag representation of the Z flag
const Z: u8 = 0b0000_0010;
/// bitflag representation of the C flag
const C: u8 = 0b0000_0001;

impl StatusRegister {
    /// returns status register represented by an 8-bit number
    pub fn to_byte(&self) -> u8 {
        // unused flag is always set to 1
        let mut byte: u8 = 0b0010_0000;

        byte = if self.n == true { byte | N } else { byte };
        byte = if self.v == true { byte | V } else { byte };
        byte = if self.b == true { byte | B } else { byte };
        byte = if self.d == true { byte | D } else { byte };
        byte = if self.i == true { byte | I } else { byte };
        byte = if self.z == true { byte | Z } else { byte };
        byte = if self.c == true { byte | C } else { byte };

        byte
    }

    /// sets status register using an 8-bit number
    pub fn set_from_byte(&mut self, p: u8) {
        self.n = if p & N == N { true } else { false };
        self.v = if p & V == V { true } else { false };
        self.b = if p & B == B { true } else { false };
        self.d = if p & D == D { true } else { false };
        self.i = if p & I == I { true } else { false };
        self.z = if p & Z == Z { true } else { false };
        self.c = if p & C == C { true } else { false };
    }
}
