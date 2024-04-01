use crate::cpu::CPU;

impl CPU {
    /// NOP
    pub(super) fn op_ea(&mut self) {
        self.pc += 1
    }

    /// LDA, Immediate
    pub(super) fn op_a9(&mut self) {
        self.a = self.get_immediate();
        self.pc += 2;
        self.cycles += 2;
    }

    /// STA, Absolute
    pub(super) fn op_8d(&mut self) {
        let address = self.get_absolute();
        self.bus.write(address, self.a);
        self.pc += 3;
        self.cycles += 4;
    }

    /// JMP, Absolute
    pub(super) fn op_4c(&mut self) {
        self.pc = self.get_absolute();
        self.cycles += 3;
    }
}
