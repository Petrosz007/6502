pub mod console;
pub mod ram;
pub mod rom;

pub trait Device {
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
}
