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

impl CPU {
    /// steps pc to next position
    pub fn step(&mut self) {
        self.pc = self.pc + 1;
    }
}
