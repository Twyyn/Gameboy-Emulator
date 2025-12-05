use crate::cpu::bitflags::bitflags;

bitflags! {
    pub struct Flags: u8 {
        /* Flag Register (F) */
        const Z = 1 << 7; /* Zero */
        const N = 1 << 6; /* Subtract */
        const H = 1 << 5; /* Half Carry */
        const C = 1 << 4; /* Carry */
    }
}
#[allow(non_snake_case)]
pub struct Registers {
    pub PC: u16,
    pub SP: u16,
    pub A: u8,
    F: Flags,
    pub B: u8,
    pub C: u8,
    pub D: u8,
    pub E: u8,
    pub H: u8,
    pub L: u8,
    pub IE: u8,
}
#[allow(non_snake_case)]
impl Registers {
    pub fn new() -> Self {
        Self {
            PC: 0,
            SP: 0,
            A: 0,
            F: Flags::empty(),
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            H: 0,
            L: 0,
            IE: 0,
        }
    }
    /*  ---------- 16-Bit Register Getters ---------- */
    pub fn AF(&self) -> u16 {
        ((self.A as u16) << 8) | (self.F.bits() as u16)
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
    /*  ---------- 16-Bit Register Setters ---------- */
    pub fn set_AF(&mut self, value: u16) {
        self.A = ((value >> 8) & 0xFF) as u8;
        self.set_F(value as u8);
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
    /* ---------- Flag Getters---------  */
    pub fn F(&self) -> u8 {
        /* Helper */
        self.F.bits()
    }
    fn flag(&self, flag: Flags) -> bool {
        /* Helper */
        self.F.contains(flag)
    }
    pub fn Z_flag(&self) -> bool {
        self.flag(Flags::Z)
    }
    pub fn N_flag(&self) -> bool {
        self.flag(Flags::N)
    }
    pub fn H_flag(&self) -> bool {
        self.flag(Flags::H)
    }
    pub fn C_flag(&self) -> bool {
        self.flag(Flags::C)
    }
    /* Flag Setters */
    fn set_F(&mut self, value: u8) {
        /* Helper */
        self.F = Flags::from_bits_truncate(value & 0xF0);
    }
    pub fn set_flag(&mut self, flag: Flags) {
        self.F.insert(flag);
    }
    pub fn set_flags(&mut self, Z: bool, N: bool, H: bool, C: bool) {
        let mut flags = Flags::empty();
        if Z {
            flags.insert(Flags::Z);
        }
        if N {
            flags.insert(Flags::N);
        }
        if H {
            flags.insert(Flags::H);
        }
        if C {
            flags.insert(Flags::C);
        }
        self.set_F(flags.bits())
    }
    /* Clear/Write Flag */
    fn clear_flag(&mut self, flag: Flags) {
        /* Helper */
        self.F.remove(flag);
    }
    pub fn update_flag(&mut self, flag: Flags, state: bool) {
        if state {
            self.set_flag(flag)
        } else {
            self.clear_flag(flag)
        }
    }
}
impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}
