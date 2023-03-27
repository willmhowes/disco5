#[derive(Copy, Clone, Default, Debug)]
pub struct PPU {
    /// VPHB SINN | NMI enable (V), PPU master/slave (P), sprite height (H), background tile select (B), sprite tile select (S), increment mode (I), nametable select (NN)
    pub PPUCTRL: u8,
    /// BGRs bMmG | color emphasis (BGR), sprite enable (s), background enable (b), sprite left column enable (M), background left column enable (m), greyscale (G)
    pub PPUMASK: u8,
    /// VSO- ---- | vblank (V), sprite 0 hit (S), sprite overflow (O); read resets write pair for $2005/$2006
    pub PPUSTATUS: u8,
    /// aaaa aaaa | OAM read/write address
    pub OAMADDR: u8,
    /// dddd dddd | OAM data read/write
    pub OAMDATA: u8,
    /// xxxx xxxx | fine scroll position (two writes: X scroll, Y scroll)
    pub PPUSCROLL: u8,
    /// aaaa aaaa | PPU read/write address (two writes: most significant byte, least significant byte)
    pub PPUADDR: u8,
    /// dddd dddd | PPU data read/write
    pub PPUDATA: u8,
    /// OAM DMA high address
    pub OAMDMA: u8,
}
