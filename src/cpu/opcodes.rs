pub enum CBType {
    Rotate,
    Shift,
    Swap,
    BitTest,
    BitReset,
    BitSet,
}

#[allow(non_camel_case_types, dead_code)]
pub enum Mnemonic {
    /*
    A8 = Absolute Unsigned 8-Bit Address
    D8 = Immediate Unsigned 8-Bit Data
    R8 = Relative Signed 8-Bit Offset

    A16 = Absolute Unsigned 16-Bit Address
    D16 = Immediate Unsigned 16-Bit Data

    I = Increment
    D = Decrement
    */

    /* Misc/Control */
    NOP,
    STOP,
    HALT,
    CB,
    DI,
    EI,
    /* Load */
    LD_R16_IMM16,
    
    /* Jump */


    /* 8-Bit, Arithmetic/Logical */


    /* 16-Bit, Arithmetic/Logical */


    /* 8-Bit, Bit/Rotations/Shifts */

}

#[derive(Copy, Clone)]
#[allow(non_camel_case_types, dead_code)]
pub enum MnemonicCB {
    RLC_A = 0x07,
    RLC_B = 0x00,
    RLC_C = 0x01,
    RLC_D = 0x02,
    RLC_E = 0x03,
    RLC_H = 0x04,
    RLC_L = 0x05,
    RLC_HL_ = 0x06,

    RRC_A = 0x0F,
    RRC_B = 0x08,
    RRC_C = 0x09,
    RRC_D = 0x0A,
    RRC_E = 0x0B,
    RRC_H = 0x0C,
    RRC_L = 0x0D,
    RRC_HL_ = 0x0E,

    RL_A = 0x17,
    RL_B = 0x10,
    RL_C = 0x11,
    RL_D = 0x12,
    RL_E = 0x13,
    RL_H = 0x14,
    RL_L = 0x15,
    RL_HL_ = 0x16,

    RR_A = 0x1F,
    RR_B = 0x18,
    RR_C = 0x19,
    RR_D = 0x1A,
    RR_E = 0x1B,
    RR_H = 0x1C,
    RR_L = 0x1D,
    RR_HL_ = 0x1E,

    SLA_A = 0x27,
    SLA_B = 0x20,
    SLA_C = 0x21,
    SLA_D = 0x22,
    SLA_E = 0x23,
    SLA_H = 0x24,
    SLA_L = 0x25,
    SLA_HL_ = 0x26,

    SRA_F = 0x2F,
    SRA_B = 0x28,
    SRA_C = 0x29,
    SRA_D = 0x2A,
    SRA_E = 0x2B,
    SRA_H = 0x2C,
    SRA_L = 0x2D,
    SRA_HL_ = 0x2E,

    SWAP_A = 0x37,
    SWAP_B = 0x30,
    SWAP_C = 0x31,
    SWAP_D = 0x32,
    SWAP_E = 0x33,
    SWAP_H = 0x34,
    SWAP_L = 0x35,
    SWAP_HL_ = 0x36,

    SRL_F = 0x3F,
    SRL_B = 0x38,
    SRL_C = 0x39,
    SRL_D = 0x3A,
    SRL_E = 0x3B,
    SRL_H = 0x3C,
    SRL_L = 0x3D,
    SRL_HL_ = 0x3E,

    BIT_0_A = 0x47,
    BIT_0_B = 0x40,
    BIT_0_C = 0x41,
    BIT_0_D = 0x42,
    BIT_0_E = 0x43,
    BIT_0_H = 0x44,
    BIT_0_L = 0x45,
    BIT_0_HL_ = 0x46,

    BIT_1_A = 0x4F,
    BIT_1_B = 0x48,
    BIT_1_C = 0x49,
    BIT_1_D = 0x4A,
    BIT_1_E = 0x4B,
    BIT_1_H = 0x4C,
    BIT_1_L = 0x4D,
    BIT_1_HL_ = 0x4E,

    BIT_2_A = 0x57,
    BIT_2_B = 0x50,
    BIT_2_C = 0x51,
    BIT_2_D = 0x52,
    BIT_2_E = 0x53,
    BIT_2_H = 0x54,
    BIT_2_L = 0x55,
    BIT_2_HL_ = 0x56,

    BIT_3_A = 0x5F,
    BIT_3_B = 0x58,
    BIT_3_C = 0x59,
    BIT_3_D = 0x5A,
    BIT_3_E = 0x5B,
    BIT_3_H = 0x5C,
    BIT_3_L = 0x5D,
    BIT_3_HL_ = 0x5E,

    BIT_4_A = 0x67,
    BIT_4_B = 0x60,
    BIT_4_C = 0x61,
    BIT_4_D = 0x62,
    BIT_4_E = 0x63,
    BIT_4_H = 0x64,
    BIT_4_L = 0x65,
    BIT_4_HL_ = 0x66,

    BIT_5_A = 0x6F,
    BIT_5_B = 0x68,
    BIT_5_C = 0x69,
    BIT_5_D = 0x6A,
    BIT_5_E = 0x6B,
    BIT_5_H = 0x6C,
    BIT_5_L = 0x6D,
    BIT_5_HL_ = 0x6E,

    BIT_6_A = 0x77,
    BIT_6_B = 0x70,
    BIT_6_C = 0x71,
    BIT_6_D = 0x72,
    BIT_6_E = 0x73,
    BIT_6_H = 0x74,
    BIT_6_L = 0x75,
    BIT_6_HL_ = 0x76,

    BIT_7_A = 0x7F,
    BIT_7_B = 0x78,
    BIT_7_C = 0x79,
    BIT_7_D = 0x7A,
    BIT_7_E = 0x7B,
    BIT_7_H = 0x7C,
    BIT_7_L = 0x7D,
    BIT_7_HL_ = 0x7E,

