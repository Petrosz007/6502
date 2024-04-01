use super::Device;

const ROM_CAPACITY: usize = (0xFFFF >> 1) + 1;

/// This ROM has a capacity of 32KB
#[derive(Debug)]
pub struct ROM {
    pub data: [u8; ROM_CAPACITY],
}

impl ROM {
    pub fn new<const T: usize>(initial_rom: [u8; T]) -> ROM {
        let mut data = [0xEA_u8; ROM_CAPACITY]; // Filling the ROM with NOP
        data[..T].copy_from_slice(&initial_rom);

        // Tells the CPU to start reading the program from $8000
        data[0x7FFC] = 0x00;
        data[0x7FFD] = 0x80;

        ROM { data }
    }

    pub fn from_vec(initial_rom: Vec<u8>) -> ROM {
        assert!(initial_rom.len() <= ROM_CAPACITY, "Rom loaded from file is bigger than the ROM capacity! ROM capacity is {ROM_CAPACITY}, loaded rom size is {}", initial_rom.len());
        let mut data = [0xEA_u8; ROM_CAPACITY]; // Filling the ROM with NOP
        data[..initial_rom.len()].copy_from_slice(&initial_rom);

        // Tells the CPU to start reading the program from $8000
        data[0x7FFC] = 0x00;
        data[0x7FFD] = 0x80;

        ROM { data }
    }

    pub fn from_raw_vec(initial_rom: Vec<u8>) -> ROM {
        let initial_rom_size = initial_rom.len();
        ROM { data: initial_rom.try_into().unwrap_or_else(|_| panic!("Rom loaded from file is not exactly the same size as the ROM capacity! ROM capacity is {ROM_CAPACITY}, loaded rom size is {initial_rom_size}")) }
    }
}

impl Device for ROM {
    fn read(&mut self, address: u16) -> u8 {
        // println!("Reading from ROM: '{address:x}'");
        self.data[address as usize]
    }

    /// ROM is read only, so this is a no-op
    fn write(&mut self, _address: u16, _data: u8) {}
}
