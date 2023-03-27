use super::cpu::ReadWrite;
use std::ops::{Index, IndexMut};

const MEMORY_SIZE: usize = 0x10000;

#[derive(Copy, Clone, Debug)]
pub struct Bus {
    pub bytes: [u8; MEMORY_SIZE],
    pub data_bus: u8,
    pub address_bus: u16,
}

impl Default for Bus {
    fn default() -> Bus {
        Bus {
            bytes: [0; MEMORY_SIZE],
            data_bus: Default::default(),
            address_bus: Default::default(),
        }
    }
}

impl Index<usize> for Bus {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        // println!("Accessing 0x{index:x} in bus immutably");
        match index {
            // 0x2002 => todo!(),
            // 0x2004 => todo!(),
            // 0x2007 => todo!(),
            _ => &self.bytes[index],
        }
    }
}

impl IndexMut<usize> for Bus {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // println!("Accessing 0x{index:x} in bus mutably");
        match index {
            // 0x2000 => todo!(),
            // 0x2001 => todo!(),
            // 0x2003 => todo!(),
            // 0x2004 => todo!(),
            // 0x2005 => todo!(),
            // 0x2006 => todo!(),
            // 0x4014 => todo!(),
            _ => &mut self.bytes[index],
        }
    }
}

impl Bus {
    /// low is write, high is read
    pub fn execute(&mut self, readwrite: ReadWrite) {
        match readwrite {
            ReadWrite::Read => {
                let address = self.address_bus;
                self.data_bus = self[usize::from(address)];
            }
            ReadWrite::Write => {
                let address = self.address_bus;
                let data = self.data_bus;
                self[usize::from(address)] = data;
            }
        }
    }
}
