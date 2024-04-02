use std::io::{self, Stdout, Write};

use super::Device;

// This Console implementation is far from Ben's IO latch and LCD module, but that's a bit too much work to emulate, maybe later
pub struct Console {
    stdout: Stdout,
}

impl Console {
    pub fn new() -> Console {
        let mut console = Console {
            stdout: io::stdout(),
        };

        // Clear the screen
        // console.write(b"\x1b[H\x1b[2J");

        console
    }

    fn write(&mut self, buf: &[u8]) {
        self.stdout
            .write_all(buf)
            .expect("Writing to the console should not fail");
        self.stdout
            .flush()
            .expect("Flushing Stdout should not fail");
    }
}

impl Device for Console {
    fn read(&mut self, _address: u16) -> u8 {
        0
    }

    fn write(&mut self, address: u16, data: u8) {
        match address {
            // Control operations
            0x0000 => {
                // Clear the line
                if data & 0b0000_0001 == 1 {
                    self.write(b"\x1b[2K\r");
                }
            }
            // Write the data byte to the Console, encoded in ASCII
            0x0001 => {
                self.write(&[data]);
            }
            _ => panic!("Trying to write to unsupported region of the Console '{address:04x}'"),
        }
    }
}
