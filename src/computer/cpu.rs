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
    pub data_bus: u8,
    pub address_bus: u16,
}

impl CPU {
    pub fn set_data_bus(&mut self, data: u8) {
        self.data_bus = data;
    }

    pub fn set_address_bus(&mut self, data: u16) {
        self.address_bus = data;
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

impl StatusRegister {
    /// returns status register represented by an 8-bit number
    pub fn to_byte(&self) -> u8 {
        // unused flag is always set to 1
        let mut byte: u8 = 0b0010_0000;

        let n: u8 = 0b1000_0000;
        let v: u8 = 0b0100_0000;
        let b: u8 = 0b0001_0000;
        let d: u8 = 0b0000_1000;
        let i: u8 = 0b0000_0100;
        let z: u8 = 0b0000_0010;
        let c: u8 = 0b0000_0001;
        byte = if self.n == true { byte | n } else { byte };
        byte = if self.v == true { byte | v } else { byte };
        byte = if self.b == true { byte | b } else { byte };
        byte = if self.d == true { byte | d } else { byte };
        byte = if self.i == true { byte | i } else { byte };
        byte = if self.z == true { byte | z } else { byte };
        byte = if self.c == true { byte | c } else { byte };

        byte
    }

    /// sets status register using an 8-bit number
    pub fn set_from_byte(&mut self, p: u8) {
        let n: u8 = 0b1000_0000;
        let v: u8 = 0b0100_0000;
        let b: u8 = 0b0001_0000;
        let d: u8 = 0b0000_1000;
        let i: u8 = 0b0000_0100;
        let z: u8 = 0b0000_0010;
        let c: u8 = 0b0000_0001;
        self.n = if p & n == n { true } else { false };
        self.v = if p & v == v { true } else { false };
        self.b = if p & b == b { true } else { false };
        self.d = if p & d == d { true } else { false };
        self.i = if p & i == i { true } else { false };
        self.z = if p & z == z { true } else { false };
        self.c = if p & c == c { true } else { false };
    }
}

impl CPU {
    /// steps pc to next position
    pub fn step_pc(&mut self) {
        self.pc = self.pc + 1;
    }
}
