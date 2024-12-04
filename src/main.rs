use std::error::Error;
use std::fmt;
use std::env;
use std::fs;
use std::result;
use std::str::FromStr;

#[derive(Debug)]
struct MachineLine {
    bin: Option<String>,
    comment: Option<String>,
    line_num: u32,
}

#[derive(Debug)]
struct OpLine {
    op: Option<String>,
    arg1: Option<String>,
    arg2: Option<String>,
    arg3: Option<String>,
    comment: Option<String>,
    line_num: u32,
}

#[derive(Debug)]
struct ParsedLine<'a> {
    tokens: Vec<&'a str>,
    comment: Option<String>,
    line_num: u32,
}

#[derive(Debug)]
enum LexError {
    InvalidOperation(String),
    InvalidArgument(String),  // Error with additional info
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LexError::InvalidOperation(ref msg) => write!(f, "Invalid input provided: {msg} is not an operator"),
            LexError::InvalidArgument(ref msg) => write!(f, "Is not an: {}", msg),
        }
    }
}

impl Error for LexError {
    // You can also add additional methods if needed, such as for logging
}



fn parse(line: &str) -> Option<ParsedLine> {
    // remove trailing/leading whitespace
    let line = line.trim();


    // Split the string into two parts: before and after the semicolon
    let (comment, rest) = if let Some(pos) = line.find(';') {
        let rest = &line[..pos];  // The part before the semicolon (trimmed)
        let comment = Some(String::from(&line[pos + 1..]));  // The part after the semicolon (trimmed)
        (comment, rest)
    } else {
        // If no semicolon, return no comment
        (None, line)  // Return the line as "rest"
    };

    // split the remaining string into tokens
    let tokens: Vec<&str> = rest.split_whitespace().collect();

    if comment.is_none() && tokens.len() == 0 {
        None //then the line is empty, return none
    } else {
        Some( ParsedLine {
            tokens,
            comment,
            line_num: 0
        })
        // dbg!(&parsed_line);    
    }
}

use Operation::*;

fn more_lex(op: Operation, parsed_line : ParsedLine ) -> Result<MachineLine, LexError> {

    if op.num_args() != parsed_line.tokens.len() as u8 - 1 {
        return Err(LexError::InvalidOperation("".to_string()));
    }

    let bin = match op {
        NOP => {
            Ok(op.as_opcode().to_owned() + "000000000000")
        }

        BRnzp => {
            // Immediates
            let IMM8 = parsed_line.tokens.get(1).expect("should have 2 args");

            if IMM8.parse::<u8>().is_ok() {
                let code = op.as_opcode().to_owned() + "0000" + format!("{:08b}", IMM8.parse::<u8>().expect("msg")).as_str();
                Ok(code)
            } else {
                Err(LexError::InvalidArgument("Bad Immediate".into()))
            }

        }

        CMP => {
            // CMP Rs, Rt
            let Rs = Register::from_str(parsed_line.tokens.get(1).expect("should have 2 args"))?;
            let Rt = Register::from_str(parsed_line.tokens.get(2).expect("should have 2 args"))?;
            let code= "00100000".to_owned() + Rs.bits() + Rt.bits(); 
            Ok(code)
        }

        ADD | SUB | MUL | DIV  => {
            // ADD Rd, Rs, Rt
            let Rd = Register::from_str(parsed_line.tokens.get(1).expect("should have 2 args"))?;
            let Rs = Register::from_str(parsed_line.tokens.get(2).expect("should have 2 args"))?;
            let Rt = Register::from_str(parsed_line.tokens.get(3).expect("should have 2 args"))?;
            let code= op.as_opcode().to_owned() + Rd.bits() + Rs.bits() + Rt.bits(); 
            Ok(code)
        }

        Operation::LDR => {
            let Rd = Register::from_str(parsed_line.tokens.get(1).expect("should have 2 args"))?;
            let Rs = Register::from_str(parsed_line.tokens.get(2).expect("should have 2 args"))?;
            let code = op.as_opcode().to_owned() + Rd.bits() + Rs.bits() + "0000";
            Ok(code)
        }
        
        Operation::STR => {
            let Rs = Register::from_str(parsed_line.tokens.get(1).expect("should have 2 args"))?;
            let Rt = Register::from_str(parsed_line.tokens.get(2).expect("should have 2 args"))?;
            let code = op.as_opcode().to_owned() + "0000" + Rs.bits() + Rt.bits();
            Ok(code)
            
        }
        Operation::CONST => {
            let Rd = Register::from_str(parsed_line.tokens.get(1).expect("should have 2 args"))?;
            let IMM8 = parsed_line.tokens.get(1).expect("should have 2 args");


            if IMM8.parse::<u8>().is_ok() {
                let code = op.as_opcode().to_owned() + Rd.bits() + format!("{:08b}", IMM8.parse::<u8>().expect("msg")).as_str();
                Ok(code + IMM8)
            } else {
                Err(LexError::InvalidArgument("Bad Immediate".into()))
            }
        }
        Operation::RET => {
            Ok(op.as_opcode().to_owned() + "000000000000")
        }
    };


    assert!(bin.as_ref().expect("msg").len() == 16); //maybe correct?

    if bin.is_ok() {
        Ok (MachineLine {
            bin: bin.ok(),
            comment: parsed_line.comment,
            line_num: parsed_line.line_num,
        })
    } else {
        Err(bin.err().unwrap())
    }
}

