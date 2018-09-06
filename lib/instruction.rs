use Register;
use std::fmt;


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Op {
    Add,
    Addi,
    And,
    Andi,
    Auipc,
    Beq,
    Bge,
    Bgeu,
    Blt,
    Bltu,
    Bne,
    Csrrc,
    Csrrci,
    Csrrs,
    Csrrsi,
    Csrrw,
    Csrrwi,
    Ebreak,
    Ecall,
    Fence,
    FenceI,
    Jal,
    Jalr,
    Lb,
    Lbu,
    Lh,
    Lhu,
    Lw,
    Lui,
    Or,
    Ori,
    RdCycle,
    RdCycleH,
    RdTime,
    RdTimeH,
    RdInstRet,
    RdInstRetH,
    Sb,
    Sh,
    Sw,
    Sll,
    Slli,
    Slt,
    Slti,
    Sltu,
    Sltiu,
    Sra,
    Srai,
    Srl,
    Srli,
    Sub,
    Xor,
    Xori,
}


#[derive(Clone, Debug)]
pub struct Fence {
    pub(crate) pi: bool,
    pub(crate) po: bool,
    pub(crate) pr: bool,
    pub(crate) pw: bool,
    pub(crate) si: bool,
    pub(crate) so: bool,
    pub(crate) sr: bool,
    pub(crate) sw: bool
}

impl Fence {
    pub fn pi(&self) -> bool { self.pi }
    pub fn po(&self) -> bool { self.po }
    pub fn pr(&self) -> bool { self.pr }
    pub fn pw(&self) -> bool { self.pw }
    pub fn si(&self) -> bool { self.si }
    pub fn so(&self) -> bool { self.so }
    pub fn sr(&self) -> bool { self.sr }
    pub fn sw(&self) -> bool { self.sw }
}


/// A decoded instruction
#[derive(Clone, Debug)]
pub struct Instruction {
    pub(crate) rs2: Register,
    pub(crate) rs1: Register,
    pub(crate) rd: Register,
    pub(crate) csr: usize,
    pub(crate) op: Op,
    pub(crate) shamt: usize,
    pub(crate) immediate: isize,
    pub(crate) fence: Option<Fence>
}


impl Instruction {
    pub fn rs2(&self) -> &Register { &self.rs2 }
    pub fn rs1(&self) -> &Register { &self.rs1 }
    pub fn rd(&self) -> &Register { &self.rd }
    pub fn csr(&self) -> usize { self.csr }
    pub fn op(&self) -> &Op { &self.op }
    pub fn shamt(&self) -> usize { self.shamt }
    pub fn immediate(&self) -> isize { self.immediate }
    pub fn fence(&self) -> Option<&Fence> { self.fence.as_ref() }
}


impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.op() {
            Op::Add =>
                write!(f, "add {}, {}, {}", self.rd(), self.rs1(), self.rs2()),
            Op::Addi =>
                write!(f, "addi {}, {}, {}", self.rd(), self.rs1(), self.immediate()),
            Op::And =>
                write!(f, "and {}, {}, {}", self.rd(), self.rs1(), self.rs2()),
            Op::Andi =>
                write!(f, "andi {}, {}, {}", self.rd(), self.rs1(), self.immediate()),
            Op::Auipc =>
                write!(f, "auipc {}, 0x{:x}", self.rd(), self.immediate()),
            Op::Beq =>
                write!(f, "beq {}, {}, {}", self.rs1(), self.rs2(), self.immediate()),
            Op::Bge =>
                write!(f, "bge {}, {}, {}", self.rs1(), self.rs2(), self.immediate()),
            Op::Bgeu =>
                write!(f, "bgeu {}, {}, {}", self.rs1(), self.rs2(), self.immediate()),
            Op::Blt =>
                write!(f, "blt {}, {}, {}", self.rs1(), self.rs2(), self.immediate()),
            Op::Bltu =>
                write!(f, "bltu {}, {}, {}", self.rs1(), self.rs2(), self.immediate()),
            Op::Bne =>
                write!(f, "bne {}, {}, {}", self.rs1(), self.rs2(), self.immediate()),
            _ => unimplemented!()
        }
    }
}