mod rom_data {
    pub static HEADER_MAGIC_NUMBER: &'static [u8; 4] = b"NES\x1A";
    pub static HEADER_NES2_IDENT: u8 = 0b0000_1000;
    pub static HEADER_CONSOLE_TYPE: u8 = 0b0000_0000;
    pub static BANK_SIZE: usize = 0x2000; // 8KiB
    pub enum TimingMode {
        Ntsc = 0,
        Pal = 1,
        Multiple = 2,
        Dendy = 3,
    }
    pub enum DefaultExpansionDevice {
        Unspecified = 0x00,
        StandarNesController = 0x01,
        /* TODO: Add the rest from this list https://wiki.nesdev.com/w/index.php/NES_2.0#Default_Expansion_Device*/
    }
}

pub struct Cartridge {
    header: [u8; 16],
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}
impl Cartridge {
    pub fn new(code: &[u8], banks: Vec<usize>) -> Self {
        let prg_rom_size: u16 = 0x001; // Check here: https://wiki.nesdev.com/w/index.php/NES_2.0#PRG-ROM_Area
        let chr_rom_size: u16 = 0x001;
        let prg_ram_size: u8 = 0x00;
        let eeprom_size: u8 = 0x00;

        let mut header = [0u8; 16];
        header[0..4].clone_from_slice(rom_data::HEADER_MAGIC_NUMBER);
        header[4] = (prg_rom_size & 0x0FF) as u8;
        header[5] = (chr_rom_size & 0x0FF) as u8;
        header[6] = 0b0000_0001;
        header[7] = 0x00 | rom_data::HEADER_NES2_IDENT | rom_data::HEADER_CONSOLE_TYPE;
        header[8] = 0x00; // Mapper MSB / Submapper
        header[9] |= ((prg_rom_size & 0xF00) >> 4) as u8;
        header[9] |= ((chr_rom_size & 0xF00) >> 8) as u8;
        header[10] = (prg_ram_size << 4 & 0xF0) | (eeprom_size & 0x0F);
        header[11] = 0x00;
        header[12] = rom_data::TimingMode::Ntsc as u8;
        header[13] = 0x00;
        header[14] = 0x00;
        header[15] = rom_data::DefaultExpansionDevice::StandarNesController as u8;

        /* TODO: Trainer area? */

        let prg_rom_real_size: usize = if prg_rom_size & 0x800 == 0 {
            // prgRomSize specifies size in 16KiB
            prg_rom_size as usize * 16 * 1024
        } else {
            // Exponent notation
            unimplemented!();
        };
        let mut prg_rom_area: Vec<u8> = vec![0; prg_rom_real_size];
        println!("Created PRG-ROM with {} bytes", prg_rom_area.len());
        for (i, bank) in banks.iter().enumerate() {
            if i == 2 {
                continue;
            }
            println!("Copying from {:04X} to {:04X}", bank, i * 0x8000);
            for j in 0..rom_data::BANK_SIZE {
                let rom_idx = i * rom_data::BANK_SIZE + j;
                let code_idx = bank + j;
                if rom_idx >= prg_rom_area.len() || code_idx >= code.len() {
                    panic!(
                        r#"Out-of-bounds exception:
          rom_idx: {:#06X}. Rom Size: {:#06X}
          code_idx: {:#06X}. Code Size: {:#06X}
          Bank: {} / Addr {:#06X}
        "#,
                        rom_idx,
                        prg_rom_area.len(),
                        code_idx,
                        code.len(),
                        i,
                        bank
                    );
                }
                prg_rom_area[rom_idx] = code[code_idx];
            }
        }

        let mut chr_rom_real_size = if chr_rom_size & 0x800 == 0 {
            chr_rom_size as usize * 8 * 1024
        } else {
            // Exponent notation
            unimplemented!();
        };
        let mut chr_rom_area = vec![0; chr_rom_real_size];
        println!("Created CHR-ROM with {} bytes", chr_rom_area.len());
        let bank_addr = banks[2];
        for i in 0..0x2000 {
            // 8KiB
            chr_rom_area[i] = code[bank_addr + i];
        }

        Self {
            header,
            prg_rom: prg_rom_area,
            chr_rom: chr_rom_area,
        }
    }
}
impl std::convert::Into<Vec<u8>> for Cartridge {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.prg_rom.len() + self.chr_rom.len() + 16);
        bytes.extend(&self.header.to_vec());
        bytes.extend(&self.prg_rom);
        bytes.extend(&self.chr_rom);
        println!("{}", bytes.len());
        bytes
    }
}
