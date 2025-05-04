use std::env;
use std::fs;
use std::ops::Index;
use std::str::FromStr;

use lib::operation::Operation;
use lib::operation::Operation::*;
use lib::*;
use serde::Serialize;
use std::path::Path;

fn parse_line(line_num: usize, line: &str) -> Option<ParsedLine> {
    // remove trailing/leading whitespace
    let line = line.trim();

    // Split the string into two parts: before and after the semicolon
    let (comment, rest) = if let Some(pos) = line.find(';') {
        let rest = &line[..pos]; // The part before the semicolon (trimmed)
        let comment = Some(String::from(&line[pos + 1..])); // The part after the semicolon (trimmed)
        (comment, rest)
    } else {
        // If no semicolon, return no comment
        (None, line) // Return the line as "rest"
    };

    let tokens: Vec<&str> = rest.split_whitespace().collect::<Vec<&str>>();
    let a: Vec<String> = tokens.into_iter().map(|a| a.to_owned()).collect();

    Some(ParsedLine {
        tokens: a,
        comment,
        line_num: (line_num as u32),
    })
}

fn operation_conv(
    mut lexed_line: Box<dyn LexedLine>,
    label_addresses: Vec<(String, u16)>,
) -> Result<MachineLine, LexError> {
    // operation_conv(a, &operation.parsed, label_numbers, &lexed_lines)
    // OperationType, &parsedline, label_nums, &lexedlines

    if lexed_line
        .as_any_mut()
        .downcast_mut::<OperationLine>()
        .is_none()
    {
        // determine the correct type
        let typ: LineType;
        if lexed_line.as_any_mut().downcast_mut::<BadLine>().is_some() {
            typ = LineType::Bad;
        } else if lexed_line
            .as_any_mut()
            .downcast_mut::<HumanLine>()
            .is_some()
        {
            typ = LineType::Human;
        } else if lexed_line
            .as_any_mut()
            .downcast_mut::<OperationLine>()
            .is_some()
        {
            typ = LineType::Operation;
        } else if lexed_line
            .as_any_mut()
            .downcast_mut::<MemoryLine>()
            .is_some()
        {
            typ = LineType::Memory;
        } else if lexed_line
            .as_any_mut()
            .downcast_mut::<LabelLine>()
            .is_some()
        {
            typ = LineType::Label;
        } else {
            return Err(LexError::InvalidArgument("terrible, man".to_owned()));
        }
        return {
            Ok(MachineLine {
                line_type: typ,
                parsed_line: lexed_line.parsed().clone(),
                bin: None,
                comment: lexed_line.parsed().comment.clone(),
                line_num: lexed_line.parsed().line_num,
            })
        };

        // {Ok (MachineLine {
        //     bin: None,
        //     comment: parsed_line.comment,
        //     line_num: parsed_line.line_num,
        //     parsed_line: parsed_line,
        //     } ) };

        // return Err(LexError::InvalidArgument("".to_string()));
    }

    let operation_line = lexed_line.as_any().downcast_ref::<OperationLine>().unwrap();
    let op: Operation = Operation::from_str(operation_line.parsed.tokens.first().unwrap()).unwrap();
    let parsed_line = operation_line.parsed.clone();

    if op.num_args() != parsed_line.tokens.len() as u8 - 1 {
        return Err(LexError::InvalidOperation("".to_string()));
    }

    let operands = &parsed_line.tokens; // this is actually the operator AND the operands. anyway...

    // define closure that (semi)gracefully handles errors that should never occur, rather than making the below match statement unreadable
    let get_operand_from_ind = |index: u16| {
        operands
            .get(index as usize)
            .unwrap_or_else(|| panic!("should have {} args", op.num_args()))
    };

    let bin = match op {
        NOP => Ok(op.as_opcode().to_owned() + "000000000000"),

        BRnzp => {
            //// NZP Shenanigans

            dbg!("BRnzp binary here");
            let branch_instr_string = get_operand_from_ind(0);
            // Extract the "nzp" flags
            let nzp_flags: String = branch_instr_string
                .chars()
                .filter(|&c| c == 'n' || c == 'z' || c == 'p')
                .collect();

            // dbg!(nzp_flags);
            // panic!();
            // Default to "1000" if no flags are found, as per the spec
            let nzp = if nzp_flags.is_empty() {
                "0000".to_string() // Default value if no flags are present
            } else {
                // Here you could further process or map the flags into the correct string
                // For simplicity, just use the flags directly
                format!(
                    "{:04b}",
                    nzp_flags.chars().fold(0, |acc, flag| {
                        (acc << 1)
                            | match flag {
                                'n' => 8,
                                'z' => 4,
                                'p' => 2,
                                _ => 0,
                            }
                    })
                )
            };

            if nzp == "0000" {
                return Err(LexError::InvalidArgument("Branch instruction with no NZP flags - will never branch. Did you mean to branch in all cases? (BRnzp)".into()));
            }

            //// end of nzp shenanigans

            // Get the operand (the label or flags part)
            let req_label = get_operand_from_ind(1); // WARN:
            dbg!("label is: {}", req_label);

            if let Some(jump_addr) = label_addresses
                .iter()
                .filter_map(|(label, line_num)| {
                    if *req_label == *label {
                        Some(*line_num)
                    } else {
                        None
                    }
                })
                .next()
            {
                let code =
                    op.as_opcode().to_owned() + &nzp + format!("{:08b}", jump_addr as u16).as_str();
                Ok(code)
            } else {
                Err(LexError::InvalidArgument("Bad Immediate".into()))
            }
        }

        CMP => {
            // CMP Rs, Rt
            let rs = Register::from_str(get_operand_from_ind(1))?;
            let rt = Register::from_str(get_operand_from_ind(2))?;
            let code = "00100000".to_owned() + rs.bits() + rt.bits();
            Ok(code)
        }

        ADD | SUB | MUL | DIV => {
            // ADD Rd, Rs, Rt
            let rd = Register::from_str(get_operand_from_ind(1))?;
            let rs = Register::from_str(get_operand_from_ind(2))?;
            let rt = Register::from_str(get_operand_from_ind(3))?;
            let code = op.as_opcode().to_owned() + rd.bits() + rs.bits() + rt.bits();
            Ok(code)
        }

        Operation::LDR => {
            let rd = Register::from_str(get_operand_from_ind(1))?;
            let rs = Register::from_str(get_operand_from_ind(2))?;
            let code = op.as_opcode().to_owned() + rd.bits() + rs.bits() + "0000";
            Ok(code)
        }

        Operation::STR => {
            let rs = Register::from_str(get_operand_from_ind(1))?;
            let rt = Register::from_str(get_operand_from_ind(2))?;
            let code = op.as_opcode().to_owned() + "0000" + rs.bits() + rt.bits();
            Ok(code)
        }
        Operation::CONST => {
            let rd = Register::from_str(get_operand_from_ind(1))?;
            let imm8 = get_operand_from_ind(2);
            let imm8 = imm8.replace("#", "");

            if imm8.parse::<u8>().is_ok() {
                let code = op.as_opcode().to_owned()
                    + rd.bits()
                    + format!("{:08b}", imm8.parse::<u8>().expect("msg")).as_str();
                Ok(code)
            } else {
                Err(LexError::InvalidArgument("Bad Immediate".into()))
            }
        }
        Operation::RET => Ok(op.as_opcode().to_owned() + "000000000000"),
    };

    //// test assertions that produced binary is accurate
    assert!(bin.as_ref().expect("msg").len() == 16); //maybe correct?

    if bin.is_ok() {
        Ok(MachineLine {
            line_type: LineType::Operation,
            bin: bin.ok(),
            comment: parsed_line.comment.clone(),
            line_num: parsed_line.line_num,
            parsed_line,
        })
    } else {
        Err(bin.err().unwrap())
    }
}

