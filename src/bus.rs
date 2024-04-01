use crate::devices::{ram::RAM, rom::ROM, Device};

pub trait Bus {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
}

pub struct BenEaterBus {
    rom: ROM,
    ram: RAM,
}

impl BenEaterBus {
    pub fn new(rom: ROM, ram: RAM) -> BenEaterBus {
        BenEaterBus { rom, ram }
    }
}

impl Bus for BenEaterBus {
    fn read(&mut self, address: u16) -> u8 {
        // println!("Reading from BUS: '{address:x}'");
        match address {
            0x6000..=0x7FFF => self.ram.read(address - 0x6000),
            0x8000..=0xFFFF => self.rom.read(address - 0x8000),
            _ => panic!("Trying to select address '{address}' on BenEaterBus, but it doesn't have a device there!")
        }
    }

    fn write(&mut self, address: u16, data: u8) {
        // println!("Writing to BUS: '{address:x}'");
        match address {
            0x6000..=0x7FFF => self.ram.write(address - 0x6000, data),
            0x8000..=0xFFFF => self.rom.write(address - 0x8000, data),
            _ => panic!("Trying to select address '{address}' on BenEaterBus, but it doesn't have a device there!")
        }
    }
}
