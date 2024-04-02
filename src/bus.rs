use crate::devices::{console::Console, ram::RAM, rom::ROM, Device};

pub trait Bus {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
}

pub struct BenEaterBus {
    ram: RAM,
    console: Console,
    rom: ROM,
}

impl BenEaterBus {
    pub fn new(ram: RAM, console: Console, rom: ROM) -> BenEaterBus {
        BenEaterBus { ram, console, rom }
    }
}

impl Bus for BenEaterBus {
    fn read(&mut self, address: u16) -> u8 {
        // println!("Reading from BUS: '{address:x}'");
        match address {
            0x0000..=0x3FFF => self.ram.read(address - 0x0000),
            0x6000..=0x600F => self.console.read(address - 0x6000),
            0x8000..=0xFFFF => self.rom.read(address - 0x8000),
            _ => panic!("Trying to select address '{address}' on BenEaterBus, but it doesn't have a device there!")
        }
    }

    fn write(&mut self, address: u16, data: u8) {
        // println!("Writing to BUS: '{address:x}'");
        match address {
            0x0000..=0x3FFF => self.ram.write(address - 0x0000, data),
            0x6000..=0x600F => self.console.write(address - 0x6000, data),
            0x8000..=0xFFFF => self.rom.write(address - 0x8000, data),
            _ => panic!("Trying to select address '{address}' on BenEaterBus, but it doesn't have a device there!")
        }
    }
}