    RES_0_A = 0x87,
    RES_0_B = 0x80,
    RES_0_C = 0x81,
    RES_0_D = 0x82,
    RES_0_E = 0x83,
    RES_0_H = 0x84,
    RES_0_L = 0x85,
    RES_0_HL_ = 0x86,

    RES_1_A = 0x8F,
    RES_1_B = 0x88,
    RES_1_C = 0x89,
    RES_1_D = 0x8A,
    RES_1_E = 0x8B,
    RES_1_H = 0x8C,
    RES_1_L = 0x8D,
    RES_1_HL_ = 0x8E,

    RES_2_A = 0x97,
    RES_2_B = 0x90,
    RES_2_C = 0x91,
    RES_2_D = 0x92,
    RES_2_E = 0x93,
    RES_2_H = 0x94,
    RES_2_L = 0x95,
    RES_2_HL_ = 0x96,

    RES_3_A = 0x9F,
    RES_3_B = 0x98,
    RES_3_C = 0x99,
    RES_3_D = 0x9A,
    RES_3_E = 0x9B,
    RES_3_H = 0x9C,
    RES_3_L = 0x9D,
    RES_3_HL_ = 0x9E,

    RES_4_A = 0xA7,
    RES_4_B = 0xA0,
    RES_4_C = 0xA1,
    RES_4_D = 0xA2,
    RES_4_E = 0xA3,
    RES_4_H = 0xA4,
    RES_4_L = 0xA5,
    RES_4_HL_ = 0xA6,

    RES_5_A = 0xAF,
    RES_5_B = 0xA8,
    RES_5_C = 0xA9,
    RES_5_D = 0xAA,
    RES_5_E = 0xAB,
    RES_5_H = 0xAC,
    RES_5_L = 0xAD,
    RES_5_HL_ = 0xAE,

    RES_6_A = 0xB7,
    RES_6_B = 0xB0,
    RES_6_C = 0xB1,
    RES_6_D = 0xB2,
    RES_6_E = 0xB3,
    RES_6_H = 0xB4,
    RES_6_L = 0xB5,
    RES_6_HL_ = 0xB6,

    RES_7_A = 0xBF,
    RES_7_B = 0xB8,
    RES_7_C = 0xB9,
    RES_7_D = 0xBA,
    RES_7_E = 0xBB,
    RES_7_H = 0xBC,
    RES_7_L = 0xBD,
    RES_7_HL_ = 0xBE,

    SET_0_A = 0xC7,
    SET_0_B = 0xC0,
    SET_0_C = 0xC1,
    SET_0_D = 0xC2,
    SET_0_E = 0xC3,
    SET_0_H = 0xC4,
    SET_0_L = 0xC5,
    SET_0_HL_ = 0xC6,

    SET_1_A = 0xCF,
    SET_1_B = 0xC8,
    SET_1_C = 0xC9,
    SET_1_D = 0xCA,
    SET_1_E = 0xCB,
    SET_1_H = 0xCC,
    SET_1_L = 0xCD,
    SET_1_HL_ = 0xCE,

    SET_2_A = 0xD7,
    SET_2_B = 0xD0,
    SET_2_C = 0xD1,
    SET_2_D = 0xD2,
    SET_2_E = 0xD3,
    SET_2_H = 0xD4,
    SET_2_L = 0xD5,
    SET_2_HL_ = 0xD6,

    SET_3_A = 0xDF,
    SET_3_B = 0xD8,
    SET_3_C = 0xD9,
    SET_3_D = 0xDA,
    SET_3_E = 0xDB,
    SET_3_H = 0xDC,
    SET_3_L = 0xDD,
    SET_3_HL_ = 0xDE,

    SET_4_A = 0xE7,
    SET_4_B = 0xE0,
    SET_4_C = 0xE1,
    SET_4_D = 0xE2,
    SET_4_E = 0xE3,
    SET_4_H = 0xE4,
    SET_4_L = 0xE5,
    SET_4_HL_ = 0xE6,

    SET_5_A = 0xEF,
    SET_5_B = 0xE8,
    SET_5_C = 0xE9,
    SET_5_D = 0xEA,
    SET_5_E = 0xEB,
    SET_5_H = 0xEC,
    SET_5_L = 0xED,
    SET_5_HL_ = 0xEE,

    SET_6_A = 0xF7,
    SET_6_B = 0xF0,
    SET_6_C = 0xF1,
    SET_6_D = 0xF2,
    SET_6_E = 0xF3,
    SET_6_H = 0xF4,
    SET_6_L = 0xF5,
    SET_6_HL_ = 0xF6,

    SET_7_A = 0xFF,
    SET_7_B = 0xF8,
    SET_7_C = 0xF9,
    SET_7_D = 0xFA,
    SET_7_E = 0xFB,
    SET_7_H = 0xFC,
    SET_7_L = 0xFD,
    SET_7_HL_ = 0xFE,
}
// #[allow(dead_code)]
// impl MnemonicCB {
//     pub fn cbtype(&self) -> CBType {
//         match *self as u8 {
//             0x00..=0x1F => CBType::Rotate,   /* RLC, RRC, RL, RR */
//             0x20..=0x2F => CBType::Shift,    /* SLA, SRA, SRL */
//             0x30..=0x37 => CBType::Swap,     /* SWAP */
//             0x40..=0x7F => CBType::BitTest,  /* BIT */
//             0x80..=0xBF => CBType::BitReset, /* RES */
//             0xC0..=0xFF => CBType::BitSet,   /* SET */
//             _ => unreachable!(),
//         }
//     }
// }
