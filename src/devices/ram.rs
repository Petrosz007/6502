use std::io::Write;

use super::Device;

const RAM_SIZE: usize = 0xFFFF + 1;

#[derive(Debug)]
pub struct RAM {
    pub data: [u8; RAM_SIZE],
}

impl RAM {
    pub fn new() -> RAM {
        RAM {
            data: [0; RAM_SIZE],
        }
    }
}

impl Device for RAM {
    fn read(&mut self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn write(&mut self, address: u16, data: u8) {
        self.data[address as usize] = data;
        // TODO: For debugging, only when memory is written, display it. This is how Ben did it in his video with LEDs
        print!("\r{data:08b}");
        std::io::stdout().flush().unwrap();
    }
}
