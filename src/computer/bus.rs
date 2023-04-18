use crate::computer::{cpu::ReadWrite, ppu::PPU, ppu_structs::PPUCTRL};
use std::ops::{Index, IndexMut};

const CPU_MEMORY_SIZE: usize = 0x10000;

#[derive(Copy, Clone, Debug)]
pub struct Bus {
    pub bytes: [u8; CPU_MEMORY_SIZE],
    pub data_bus: u8,
    pub address_bus: u16,
    pub ppu: PPU,
}

impl Default for Bus {
    fn default() -> Bus {
        Bus {
            bytes: [0; CPU_MEMORY_SIZE],
            data_bus: Default::default(),
            address_bus: Default::default(),
            ppu: Default::default(),
        }
    }
}

impl Index<usize> for Bus {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        // println!("Accessing 0x{index:x} in bus immutably");
        match index {
            // oam_addr_first_write needs to be reset when 0x2002 is read
            0x2002 => &self.ppu.ppu_status,
            0x2004 => &self.ppu.oam_data,
            0x2007 => {
                let lo = self.ppu.ppu_addr_low;
                let hi = self.ppu.ppu_addr_high;
                let address = (u16::from(hi) << 8) + u16::from(lo);
                &self.ppu.memory[usize::from(address)]
            }
            // 0x4016 => todo!(),
            // 0x4017 => todo!(),
            _ => {
                // println!("LOADING: 0x{:0>2x}", self.bytes[index]);
                &self.bytes[index]
            }
        }
    }
}

impl IndexMut<usize> for Bus {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        // println!("Accessing 0x{index:x} in bus mutably");
        match index {
            0x2000 => &mut self.ppu.ppu_ctrl,
            0x2001 => &mut self.ppu.ppu_mask,
            0x2003 => &mut self.ppu.oam_addr,
            0x2004 => &mut self.ppu.oam_data,
            0x2005 => &mut self.ppu.ppu_scroll,
            0x2006 => {
                if self.ppu.ppu_addr_received_first_write == false {
                    self.ppu.ppu_addr_received_first_write =
                        !self.ppu.ppu_addr_received_first_write;
                    &mut self.ppu.ppu_addr_high
                } else {
                    self.ppu.ppu_addr_received_first_write =
                        !self.ppu.ppu_addr_received_first_write;
                    &mut self.ppu.ppu_addr_low
                }
            }
            0x2007 => {
                // calculate full ppu_addr address
                let lo = self.ppu.ppu_addr_low;
                let hi = self.ppu.ppu_addr_high;
                let address = (u16::from(hi) << 8) + u16::from(lo);

                // increment address in ppu_addr register
                let increment =
                    if self.ppu.ppu_ctrl & PPUCTRL::VRAM_INCR.bits() == PPUCTRL::VRAM_INCR.bits() {
                        32
                    } else {
                        1
                    };
                let new_address = address.wrapping_add(increment);
                self.ppu.ppu_addr_low = new_address as u8;
                self.ppu.ppu_addr_high = (new_address >> 8) as u8;

                // uncomment to print address in 0x2006 being written to
                // println!("--------------------- 0x2007, to 0x{:0>4x}", address);
                // let mut line = String::new();
                // let b1 = std::io::stdin().read_line(&mut line).unwrap();
                // println!("{:?}", &self.ppu.memory[0x2000..0x2400]);

                // return address from ppu_addr before it was incremented
                &mut self.ppu.memory[usize::from(address)]
            }
            // 0x4014 => todo!(),
            // 0x4016 => todo!(),
            _ => {
                // println!("WRITING TO: 0x{:0>4x}", index);
                &mut self.bytes[index]
            }
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
