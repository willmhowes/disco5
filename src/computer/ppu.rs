use crate::computer::ppu_structs::{PPUCTRL, SYSTEM_COLOR_PALETTE};

const PPU_MEMORY_SIZE: usize = 0x4000;
const OAM_SIZE: usize = 0x100;

const FRAME_WIDTH: usize = 256;
const FRAME_HEIGHT: usize = 240;
const FRAME_BUFFER_SIZE: usize = FRAME_WIDTH * FRAME_HEIGHT;

const TILE_SIZE: usize = 8;
const FRAME_WIDTH_IN_TILES: usize = FRAME_WIDTH / TILE_SIZE;
const FRAME_HEIGHT_IN_TILES: usize = FRAME_HEIGHT / TILE_SIZE;

const ATTRIBUTE_TABLE_COVERAGE_SIZE: usize = TILE_SIZE * 4;

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
    pub ppu_addr_low: u8,
    pub ppu_addr_high: u8,
    // This needs to be a mutex
    pub ppu_addr_received_first_write: bool,
    /// OAM DMA high address
    pub oam_dma: u8,
    /// PPU address space
    pub memory: [u8; PPU_MEMORY_SIZE],
    /// Object Attribute Memory (OAM) array
    pub oam: [u8; OAM_SIZE],
    y_pixel: usize,
    x_pixel: usize,
}

impl Default for PPU {
    fn default() -> PPU {
        PPU {
            ppu_ctrl: Default::default(),
            ppu_mask: Default::default(),
            ppu_status: 0x80,
            oam_addr: Default::default(),
            oam_data: Default::default(),
            ppu_scroll: Default::default(),
            ppu_addr_low: Default::default(),
            ppu_addr_high: Default::default(),
            ppu_addr_received_first_write: Default::default(),
            oam_dma: Default::default(),
            memory: [0; PPU_MEMORY_SIZE],
            oam: [0; OAM_SIZE],
            y_pixel: Default::default(),
            x_pixel: Default::default(),
        }
    }
}

impl PPU {
    pub fn increment_line_counter(&self) {
        // let mut guard = self.line_counter.lock().unwrap();
        // *guard +=1;
        // drop(guard);
    }

    /// VPHB SINN | NMI enable (V), PPU master/slave (P), sprite height (H), background tile select (B), sprite tile select (S), increment mode (I), nametable select (NN)
    pub fn ppu_ctrl_write(&mut self, input: u8) {
        todo!();
    }

    /// BGRs bMmG | color emphasis (BGR), sprite enable (s), background enable (b), sprite left column enable (M), background left column enable (m), greyscale (G)
    pub fn ppu_mask_write(&mut self, input: u8) {
        todo!();
    }

    /// VSO- ---- | vblank (V), sprite 0 hit (S), sprite overflow (O); read resets write pair for $2005/$2006
    pub fn ppu_status_read(&self) -> &u8 {
        self.increment_line_counter();
        &0x80
    }

    /// aaaa aaaa | OAM read/write address
    pub fn oam_addr_write(&mut self, input: u8) {
        todo!();
    }

    /// dddd dddd | OAM data read/write
    pub fn oam_data_read(&self) -> &u8 {
        todo!();
    }

    /// dddd dddd | OAM data read/write
    pub fn oam_data_write(&mut self, input: u8) {
        todo!();
    }

    /// xxxx xxxx | fine scroll position (two writes: X scroll, Y scroll)
    pub fn ppu_scroll_write(&mut self, input: u8) {
        todo!();
    }

    /// aaaa aaaa | PPU read/write address (two writes: most significant byte, least significant byte)
    pub fn ppu_addr_write(&mut self, input: u8) {
        todo!();
    }

    /// dddd dddd | PPU data read/write
    pub fn ppu_data_read(&self) -> &u8 {
        todo!();
    }

    /// dddd dddd | PPU data read/write
    pub fn ppu_data_write(&mut self, input: u8) {
        todo!();
    }

    /// OAM DMA high address
    pub fn oam_dma_write(&mut self, input: u8) {
        todo!();
    }

    // (X,Y) (256,240) (32,30)
    fn fetch_nametable_byte(&self) -> u8 {
        // floor divide coodinates to get nametable coordinate
        let x = self.x_pixel / TILE_SIZE;
        let y = self.y_pixel / TILE_SIZE;
        let index = y * FRAME_WIDTH_IN_TILES + x;
        let index = index + 0x2000;
        self.memory[index]
    }

