#[derive(Debug)]
pub enum Operation {
    NOP,   // No operation
    BRnzp, // Branch if the condition codes are non-zero (in ARM-like assembly)
    CMP,   // Compare
    ADD,   // Addition
    SUB,   // Subtraction
    MUL,   // Multiplication
    DIV,   // Division
    LDR,   // Load from memory
    STR,   // Store to memory
    CONST, // Load constant
    RET,   // Return from function
}

impl Operation {
    // Return a string representation of the operation
    pub fn name(&self) -> &'static str {
        match self {
            Operation::NOP => "NOP",
            Operation::BRnzp => "BRnzp",
            Operation::CMP => "CMP",
            Operation::ADD => "ADD",
            Operation::SUB => "SUB",
            Operation::MUL => "MUL",
            Operation::DIV => "DIV",
            Operation::LDR => "LDR",
            Operation::STR => "STR",
            Operation::CONST => "CONST",
            Operation::RET => "RET",
        }
    }

    pub fn as_opcode(&self) -> &'static str {
        match self {
            Operation::NOP => "0000",
            Operation::BRnzp => "0001",
            Operation::CMP => "0010",
            Operation::ADD => "0011",
            Operation::SUB => "0100",
            Operation::MUL => "0101",
            Operation::DIV => "0110",
            Operation::LDR => "0111",
            Operation::STR => "1000",
            Operation::CONST => "1001",
            Operation::RET => "1111",
        }
    }

    pub fn num_args(&self) -> u8 {
        match self {
            Operation::NOP => 0,
            Operation::BRnzp => 1,
            Operation::CMP => 2,
            Operation::ADD => 3,
            Operation::SUB => 3,
            Operation::MUL => 3,
            Operation::DIV => 3,
            Operation::LDR => 2,
            Operation::STR => 2,
            Operation::CONST => 2,
            Operation::RET => 0,
        }
    }

    // A method to parse a string into an Operation enum
    pub fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "NOP" => Ok(Operation::NOP),
            "BRnzp" => Ok(Operation::BRnzp),
            "BRn" => Ok(Operation::BRnzp), //not super clear from documentation which is "proper" or if both are OK
            "CMP" => Ok(Operation::CMP),
            "ADD" => Ok(Operation::ADD),
            "SUB" => Ok(Operation::SUB),
            "MUL" => Ok(Operation::MUL),
            "DIV" => Ok(Operation::DIV),
            "LDR" => Ok(Operation::LDR),
            "STR" => Ok(Operation::STR),
            "CONST" => Ok(Operation::CONST),
            "RET" => Ok(Operation::RET),
            _ => Err(()),
        }
    }
}
