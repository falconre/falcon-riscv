mod decoder;
mod instruction;
mod register;

pub use decoder::decode;
pub use instruction::{Fence, Instruction, Op};
pub use register::Register;

#[cfg(test)]
mod test;
