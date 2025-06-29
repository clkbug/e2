use std::ffi::os_str::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reg {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,
}

impl std::fmt::Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reg_name = match self {
            Reg::X0 => "x0",
            Reg::X1 => "x1",
            Reg::X2 => "x2",
            Reg::X3 => "x3",
            Reg::X4 => "x4",
            Reg::X5 => "x5",
            Reg::X6 => "x6",
            Reg::X7 => "x7",
            Reg::X8 => "x8",
            Reg::X9 => "x9",
            Reg::X10 => "x10",
            Reg::X11 => "x11",
            Reg::X12 => "x12",
            Reg::X13 => "x13",
            Reg::X14 => "x14",
            Reg::X15 => "x15",
            Reg::X16 => "x16",
            Reg::X17 => "x17",
            Reg::X18 => "x18",
            Reg::X19 => "x19",
            Reg::X20 => "x20",
            Reg::X21 => "x21",
            Reg::X22 => "x22",
            Reg::X23 => "x23",
            Reg::X24 => "x24",
            Reg::X25 => "x25",
            Reg::X26 => "x26",
            Reg::X27 => "x27",
            Reg::X28 => "x28",
            Reg::X29 => "x29",
            Reg::X30 => "x30",
            Reg::X31 => "x31",
        };
        write!(f, "{reg_name}")
    }
}

const REGISTER_COUNT: usize = 32;

pub enum Instruction {
    Add(Reg, Reg, Reg),
}

impl Instruction {
    pub fn to_string(&self) -> String {
        match self {
            Instruction::Add(rd, rs1, rs2) => format!(
                "add {}, {}, {}",
                rd.to_string(),
                rs1.to_string(),
                rs2.to_string()
            ),
        }
    }

    fn to_bytes_r(rd: Reg, rs1: Reg, rs2: Reg, funct3: u32, funct7: u32, opcode: u32) -> u32 {
        let rd_num = rd as u8;
        let rs1_num = rs1 as u8;
        let rs2_num = rs2 as u8;
        (funct7 as u32) << 25
            | (rs2_num as u32) << 20
            | (rs1_num as u32) << 15
            | (funct3 << 12)
            | (rd_num as u32) << 7
            | opcode
    }

    pub fn to_bytes(&self) -> u32 {
        match self {
            Instruction::Add(rd, rs1, rs2) => {
                Self::to_bytes_r(*rd, *rs1, *rs2, 0b000, 0b0000000, 0b0110011)
            }
        }
    }
}

pub struct Block {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

impl Block {
    pub fn new(name: String) -> Self {
        Block {
            name,
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }

    pub fn to_string(&self) -> String {
        let mut result = format!("{}:\n", self.name);
        for inst in &self.instructions {
            result.push_str(&inst.to_string());
            result.push('\n');
        }
        result
    }

    pub fn to_hex(&self) -> String {
        let mut result = String::new();
        for inst in &self.instructions {
            result.push_str(&format!("{:08x}\n", inst.to_bytes()));
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_to_string() {
        let inst = Instruction::Add(Reg::X1, Reg::X2, Reg::X3);
        assert_eq!(inst.to_string(), "add x1, x2, x3");
        let inst = Instruction::Add(Reg::X4, Reg::X5, Reg::X6);
        assert_eq!(inst.to_string(), "add x4, x5, x6");
    }
}
