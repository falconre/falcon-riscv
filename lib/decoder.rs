use {Fence, Instruction, Op, Register};


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OpType {
    R,
    I,
    S,
    Sb,
    U,
    Uj,
    Special
}


/// This struct is used to hold instruction information for decoding
#[derive(Clone, Debug)]
struct Opcode {
    funct3: u32,
    funct7: u32,
    opcode: u32,
    op: Op,
    op_type: OpType,
}


use self::OpType::*;
use Op::*;


const RV32I_OPCODES: &'static [Opcode] = &[
    Opcode { funct7: 0,         funct3: 0,      opcode: 0b0110111, op: Lui,    op_type: U },
    Opcode { funct7: 0,         funct3: 0,      opcode: 0b0010111, op: Auipc,  op_type: U },
    Opcode { funct7: 0,         funct3: 0,      opcode: 0b1101111, op: Jal,    op_type: Uj },
    Opcode { funct7: 0,         funct3: 0b000,  opcode: 0b1100111, op: Jalr,   op_type: S },
    Opcode { funct7: 0,         funct3: 0b000,  opcode: 0b1100011, op: Beq,    op_type: OpType::Sb },
    Opcode { funct7: 0,         funct3: 0b001,  opcode: 0b1100011, op: Bne,    op_type: OpType::Sb },
    Opcode { funct7: 0,         funct3: 0b100,  opcode: 0b1100011, op: Blt,    op_type: OpType::Sb },
    Opcode { funct7: 0,         funct3: 0b101,  opcode: 0b1100011, op: Bge,    op_type: OpType::Sb },
    Opcode { funct7: 0,         funct3: 0b110,  opcode: 0b1100011, op: Bltu,   op_type: OpType::Sb },
    Opcode { funct7: 0,         funct3: 0b111,  opcode: 0b1100011, op: Bgeu,   op_type: OpType::Sb },
    Opcode { funct7: 0,         funct3: 0b000,  opcode: 0b0000011, op: Lb,     op_type: I },
    Opcode { funct7: 0,         funct3: 0b001,  opcode: 0b0000011, op: Lh,     op_type: I },
    Opcode { funct7: 0,         funct3: 0b010,  opcode: 0b0000011, op: Lw,     op_type: I },
    Opcode { funct7: 0,         funct3: 0b100,  opcode: 0b0000011, op: Lbu,    op_type: I },
    Opcode { funct7: 0,         funct3: 0b101,  opcode: 0b0000011, op: Lhu,    op_type: I },
    Opcode { funct7: 0,         funct3: 0b000,  opcode: 0b0100011, op: Op::Sb, op_type: OpType::Sb },
    Opcode { funct7: 0,         funct3: 0b001,  opcode: 0b0100011, op: Sh,     op_type: OpType::Sb },
    Opcode { funct7: 0,         funct3: 0b010,  opcode: 0b0100011, op: Sw,     op_type: OpType::Sb },
    Opcode { funct7: 0,         funct3: 0b000,  opcode: 0b0010011, op: Addi,   op_type: I },
    Opcode { funct7: 0,         funct3: 0b010,  opcode: 0b0010011, op: Slti,   op_type: I },
    Opcode { funct7: 0,         funct3: 0b011,  opcode: 0b0010011, op: Sltiu,  op_type: I },
    Opcode { funct7: 0,         funct3: 0b100,  opcode: 0b0010011, op: Xori,   op_type: I },
    Opcode { funct7: 0,         funct3: 0b110,  opcode: 0b0010011, op: Ori,    op_type: I },
    Opcode { funct7: 0,         funct3: 0b111,  opcode: 0b0010011, op: Andi,   op_type: I },
    Opcode { funct7: 0b0000000, funct3: 0b001,  opcode: 0b0010011, op: Slli,   op_type: R },
    Opcode { funct7: 0b0000000, funct3: 0b101,  opcode: 0b0010011, op: Srli,   op_type: R },
    Opcode { funct7: 0b0100000, funct3: 0b101,  opcode: 0b0010011, op: Srai,   op_type: R },
    Opcode { funct7: 0b0000000, funct3: 0b000,  opcode: 0b0110011, op: Add,    op_type: R },
    Opcode { funct7: 0b0100000, funct3: 0b000,  opcode: 0b0110011, op: Sub,    op_type: R },
    Opcode { funct7: 0b0000000, funct3: 0b001,  opcode: 0b0110011, op: Sll,    op_type: R },
    Opcode { funct7: 0b0000000, funct3: 0b010,  opcode: 0b0110011, op: Slt,    op_type: R },
    Opcode { funct7: 0b0000000, funct3: 0b011,  opcode: 0b0110011, op: Sltu,   op_type: R },
    Opcode { funct7: 0b0000000, funct3: 0b100,  opcode: 0b0110011, op: Xor,    op_type: R },
    Opcode { funct7: 0b0000000, funct3: 0b101,  opcode: 0b0010011, op: Srl,    op_type: R },
    Opcode { funct7: 0b0100000, funct3: 0b101,  opcode: 0b0010011, op: Sra,    op_type: R },
    Opcode { funct7: 0b0000000, funct3: 0b110,  opcode: 0b0110011, op: Or,     op_type: R },
    Opcode { funct7: 0b0000000, funct3: 0b111,  opcode: 0b0110011, op: And,    op_type: R },
    Opcode { funct7: 0,         funct3: 0b000,  opcode: 0b0001111, op: Fence,  op_type: I },
    Opcode { funct7: 0,         funct3: 0b001,  opcode: 0b0001111, op: FenceI, op_type: I },
    Opcode { funct7: 0,         funct3: 0b000,  opcode: 0b1110011, op: Ecall,  op_type: Special },
    Opcode { funct7: 0,         funct3: 0b000,  opcode: 0b1110011, op: Ebreak, op_type: Special },
    Opcode { funct7: 0,         funct3: 0b001,  opcode: 0b1110011, op: Csrrw,  op_type: Special },
    Opcode { funct7: 0,         funct3: 0b010,  opcode: 0b1110011, op: Csrrs,  op_type: Special },
    Opcode { funct7: 0,         funct3: 0b011,  opcode: 0b1110011, op: Csrrc,  op_type: Special },
    Opcode { funct7: 0,         funct3: 0b101,  opcode: 0b1110011, op: Csrrwi, op_type: Special },
    Opcode { funct7: 0,         funct3: 0b110,  opcode: 0b1110011, op: Csrrsi, op_type: Special },
    Opcode { funct7: 0,         funct3: 0b111,  opcode: 0b1110011, op: Csrrci, op_type: Special }
];


