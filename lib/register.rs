use std::fmt;


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Register {
    Zero, // x0
    Ra,   // x1
    Sp,   // x2
    Gp,   // x3
    Tp,   // x4
    T0,   // x5
    T1,   // x6
    T2,   // x7
    Fp,   // x8
    S1,   // x9
    A0,   // x10
    A1,   // x11
    A2,  // x12
    A3,  // x13
    A4,   // x14
    A5,   // x15
    A6,   // x16
    A7,   // x17
    S2,   // x18
    S3,   // x19
    S4,   // x20
    S5,   // x21
    S6,   // x22
    S7,   // x23
    S8,   // x24
    S9,   // x25
    S10,   // x26
    S11,   // x27
    T3,   // x28
    T4,   // x29
    T5,   // x30
    T6,   // x31
    Invalid
}


impl Register {
    pub(crate) fn from_u32(u: u32) -> Register {
        match u {
            0  => Register::Zero,
            1  => Register::Ra,
            2  => Register::Sp,
            3  => Register::Gp,
            4  => Register::Tp,
            5  => Register::T0,
            6  => Register::T1,
            7  => Register::T2,
            8  => Register::Fp,
            9  => Register::S1,
            10 => Register::A0,
            11 => Register::A1,
            12 => Register::A2,
            13 => Register::A3,
            14 => Register::A4,
            15 => Register::A5,
            16 => Register::A6,
            17 => Register::A7,
            18 => Register::S2,
            19 => Register::S3,
            20 => Register::S4,
            21 => Register::S5,
            22 => Register::S6,
            23 => Register::S7,
            24 => Register::S8,
            25 => Register::S9,
            26 => Register::S10,
            27 => Register::S11,
            28 => Register::T3,
            29 => Register::T4,
            30 => Register::T5,
            31 => Register::T6,
            _ => Register::Invalid
        }
    }
}


impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Register::Zero => write!(f, "0"),
            Register::Ra => write!(f, "ra"),
            Register::Sp => write!(f, "sp"),
            Register::Gp => write!(f, "gp"),
            Register::Tp => write!(f, "tp"),
            Register::Fp => write!(f, "fp"),
            Register::S1 => write!(f, "s1"),
            Register::S2 => write!(f, "s2"),
            Register::S3 => write!(f, "s3"),
            Register::S4 => write!(f, "s4"),
            Register::S5 => write!(f, "s5"),
            Register::S6 => write!(f, "s6"),
            Register::S7 => write!(f, "s7"),
            Register::S8 => write!(f, "s8"),
            Register::S9 => write!(f, "s9"),
            Register::S10 => write!(f, "s10"),
            Register::S11 => write!(f, "s11"),
            Register::A0 => write!(f, "a0"),
            Register::A1 => write!(f, "a1"),
            Register::A2 => write!(f, "a2"),
            Register::A3 => write!(f, "a3"),
            Register::A4 => write!(f, "a4"),
            Register::A5 => write!(f, "a5"),
            Register::A6 => write!(f, "a6"),
            Register::A7 => write!(f, "a7"),
            Register::T0 => write!(f, "t0"),
            Register::T1 => write!(f, "t1"),
            Register::T2 => write!(f, "t2"),
            Register::T3 => write!(f, "t3"),
            Register::T4 => write!(f, "t4"),
            Register::T5 => write!(f, "t5"),
            Register::T6 => write!(f, "t6"),
            Register::Invalid => write!(f, "invalid")
        }
    }
}