fn extract_label_assoc_lines(lexed_lines: &Vec<Box<dyn LexedLine>>) -> Vec<(String, u32)> {
    // handle Memory and labels (labels must be done prior to operations)
    let mut labels_lines: Vec<(String, u32)> = vec![]; //maps a Label String to the next line number which is a valid operation

    for line in lexed_lines {
        if let Some(label_line) = line.as_any().downcast_ref::<LabelLine>() {
            // handle all label lines
            if label_line.parsed.tokens.len() != 1 {
                todo!()
            } else {
                let label = label_line
                    .parsed
                    .tokens
                    .first()
                    .unwrap()
                    .clone()
                    .replace(":", "");
                let num = label_line.parsed.line_num;

                //// go through the parsed lines and find the next operation line
                let line_number: u32; //a place to store the line number we find in the following loop
                let mut i = num; //loop variable

                loop {
                    let check_line = lexed_lines.index(i as usize);
                    if let Some(a) = check_line.as_any().downcast_ref::<OperationLine>() {
                        //you found it!
                        line_number = a.parsed.line_num;
                        break;
                    }

                    i += 1;
                }

                labels_lines.push((label, line_number));
            }
        }
    }

    labels_lines
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = &args[1];
    let output_path = &args[3];

    if "-o" != &args[2] {
        eprintln!(
            "Error: expected '-o' as the third argument, got '{}'",
            &args[2]
        );
        std::process::exit(1);
    }

    let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");

    let lines = contents.lines();

    //start with list of lines (Label/operation/none/error)+comment?
    //handle all cases with labels/memory
    //handle all cases with operations
    //print all lines, exluding those with no comments and no statements

    let parsed_lines: Vec<ParsedLine> = lines
        .enumerate()
        .filter_map(|(line_num, line)| parse_line(line_num, line))
        .collect();
    // dbg!(&parsed_lines);

    let mut lexed_lines: Vec<Box<dyn LexedLine>> = parsed_lines
        .into_iter()
        .map(|item| identify_line(item))
        .collect();

    // dbg!(&lexed_lines);

    let label_lines = extract_label_assoc_lines(&lexed_lines);

    //// now that the label list is generated, it's possible to statically point branch/jump instructs
    // number the instructions based on the actual address (is there a better method? almost certainly!)
    let mut i: u16 = 0;
    for line in &mut lexed_lines {
        if let Some(operation_line) = line.as_any_mut().downcast_mut::<OperationLine>() {
            operation_line.instruct_num = Some(i);
            i += 1;
        }
    }

    //for u32 in label lines, do lexed_lines.get(u32) convert to operation line fmt and take its instruct number)

    let label_addresses: Vec<(String, u16)> = label_lines
        .into_iter()
        .map(|(a, b)| {
            (
                a,
                lexed_lines
                    .get(b as usize)
                    .unwrap()
                    .as_any()
                    .downcast_ref::<OperationLine>()
                    .unwrap()
                    .instruct_num
                    .unwrap(),
            )
        })
        .collect(); //todo!() complete conversion into address num (baked into struct)

    // dbg!(&label_addresses);

    let mut operations = Vec::new();
    let mut memories = Vec::new();

    for line in lexed_lines {
        match operation_conv(line, label_addresses.clone()) {
            Ok(line) => match line.line_type {
                LineType::Operation => operations.push(line),
                LineType::Memory => memories.push(line),
                _ => {}
            },
            Err(err) => {
                eprintln!("Error assembling line: {:?}", err);
                std::process::exit(1);
            }
        }
    }

    dbg!(&memories);
    dbg!(&operations);

    // Extract threads count (from .threads directive)
    let threads = memories
        .iter()
        .find(|m| m.parsed_line.tokens.first().unwrap() == ".threads")
        .and_then(|m| m.parsed_line.tokens.get(1))
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(1);

    // Extract .data values as a vector of u8

    let initial_data: Vec<u8> = memories
        .iter()
        .filter(|m| m.parsed_line.tokens.first().map(|t| t.as_str()) == Some(".data"))
        .flat_map(|m| {
            m.parsed_line
                .tokens
                .iter()
                .skip(1) // skip ".data"
                .map(|tok| tok.parse::<u8>().expect("Invalid u8 in .data"))
        })
        .collect();

    // Convert operations to hex strings
    let program_memory: Vec<String> = operations
        .into_iter()
        .map(|m| {
            let bin = m.bin.unwrap();
            let value = u16::from_str_radix(&bin, 2).unwrap();
            format!("0x{:04x}", value)
        })
        .collect();

    //dbg!(&initial_data);

    #[derive(Serialize)]
    struct Hardware {
        program_addr_bits: u32,
        program_data_bits: u32,
        program_channels: u32,
        data_addr_bits: u32,
        data_data_bits: u32,
        data_channels: u32,
    }

    #[derive(Serialize)]
    struct Output {
        testname: String,
        memory_delay: u32,
        threads: u32,
        hardware: Hardware,
        program_memory: Vec<String>,
        initial_data: Vec<u8>,
    }

    // Build output
    let testname = Path::new(input_path)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let hardware = Hardware {
        program_addr_bits: 8,
        program_data_bits: 16,
        program_channels: 1,
        data_addr_bits: 8,
        data_data_bits: 8,
        data_channels: 4,
    };
    let output = Output {
        testname,
        memory_delay: 1, // makes for faster tests
        threads,
        hardware,
        program_memory,
        initial_data,
    };

    // Print JSON to stdout
    std::fs::write(output_path, serde_json::to_string_pretty(&output).unwrap()).unwrap();
}