    fn fetch_attribute_byte(&self) -> u8 {
        // TODO: pretty sure this needs to update line_x and line_y values
        let x = self.x_pixel / ATTRIBUTE_TABLE_COVERAGE_SIZE;
        let y = self.y_pixel / ATTRIBUTE_TABLE_COVERAGE_SIZE;
        let index = y * 8 + x;
        let index = index + 0x23C0;
        self.memory[index]
    }

    /// returns back subpalette index in the lowest two bytes
    fn fetch_attribute_byte_subpalette_index(&self, attribute_byte: u8) -> u8 {
        let x = self.x_pixel % ATTRIBUTE_TABLE_COVERAGE_SIZE;
        let y = self.y_pixel % ATTRIBUTE_TABLE_COVERAGE_SIZE;
        // deconstruct the attribute byte to determine subpalette index
        // and wipe upper six bits if necessary
        if x > 16 && y > 16 {
            // bottom right quadrant
            attribute_byte >> 6
        } else if y > 16 {
            // bottom left quadrant
            (attribute_byte >> 4) & 0b00000011
        } else if x > 16 {
            // top right quadrant
            (attribute_byte >> 2) & 0b00000011
        } else {
            // top left quadrant
            attribute_byte & 0b00000011
        }
    }

    fn fetch_line_from_pattern_table(&self, nametable_index: u8) -> (u8, u8) {
        let background_pattern_table: usize = if self.ppu_ctrl & PPUCTRL::BG_PATTERN_TABLE.bits()
            == PPUCTRL::BG_PATTERN_TABLE.bits()
        {
            0x1000
        } else {
            0x0000
        };
        let index = background_pattern_table + usize::from(nametable_index) * 16;
        let line_within_tile = self.y_pixel % TILE_SIZE;
        let index = index + line_within_tile;
        (self.memory[index], self.memory[index + 8])
    }

    pub fn render_tile(&self, buffer: &mut [(u8, u8, u8)]) {
        let n = self.fetch_nametable_byte();
        let a = self.fetch_attribute_byte();
        // determine the tile's color palette
        let palette_index = self.fetch_attribute_byte_subpalette_index(a);

        // $3F00 	    Universal background color
        // $3F01-$3F03 	Background palette 0
        // $3F05-$3F07 	Background palette 1
        // $3F09-$3F0B 	Background palette 2
        // $3F0D-$3F0F 	Background palette 3

        // store each system color palette index
        let color_0_index = self.memory[0x3f00];
        let color_1_index = self.memory[0x3f01 + usize::from(palette_index) * 4];
        let color_2_index = self.memory[0x3f02 + usize::from(palette_index) * 4];
        let color_3_index = self.memory[0x3f03 + usize::from(palette_index) * 4];

        // fetch rgb values for each color in color palette
        let color_0 = SYSTEM_COLOR_PALETTE[usize::from(color_0_index)];
        let color_1 = SYSTEM_COLOR_PALETTE[usize::from(color_1_index)];
        let color_2 = SYSTEM_COLOR_PALETTE[usize::from(color_2_index)];
        let color_3 = SYSTEM_COLOR_PALETTE[usize::from(color_3_index)];

        let (tile_line_low, tile_line_high) = self.fetch_line_from_pattern_table(n);

        // merge the low and high byte for each pixel and assign color to buffer
        let mut line_index: u8 = 0x80;
        for i in 0..8 {
            // println!("{:0>8b}", line_index);
            if line_index & tile_line_low == line_index && line_index & tile_line_high == line_index
            {
                buffer[i] = color_3;
            } else if line_index & tile_line_high == line_index {
                buffer[i] = color_2;
            } else if line_index & tile_line_low == line_index {
                buffer[i] = color_1;
            } else {
                buffer[i] = color_0;
            }
            line_index = line_index >> 1;
        }
    }

    pub fn render_line(&self, buffer: &mut [(u8, u8, u8)]) {
        for i in 0..FRAME_WIDTH_IN_TILES {
            let tile_ref = &mut buffer[TILE_SIZE * i..TILE_SIZE * i + TILE_SIZE];
            self.render_tile(tile_ref);
        }
    }

    pub fn render_frame(&self) {
        let mut frame_buffer: [(u8, u8, u8); FRAME_BUFFER_SIZE] = [(0, 0, 0); FRAME_BUFFER_SIZE];
        for i in 0..FRAME_HEIGHT {
            let line_ref = &mut frame_buffer[FRAME_WIDTH * i..FRAME_WIDTH * i + FRAME_WIDTH];
            self.render_line(line_ref);
        }

        println!("{:?}", frame_buffer);
    }
}
