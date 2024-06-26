use std::{fs, thread, time::Duration};

use devices::{console::Console, ram::RAM, rom::ROM};

use crate::{bus::BenEaterBus, cpu::CPU};

mod bus;
mod cpu;
mod devices;

fn main() {
    let program =
        fs::read("programs/target/hello_world.rom").expect("The file should be read correctly");

    let ram = RAM::new();
    let console = Console::new();
    let rom = ROM::from_raw_vec(program);
    let bus = BenEaterBus::new(ram, console, rom);

    let mut cpu = CPU::new(Box::new(bus));

    cpu.reset();

    // let start = SystemTime::now();
    // for iteration in 0..1_000_000_000usize {
    //     // thread::sleep(time::Duration::from_millis(100));
    //     cpu.fetch_and_execute_instruction();
    //     // println!("ROM: {:?}", rom);
    //     if iteration % 90_000_000 == 0 {
    //         let now = SystemTime::now();
    //         print!(
    //             "\rRAM: {:08b}  Cycle: {} speed: {} MHz",
    //             // ram.borrow_mut().data[0],
    //             0,
    //             cpu.cycles,
    //             iteration as f64 / now.duration_since(start).unwrap().as_secs_f64() / 1_000_000f64
    //         );
    //         io::stdout().flush().unwrap();
    //     }
    // }
    loop {
        cpu.fetch_and_execute_instruction();
        thread::sleep(Duration::from_millis(50));
    }
}