fn decode_r(opcode: &Opcode, word: u32) -> Instruction {
    let rs2 = (word >> 20) & 0x1f;
    let rs1 = (word >> 15) & 0x1f;
    let rd = (word >> 7) & 0x1f;

    Instruction {
        rs2: Register::from_u32(rs2),
        rs1: Register::from_u32(rs1),
        rd: Register::from_u32(rd),
        csr: 0,
        op: opcode.op.clone(),
        shamt: 0,
        immediate: 0,
        fence: None
    }
}


fn decode_i(opcode: &Opcode, word: u32) -> Instruction {
    let rs1 = (word >> 15) & 0x1f;
    let rd = (word >> 7) & 0x1f;

    let imm = (word >> 20) & 0xfff;
    let mut immediate: isize = imm as isize;
    if imm & 0x800 > 0 {
        immediate = 0 - (0x1000 - immediate);
    }

    Instruction {
        rs2: Register::Invalid,
        rs1: Register::from_u32(rs1),
        rd: Register::from_u32(rd),
        csr: 0,
        op: opcode.op.clone(),
        shamt: 0,
        immediate: immediate,
        fence: None
    }
}


fn decode_s(opcode: &Opcode, word: u32) -> Instruction {
    let rs1 = (word >> 15) & 0x1f;
    let rs2 = (word >> 20) & 0x1f;

    let mut imm = (word >> 20) & 0xfe0;
    imm |= (word >> 7) & 0x1f;
    let mut immediate: isize = imm as isize;
    if imm & 0x800 > 0 {
        immediate = 0 - (0x1000 - immediate);
    }

    Instruction {
        rs2: Register::from_u32(rs2),
        rs1: Register::from_u32(rs1),
        rd: Register::Invalid,
        csr: 0,
        op: opcode.op.clone(),
        shamt: 0,
        immediate: immediate,
        fence: None
    }
}


fn decode_sb(opcode: &Opcode, word: u32) -> Instruction {
    let rs1 = (word >> 15) & 0x1f;
    let rs2 = (word >> 20) & 0x1f;

    let mut imm = ((word >> 7) & 1) << 11;
    imm |= ((word >> 8) & 0xf) << 1;
    imm |= ((word >> 25) & 0x1f) << 5;
    imm |= ((word >> 31) & 1) << 12;
    let mut immediate: isize = imm as isize;
    if imm & 0x800 > 0 {
        immediate = 0 - (0x1000 - immediate);
    }
    Instruction {
        rs2: Register::from_u32(rs2),
        rs1: Register::from_u32(rs1),
        rd: Register::Invalid,
        csr: 0,
        op: opcode.op.clone(),
        shamt: 0,
        immediate: immediate,
        fence: None
    }
}


fn decode_u(opcode: &Opcode, word: u32) -> Instruction {
    let rd = (word >> 7) & 0x1f;

    let imm = word >> 12;
    let immediate: isize = imm as isize;
    Instruction {
        rs2: Register::Invalid,
        rs1: Register::Invalid,
        rd: Register::from_u32(rd),
        csr: 0,
        op: opcode.op.clone(),
        shamt: 0,
        immediate: immediate,
        fence: None
    }
}


