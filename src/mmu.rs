use input_output::InputOutput;
use mbc::no_mbc::NoMbc;
use simple::SimpleRegion;

mod simple;
mod input_output;
mod mbc;

static MBC_INFO_ADDRESS: usize = 0x0147;
static MBC_ROM_SIZE_ADDRESS: usize = 0x0148;
static MBC_RAM_SIZE_ADDRESS: usize = 0x0149;

pub trait MemoryOperations {
    /// Read a byte from the memory region
    fn read_byte(&self, address: u16) -> u8;

    /// Write a byte to the memory region
    fn write_byte(&mut self, address: u16, value: u8);

    /// Read a 16-bit word from the memory region
    fn read_word(&self, address: u16) -> u16 {
        let low_byte = self.read_byte(address) as u16;
        let high_byte = self.read_byte(address + 1) as u16;
        (high_byte << 8) | low_byte
    }
}

pub trait NonMbcOperations: MemoryOperations {
    /// Fill the memory region with the data from the slice
    fn fill_from_slice(&mut self, data: &[u8]);
}

trait MemoryBankControllerOperations: MemoryOperations {
    /// Initialize the Memory Bank Controller
    fn init(&mut self, rom_size: u8, cartridge_type: u8, ram_size: u8);

    /// Fill a specific rom bank with the data from the slice
    fn fill_rom_bank_from_slice(&mut self, bank: u8, data: &[u8; 0x4000]);

    /// Fill a specific ram bank with the data from the slice
    fn fill_ram_bank_from_slice(&mut self, bank: u8, data: &[u8; 0x2000]);

    /// Switch the ROM bank
    fn switch_rom_bank(&mut self, bank: u8);

    /// Get the amount of ROM banks
    fn switch_ram_bank(&mut self, bank: u8);

    /// Enable or disable the RAM
    fn enable_ram(&mut self, enable: bool);
}

struct MMU {
    /// 0x0000 to 0x3FFF - ROM Bank 00
    pub bank_00: SimpleRegion,

    /// The cartridge is a separate memory region from 
    /// 0x4000 to 0x7FFF for ROM 
    /// 0xA000 to 0xBFFF for RAM
    pub mbc: Box<dyn MemoryBankControllerOperations>,

    /// 0x8000 to 0x9FFF - Graphics RAM
    pub VRAM: SimpleRegion,

    /// 0xA000 to 0xBFFF - External RAM
    /// Handled by the cartridge

    /// 0xC000 to 0xDFFF - Working RAM
    pub WRAM: SimpleRegion,

    /// 0xE000 to 0xFDFF - Mirror of C000~DDFF (ECHO RAM)
    /// Handled by the Working RAM (MMU has to handle this)

    /// 0xFE00 to 0xFE9F - Object Attribute Memory (OAM)
    pub OAM: SimpleRegion,

    /// 0xFF00 to 0xFF7F - I/O Registers
    pub IO: InputOutput,

    /// 0xFF80 to 0xFFFE - High RAM (HRAM)
    pub HRAM: SimpleRegion,

    /// 0xFFFF - Interrupt Enable Register
    pub interrupt_enable: u8
}

impl MMU {
    pub fn new_from_vec(rom: Vec<u8>) -> Self {
        let mbc_info = match rom.get(MBC_INFO_ADDRESS) {
            Some(&mbc_info) => mbc_info,
            None => panic!("No MBC info found in ROM")
        };

        let mut mmu = MMU::new_from_mbc_info(mbc_info);

        mmu.fill_from_slice(rom.as_slice());

        mmu
    }

    pub fn new_from_mbc_info(mbc_info: u8) -> Self {
        let cartridge: Box<dyn MemoryBankControllerOperations> = match mbc_info {
            0x00 => Box::new(NoMbc::default()),
            _ => panic!("Unsupported MBC type: {mbc_info}")
        };

        MMU {
            bank_00: SimpleRegion::new(0x4000, false),
            mbc: cartridge,
            VRAM: SimpleRegion::new(0x2000, true),
            WRAM: SimpleRegion::new(0x2000, true),
            OAM: SimpleRegion::new(0x00A0, true),
            IO: InputOutput::new(0x0080),
            HRAM: SimpleRegion::new(0x007F, true),
            interrupt_enable: 0
        }
    }
}

impl NonMbcOperations for MMU {
    fn fill_from_slice(&mut self, data: &[u8]) {
        // Get relevant information from the ROM for the mbc
        let rom_size = match data.get(MBC_ROM_SIZE_ADDRESS) {
            Some(&rom_size) => rom_size,
            None => panic!("No ROM size found in ROM")
        };
        let ram_size = match data.get(MBC_RAM_SIZE_ADDRESS) {
            Some(&ram_size) => ram_size,
            None => panic!("No RAM size found in ROM")
        };
        let mbc_info = match data.get(MBC_INFO_ADDRESS) {
            Some(&mbc_info) => mbc_info,
            None => panic!("No MBC info found in ROM")
        };
        // Initialize the MBC
        self.mbc.init(rom_size, mbc_info, ram_size);

        // Fill the ROM bank 00
        self.bank_00.fill_from_slice(&data[0x0000..0x4000]);

        // Fill the mbc with 
        
    }
}

impl MemoryOperations for MMU {    
    fn read_byte(&self, address: u16) -> u8 {
        0
    }

    fn write_byte(&mut self, address: u16, value: u8) {
    }
}