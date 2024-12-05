use std::any::Any;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

pub mod operation;
use crate::operation::Operation;

#[derive(Debug)]
pub struct MachineLine {
    pub line_type: LineType,
    pub parsed_line: ParsedLine,
    pub bin: Option<String>,
    pub comment: Option<String>,
    pub line_num: u32,
}

#[derive(Debug, Clone)]
pub struct ParsedLine {
    pub tokens: Vec<String>,
    pub comment: Option<String>,
    pub line_num: u32,
}

pub fn identify_line(line: ParsedLine) -> Box<dyn LexedLine> {
    match line.tokens.first() {
        Some(first_token) => {
            let is_memory_dir = |a: &String| a.chars().next().is_some_and(|char_1| char_1 == '.'); //checks if the first token is a memory directive
            let is_label = |a: &String| a.chars().last().is_some_and(|char_1| char_1 == ':'); //checks if the first token is a label

            if is_memory_dir(first_token) {
                Box::new(MemoryLine { parsed: line })
            } else if is_label(first_token) {
                Box::new(LabelLine { parsed: line })
            } else if Operation::from_str(first_token).is_ok() {
                Box::new(OperationLine {
                    parsed: line,
                    instruct_num: None,
                })
            } else {
                Box::new(BadLine { parsed: line })
            }
        }
        None => Box::new(HumanLine { parsed: line }),
    }
}

/// Types of Lexed and Parsed Lines
/// ---
///

// Define a trait that the struct will implement
pub trait LexedLine: std::fmt::Debug + std::any::Any {
    // fn parsed(&self) -> &ParsedLine {
    //     self.parsed
    // };
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn parsed(&self) -> &ParsedLine;
}

// this is so verbose it's actually crazy. Just know that this exists so that I can downcast the generic "LexedLine" type
impl LexedLine for OperationLine {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn parsed(&self) -> &ParsedLine {
        &self.parsed
    }
}

impl LexedLine for LabelLine {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn parsed(&self) -> &ParsedLine {
        &self.parsed
    }
}

impl LexedLine for MemoryLine {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn parsed(&self) -> &ParsedLine {
        &self.parsed
    }
}

impl LexedLine for HumanLine {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn parsed(&self) -> &ParsedLine {
        &self.parsed
    }
}

impl LexedLine for BadLine {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn parsed(&self) -> &ParsedLine {
        &self.parsed
    }
}
#[derive(Debug, PartialEq)]
pub enum LineType {
    Operation,
    Bad,
    Human,
    Memory,
    Label,
}

#[derive(Debug)]
pub struct BadLine {
    pub parsed: ParsedLine,
}
#[derive(Debug)]
pub struct OperationLine {
    pub parsed: ParsedLine,
    pub instruct_num: Option<u16>,
}

#[derive(Debug)]
pub struct HumanLine {
    // human lines are either comment only, or whitespace only
    pub parsed: ParsedLine,
}
#[derive(Debug)]
pub struct MemoryLine {
    pub parsed: ParsedLine,
}

#[derive(Debug)]
pub struct LabelLine {
    pub parsed: ParsedLine,
}

/// Custom Error Type
/// ---

#[derive(Debug)]
pub enum LexError {
    InvalidOperation(String),
    InvalidArgument(String), // Error with additional info
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LexError::InvalidOperation(ref msg) => {
                write!(f, "Invalid input provided: {msg} is not an operator")
            }
            LexError::InvalidArgument(ref msg) => write!(f, "Is not an: {}", msg),
        }
    }
}

impl Error for LexError {
    // You can also add additional methods if needed, such as for logging
}

/// Register Definitions
/// ---

pub enum Register {
    // General-purpose read/write registers
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,

    // Special-purpose read-only registers
    BlockIdx,  // %blockIdx
    BlockDim,  // %blockDim
    ThreadIdx, // %threadIdx
}

impl Register {
    // A method to return the name of the register as a string for easier printing
    pub fn name(&self) -> &'static str {
        match self {
            Register::R0 => "R0",
            Register::R1 => "R1",
            Register::R2 => "R2",
            Register::R3 => "R3",
            Register::R4 => "R4",
            Register::R5 => "R5",
            Register::R6 => "R6",
            Register::R7 => "R7",
            Register::R8 => "R8",
            Register::R9 => "R9",
            Register::R10 => "R10",
            Register::R11 => "R11",
            Register::R12 => "R12",
            Register::BlockIdx => "%blockIdx",
            Register::BlockDim => "%blockDim",
            Register::ThreadIdx => "%threadIdx",
        }
    }

    pub fn bits(&self) -> &'static str {
        match self {
            Register::R0 => "0000",
            Register::R1 => "0001",
            Register::R2 => "0010",
            Register::R3 => "0011",
            Register::R4 => "0100",
            Register::R5 => "0101",
            Register::R6 => "0110",
            Register::R7 => "0111",
            Register::R8 => "1000",
            Register::R9 => "1001",
            Register::R10 => "1010",
            Register::R11 => "1011",
            Register::R12 => "1100",
            Register::BlockIdx => "1101",
            Register::BlockDim => "1110",
            Register::ThreadIdx => "1111",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, LexError> {
        let no_commas = s.replace(",", ""); //remove commas (allows for comma or no commas) should probably sophisticate later
        match no_commas.as_str() {
            "R0" => Ok(Register::R0),
            "R1" => Ok(Register::R1),
            "R2" => Ok(Register::R2),
            "R3" => Ok(Register::R3),
            "R4" => Ok(Register::R4),
            "R5" => Ok(Register::R5),
            "R6" => Ok(Register::R6),
            "R7" => Ok(Register::R7),
            "R8" => Ok(Register::R8),
            "R9" => Ok(Register::R9),
            "R10" => Ok(Register::R10),
            "R11" => Ok(Register::R11),
            "R12" => Ok(Register::R12),
            "%blockIdx" => Ok(Register::BlockIdx),
            "%blockDim" => Ok(Register::BlockDim),
            "%threadIdx" => Ok(Register::ThreadIdx),
            _ => Err(LexError::InvalidArgument(
                "Invalid register string".to_string(),
            )),
        }
    }
}
