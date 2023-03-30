const PPU_MEMORY_SIZE: usize = 0x4000;
const OAM_SIZE: usize = 0x100;

#[derive(Copy, Clone, Debug)]
pub struct PPU {
    /// VPHB SINN | NMI enable (V), PPU master/slave (P), sprite height (H), background tile select (B), sprite tile select (S), increment mode (I), nametable select (NN)
    pub ppu_ctrl: u8,
    /// BGRs bMmG | color emphasis (BGR), sprite enable (s), background enable (b), sprite left column enable (M), background left column enable (m), greyscale (G)
    pub ppu_mask: u8,
    /// VSO- ---- | vblank (V), sprite 0 hit (S), sprite overflow (O); read resets write pair for $2005/$2006
    pub ppu_status: u8,
    /// aaaa aaaa | OAM read/write address
    pub oam_addr: u8,
    /// dddd dddd | OAM data read/write
    pub oam_data: u8,
    /// xxxx xxxx | fine scroll position (two writes: X scroll, Y scroll)
    pub ppu_scroll: u8,
    /// aaaa aaaa | PPU read/write address (two writes: most significant byte, least significant byte)
    pub ppu_addr: u8,
    /// dddd dddd | PPU data read/write
    pub ppu_data: u8,
    /// OAM DMA high address
    pub oam_dma: u8,
    /// PPU address space
    pub memory: [u8; PPU_MEMORY_SIZE],
    /// Object Attribute Memory (OAM) array
    pub oam: [u8; OAM_SIZE]
}

impl Default for PPU {
    fn default() -> PPU {
        PPU {
            ppu_ctrl : Default::default(),
            ppu_mask : Default::default(),
            ppu_status : Default::default(),
            oam_addr : Default::default(),
            oam_data : Default::default(),
            ppu_scroll : Default::default(),
            ppu_addr : Default::default(),
            ppu_data : Default::default(),
            oam_dma : Default::default(),
            memory : [0; PPU_MEMORY_SIZE],
            oam : [0; OAM_SIZE],
        }
    }
}

