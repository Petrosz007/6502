use crate::bus::Bus;

mod instructions;

const STACK_SIZE: usize = 0xFF + 1;
const DEFAULT_STATUS: u8 = 0b0010_0000;
const RESET_STATUS: u8 = 0b0011_0100;

pub struct CPU {
    /// Program Counter
    pc: u16,
    /// Stack Pointer
    s: u8,
    /// Accumulator
    a: u8,
    /// Index Register X
    x: u8,
    /// Index Register Y
    y: u8,
    /// Processor Status
    status: u8,
    /// Stack
    /// Addressed between $0100 and $01FF
    stack: [u8; STACK_SIZE],
    /// Number of procerror cycles executed
    cycles: usize,
    /// Main Bus
    bus: Box<dyn Bus>,
}

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
enum StatusFlag {
    /// The carry flag is set if the last operation caused an overflow from bit 7 of the result or an underflow from bit 0.
    /// This condition is set during arithmetic, comparison and during logical shifts.
    /// It can be explicitly set using the 'Set Carry Flag' (SEC) instruction and cleared with 'Clear Carry Flag' (CLC).
    CarryFlag = 0,
    /// The zero flag is set if the result of the last operation as was zero.
    ZeroFlag = 1,
    /// The interrupt disable flag is set if the program has executed a 'Set Interrupt Disable' (SEI) instruction.
    /// While this flag is set the processor will not respond to interrupts from devices until it is cleared by a 'Clear Interrupt Disable' (CLI) instruction.
    InterruptDisable = 2,
    /// While the decimal mode flag is set the processor will obey the rules of Binary Coded Decimal (BCD) arithmetic during addition and subtraction.
    /// The flag can be explicitly set using 'Set Decimal Flag' (SED) and cleared with 'Clear Decimal Flag' (CLD).
    DecimalMode = 3,
    /// The break command bit is set when a BRK instruction has been executed and an interrupt has been generated to process it.
    BreakCommand = 4,
    /// The overflow flag is set during arithmetic operations if the result has yielded an invalid 2's complement result
    /// (e.g. adding to positive numbers and ending up with a negative result: 64 + 64 => -128).
    /// It is determined by looking at the carry between bits 6 and 7 and between bit 7 and the carry flag.
    OverflowFlag = 6,
    /// The negative flag is set if the result of the last operation had bit 7 set to a one.
    NegativeFlag = 7,
}

impl CPU {
    pub fn new(bus: Box<dyn Bus>) -> CPU {
        CPU {
            pc: 0,
            s: 0,
            a: 0,
            x: 0,
            y: 0,
            status: DEFAULT_STATUS,
            stack: [0; STACK_SIZE],
            cycles: 0,
            bus,
        }
    }

    pub fn reset(&mut self) {
        self.s = 0;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.status = RESET_STATUS;
        self.stack = [0; STACK_SIZE];
        self.cycles = 0;
        self.pc = self.bus.read(0xFFFC) as u16 | ((self.bus.read(0xFFFD) as u16) << 8);
    }

    pub fn fetch_and_execute_instruction(&mut self) {
        let instruction = self.bus.read(self.pc);
        // println!(
        //     "pc: {:x} opcode: {instruction:x} a: {:x} x: {:x} y: {:x} cycle: {}",
        //     self.pc, self.a, self.x, self.y, self.cycles
        // );

        match instruction {
            0x4C => self.op_4c(),
            0x6A => self.op_6a(),
            0x8D => self.op_8d(),
            0xA2 => self.op_a2(),
            0xA9 => self.op_a9(),
            0xBD => self.op_bd(),
            0xE8 => self.op_e8(),
            0xEA => self.op_ea(),
            0xF0 => self.op_f0(),
            unknown_opcode => panic!(
                "Unknown op code encountered '{unknown_opcode:x}' at memory location {:x}",
                self.pc
            ),
        }
    }

    fn get_status_flag(&mut self, flag: StatusFlag) -> u8 {
        (self.status & (1 << flag as u8)) >> flag as u8
    }

    /// The value must be 0 or 1
    fn set_status_flag(&mut self, flag: StatusFlag, value: u8) {
        if value == 1 {
            self.status |= 1 << (flag as u8)
        } else {
            self.status &= !(1 << (flag as u8))
        }
    }

    fn get_immediate(&mut self) -> u8 {
        self.bus.read(self.pc + 1)
    }

    fn get_absolute(&mut self) -> u16 {
        self.bus.read(self.pc + 1) as u16 | ((self.bus.read(self.pc + 2) as u16) << 8)
    }

    fn get_absolute_x(&mut self) -> u8 {
        let absolute_address = self.get_absolute();
        self.bus.read(absolute_address + self.x as u16)
    }

    /// We have to handle this number as a signed integer, so i8, because the relative offset can be -128 to +127
    fn get_relative(&mut self) -> i8 {
        self.bus.read(self.pc + 1) as i8
    }
}
