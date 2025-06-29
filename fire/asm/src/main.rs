use asmlib::riscv::{Instruction, Reg};

fn main() {
    let mut block = asmlib::riscv::Block::new("main".to_string());
    let inst1 = Instruction::Add(Reg::X1, Reg::X2, Reg::X3);
    let inst2 = Instruction::Add(Reg::X4, Reg::X5, Reg::X6);
    block.add_instruction(inst1);
    block.add_instruction(inst2);
    println!("{}", block.to_string());
    println!("{}", block.to_hex());
}
