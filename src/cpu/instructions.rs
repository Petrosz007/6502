use super::StatusFlag::*;
use crate::cpu::CPU;

impl CPU {
    /// BEQ
    pub(super) fn op_f0(&mut self) {
        if self.get_status_flag(ZeroFlag) == 0b1 {
            self.pc = self.pc.wrapping_add_signed(self.get_relative() as i16);
            self.cycles += 1; // TODO: +1 if to a new page
        }

        self.pc += 2;
        self.cycles += 2;
    }

    /// NOP
    pub(super) fn op_ea(&mut self) {
        self.pc += 1
    }

    /// LDA, Immediate
    pub(super) fn op_a9(&mut self) {
        self.a = self.get_immediate();

        self.set_status_flag(ZeroFlag, if self.a == 0b0 { 0b1 } else { 0b0 });
        self.set_status_flag(NegativeFlag, self.a >> 7);

        self.pc += 2;
        self.cycles += 2;
    }

    /// LDA, Absolute,X
    pub(super) fn op_bd(&mut self) {
        self.a = self.get_absolute_x();

        self.set_status_flag(ZeroFlag, if self.a == 0b0 { 0b1 } else { 0b0 });
        self.set_status_flag(NegativeFlag, self.a >> 7);

        self.pc += 3;
        self.cycles += 4; // TODO: +1 if page is crossed
    }

    /// LDX, Immediate
    pub(super) fn op_a2(&mut self) {
        self.x = self.get_immediate();

        self.set_status_flag(ZeroFlag, if self.x == 0b0 { 0b1 } else { 0b0 });
        self.set_status_flag(NegativeFlag, self.x >> 7);

        self.pc += 2;
        self.cycles += 2;
    }

    /// INX
    pub(super) fn op_e8(&mut self) {
        self.x += 1;

        self.set_status_flag(ZeroFlag, if self.x == 0b0 { 0b1 } else { 0b0 });
        self.set_status_flag(NegativeFlag, self.x >> 7);

        self.pc += 1;
        self.cycles += 2;
    }

    /// JMP, Absolute
    pub(super) fn op_4c(&mut self) {
        self.pc = self.get_absolute();
        self.cycles += 3;
    }

    // ROR, Accumulator
    pub(super) fn op_6a(&mut self) {
        let new_a = (self.a >> 1) | (self.get_status_flag(CarryFlag) << 7);

        self.set_status_flag(CarryFlag, self.a & 0b1);
        self.set_status_flag(ZeroFlag, new_a & 0b0);
        self.set_status_flag(NegativeFlag, new_a >> 7);

        self.a = new_a;
        self.pc += 1;
        self.cycles += 2;
    }

    /// STA, Absolute
    pub(super) fn op_8d(&mut self) {
        let address = self.get_absolute();
        self.bus.write(address, self.a);
        self.pc += 3;
        self.cycles += 4;
    }
}