fn lex(parsed_line : ParsedLine) -> Result<MachineLine, LexError> {

    let op = parsed_line.tokens.get(0);

    match op {
        // If there are no code tokens in the line...
        None => Ok(MachineLine {
            bin: None,
            comment: parsed_line.comment,
            line_num: parsed_line.line_num,
        }),  

        // if there are code tokens in the line...
        Some(inner) => {

            let mapped = Operation::from_str(inner);

            if mapped.is_ok() {
                more_lex(mapped.unwrap(), parsed_line)
            } else {
                // if the first token is invalid
                println!("{}", inner);
                println!("Code cannot be mapped!");
                Err(LexError::InvalidOperation(inner.to_string()))
            }
                
        }
    }
}




fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.lines();

    
    let parsed_lines :Vec<ParsedLine<'_>>= lines.filter_map(|line| parse(line)).collect();
    dbg!(&parsed_lines);

    for line in parsed_lines {
        match lex(line) {
            Ok(magic_sparkles) => {

                let bin = match magic_sparkles.bin {
                    Some(bin) => bin ,
                    None => "".to_owned(),
                };

                let comment = match magic_sparkles.comment {
                    Some(comment) => comment ,
                    None => "".to_owned(),
                };
                
                println!("0b{}, # ; {}", bin, comment);
            },
            Err(scary_monsters) => {
                dbg!(scary_monsters);
            },
        } 
    };


    println!("In file {file_path}");
}


#[derive(Debug)]
enum Operation {
    NOP,    // No operation
    BRnzp,  // Branch if the condition codes are non-zero (in ARM-like assembly)
    CMP,    // Compare
    ADD,    // Addition
    SUB,    // Subtraction
    MUL,    // Multiplication
    DIV,    // Division
    LDR,    // Load from memory
    STR,    // Store to memory
    CONST,  // Load constant
    RET,    // Return from function
}

impl Operation {
    // Return a string representation of the operation
    fn name(&self) -> &'static str {
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

    fn as_opcode(&self) -> &'static str {
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

    fn num_args(&self) -> u8 {
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
    fn from_str(s: &str) -> Result<Self, ()> {
        match s {
            "NOP" => Ok(Operation::NOP),
            "BRnzp" => Ok(Operation::BRnzp),
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

enum Register {
    // General-purpose read/write registers
    R0, R1, R2, R3, R4, R5, R6, R7, R8, R9, R10, R11, R12,
    
    // Special-purpose read-only registers
    BlockIdx,  // %blockIdx
    BlockDim,  // %blockDim
    ThreadIdx, // %threadIdx
}

impl Register {
    // A method to return the name of the register as a string for easier printing
    fn name(&self) -> &'static str {
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

    fn bits(&self) -> &'static str{
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

    fn from_str(s: &str) -> Result<Self, LexError> {
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
            _ => Err(LexError::InvalidArgument("Invalid register string".to_string())),
        }
    }
}