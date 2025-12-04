pub enum Flag {
    Z = 0b10000000,
    N = 0b01000000,
    H = 0b00100000,
    C = 0b00010000,
}
#[allow(non_snake_case)]
pub struct Register {
    PC: u16,
    SP: u16,
    A: u8,
    F: u8,
    B: u8,
    C: u8,
    D: u8,
    E: u8,
    H: u8,
    L: u8,
    pub IE: u8,
}
#[allow(non_snake_case)]
impl Register {
    pub fn new() -> Self {
        Self {
            PC: 0,
            SP: 0,
            A: 0,
            F: 0,
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            H: 0,
            L: 0,
            IE: 0,
        }
    }
    /*  ---------- 16-bit Register Getters ---------- */
    pub fn AF(&self) -> u16 {
        ((self.A as u16) << 8) | (self.F as u16)
    }
    pub fn BC(&self) -> u16 {
        ((self.B as u16) << 8) | (self.C as u16)
    }
    pub fn DE(&self) -> u16 {
        ((self.D as u16) << 8) | (self.E as u16)
    }
    pub fn HL(&self) -> u16 {
        ((self.H as u16) << 8) | (self.L as u16)
    }
    /*  ---------- 16-bit Register Setters ---------- */
    pub fn set_AF(&mut self, value: u16) {
        self.A = ((value >> 8) & 0xFF) as u8;
        self.F = (value & 0xF0) as u8;
    }
    pub fn set_BC(&mut self, value: u16) {
        self.B = ((value >> 8) & 0xFF) as u8;
        self.C = (value & 0xFF) as u8;
    }
    pub fn set_DE(&mut self, value: u16) {
        self.D = ((value >> 8) & 0xFF) as u8;
        self.E = (value & 0xFF) as u8;
    }
    pub fn set_HL(&mut self, value: u16) {
        self.H = ((value >> 8) & 0xFF) as u8;
        self.L = (value & 0xFF) as u8;
    }
    /* ---------- Flag Helpers ----------  */
    pub fn get_flag(&self, flag: Flag) -> bool {
        (self.F & flag as u8) != 0
    }
    pub fn set_flag(&mut self, flag: Flag) {
        self.F |= flag as u8;
    }
    pub fn clear_flag(&mut self, flag: Flag) {
        self.F &= !(flag as u8);
    }
    pub fn write_flag(&mut self, flag: Flag, state: bool) {
        if state {
            self.set_flag(flag)
        } else {
            self.clear_flag(flag)
        }
    }
}
