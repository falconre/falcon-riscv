use {decode, Op, Register};


/*
   0:   00c58533            add a0,a1,a2
   4:   12358513            addi    a0,a1,291
   8:   002372b3            and t0,t1,sp
   c:   fff1f493            andi    s1,gp,-1
*/


#[test]
fn add() {
    let word: u32 = 0x00c58533;

    let instruction = decode(word).unwrap();

    assert_eq!(*instruction.op(), Op::Add);
    assert_eq!(*instruction.rd(), Register::A0);
    assert_eq!(*instruction.rs1(), Register::A1);
    assert_eq!(*instruction.rs2(), Register::A2);
}


#[test]
fn addi() {
    let word: u32 = 0x12358513;

    let instruction = decode(word).unwrap();

    assert_eq!(*instruction.op(), Op::Addi);
    assert_eq!(*instruction.rd(), Register::A0);
    assert_eq!(*instruction.rs1(), Register::A1);
    assert_eq!(instruction.immediate(), 0x123);
}


#[test]
fn and() {
    let word: u32 = 0x002372b3;

    let instruction = decode(word).unwrap();

    assert_eq!(*instruction.op(), Op::And);
    assert_eq!(*instruction.rd(), Register::T0);
    assert_eq!(*instruction.rs1(), Register::T1);
    assert_eq!(*instruction.rs2(), Register::Sp);
}


#[test]
fn andi() {
    let word: u32 = 0xfff1f493;

    let instruction = decode(word).unwrap();

    assert_eq!(*instruction.op(), Op::Andi);
    assert_eq!(*instruction.rd(), Register::S1);
    assert_eq!(*instruction.rs1(), Register::Gp);
    assert_eq!(instruction.immediate(), -1);
}


#[test]
fn auipc() {
    let word: u32 = 0x12345517;

    let instruction = decode(word).unwrap();

    assert_eq!(*instruction.op(), Op::Auipc);
    assert_eq!(*instruction.rd(), Register::A0);
    assert_eq!(instruction.immediate(), 0x12345);
}


#[test]
fn beq() {
    let word: u32 = 0x00b50663;

    let instruction = decode(word).unwrap();

    assert_eq!(*instruction.op(), Op::Beq);
    assert_eq!(*instruction.rs1(), Register::A0);
    assert_eq!(*instruction.rs2(), Register::A1);
    assert_eq!(instruction.immediate(), 12);
}


#[test]
fn bge() {
    let word: u32 = 0x00b55263;

    let instruction = decode(word).unwrap();

    assert_eq!(*instruction.op(), Op::Beq);
    assert_eq!(*instruction.rs1(), Register::A0);
    assert_eq!(*instruction.rs2(), Register::A1);
    assert_eq!(instruction.immediate(), 4);
}


#[test]
fn bgeu() {
    let word: u32 = 0x00b57263;

    let instruction = decode(word).unwrap();

    assert_eq!(*instruction.op(), Op::Beq);
    assert_eq!(*instruction.rs1(), Register::A0);
    assert_eq!(*instruction.rs2(), Register::A1);
    assert_eq!(instruction.immediate(), 4);
}


#[test]
fn blt() {
    let word: u32 = 0x00b54263;

    let instruction = decode(word).unwrap();

    assert_eq!(*instruction.op(), Op::Beq);
    assert_eq!(*instruction.rs1(), Register::A0);
    assert_eq!(*instruction.rs2(), Register::A1);
    assert_eq!(instruction.immediate(), 4);
}


#[test]
fn bltu() {
    let word: u32 = 0x00b56263;

    let instruction = decode(word).unwrap();

    assert_eq!(*instruction.op(), Op::Beq);
    assert_eq!(*instruction.rs1(), Register::A0);
    assert_eq!(*instruction.rs2(), Register::A1);
    assert_eq!(instruction.immediate(), 4);
}


#[test]
fn bne() {
    let word: u32 = 0x00b51263;

    let instruction = decode(word).unwrap();

    assert_eq!(*instruction.op(), Op::Beq);
    assert_eq!(*instruction.rs1(), Register::A0);
    assert_eq!(*instruction.rs2(), Register::A1);
    assert_eq!(instruction.immediate(), 4);
}