fn decode_uj(opcode: &Opcode, word: u32) -> Instruction {
    let rd = (word >> 7) & 0x1f;

    let mut imm = word & 0x000f_f000;
    imm |= ((word >> 20) & 1) << 11;
    imm |= ((word >> 21) & 0x3ff) << 1;
    imm |= ((word >> 31) & 1) << 20;

    let immediate: isize = imm as isize;

    Instruction {
        rs2: Register::Invalid,
        rs1: Register::Invalid,
        rd: Register::from_u32(rd),
        csr: 0,
        op: opcode.op.clone(),
        shamt: 0,
        immediate: immediate,
        fence: None
    }
}


fn decode_fence(opcode: &Opcode, word: u32) -> Instruction {
    let rs1 = (word >> 15) & 0x1f;
    let rd = (word >> 7) & 0x1f;

    let fence = Fence {
        pi: if ((word >> 27) & 1) == 1 { true } else { false },
        po: if ((word >> 26) & 1) == 1 { true } else { false },
        pr: if ((word >> 25) & 1) == 1 { true } else { false },
        pw: if ((word >> 24) & 1) == 1 { true } else { false },
        si: if ((word >> 23) & 1) == 1 { true } else { false },
        so: if ((word >> 22) & 1) == 1 { true } else { false },
        sr: if ((word >> 21) & 1) == 1 { true } else { false },
        sw: if ((word >> 20) & 1) == 1 { true } else { false },
    };

    Instruction {
        rs2: Register::Invalid,
        rs1: Register::from_u32(rs1),
        rd: Register::from_u32(rd),
        csr: 0,
        op: opcode.op.clone(),
        shamt: 0,
        immediate: 0,
        fence: Some(fence)
    }
}


fn decode_special(opcode: &Opcode, word: u32) -> Instruction {

    match opcode.op {
        Op::Ecall |
        Op::Ebreak => {
            let op =
                if (word >> 20) & 0xfff == 0 {
                    Op::Ecall
                } else {
                    Op::Ebreak
                };
            Instruction {
                rs2: Register::Invalid,
                rs1: Register::Invalid,
                rd: Register::Invalid,
                csr: 0,
                op: op,
                shamt: 0,
                immediate: 0,
                fence: None
            }
        },
        Op::Csrrc |
        Op::Csrrs |
        Op::Csrrw => {
            let rs1 = (word >> 15) & 0x1f;
            let rd = (word >> 7) & 0x1f;
            let csr = (word >> 20) & 0xfff;
            Instruction {
                rs2: Register::Invalid,
                rs1: Register::from_u32(rs1),
                rd: Register::from_u32(rd),
                csr: csr as usize,
                op: opcode.op.clone(),
                shamt: 0,
                immediate: 0,
                fence: None
            }
        },
        Op::Csrrci |
        Op::Csrrsi |
        Op::Csrrwi => {
            let imm = (word >> 15) & 0x1f;
            let rd = (word >> 7) & 0x1f;
            let csr = (word >> 20) & 0xfff;
            Instruction {
                rs2: Register::Invalid,
                rs1: Register::Invalid,
                rd: Register::from_u32(rd),
                csr: csr as usize,
                op: opcode.op.clone(),
                shamt: 0,
                immediate: imm as isize,
                fence: None
            }
        },
        _ => panic!("decode_special called with an invalid opcode")
    }

}


pub fn decode(word: u32) -> Option<Instruction> {
    let opcode = word & 0x7f;
    let funct3 = (word >> 12) & 0x7;
    let funct7 = (word >> 25) & 0x7f;

    for o in RV32I_OPCODES {
        // println!(
        //     "{:?} {:b}=={:b} {:b}=={:b} {:b}=={:b}",
        //     o.op,
        //     opcode, o.opcode,
        //     funct3, o.funct3,
        //     funct7, o.funct7);
        match o.op_type {
            OpType::R => {
                if opcode == o.opcode &&
                   funct3 == o.funct3 &&
                   funct7 == o.funct7 {
                    return Some(decode_r(o, word));
                }
            },
            OpType::I => {
                if opcode == o.opcode && funct3 == o.funct3 {
                    if o.op == Fence || o.op == FenceI {
                        return Some(decode_fence(o, word));
                    }
                    return Some(decode_i(o, word));
                }
            },
            OpType::S => {
                if opcode == o.opcode && funct3 == o.funct3 {
                    return Some(decode_s(o, word));
                }
            },
            OpType::Sb => {
                if opcode == o.opcode {
                    return Some(decode_sb(o, word));
                }
            },
            OpType::U => {
                if opcode == o.opcode {
                    return Some(decode_u(o, word));
                }
            },
            OpType::Uj => {
                if opcode == o.opcode {
                    return Some(decode_uj(o, word));
                }
            },
            OpType::Special => {
                if opcode == o.opcode && funct3 == o.funct3 {
                    return Some(decode_special(o, word));
                }
            }
        }
    }

    None
}