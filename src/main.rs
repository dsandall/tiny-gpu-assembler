use std::env;
use std::fs;
use std::ops::Index;
use std::str::FromStr;

use lib::operation::Operation;
use lib::operation::Operation::*;
use lib::*;

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

fn operation_conv<'a>(
    mut lexed_line: Box<dyn LexedLine>,
    label_addresses: Vec<(String, u16)>,
) -> Result<MachineLine, LexError> {
    // operation_conv(a, &operation.parsed, label_numbers, &lexed_lines)
    // OperationType, &parsedline, label_nums, &lexedlines

    if lexed_line.as_any_mut().downcast_mut::<OperationLine>().is_none() {
        // determine the correct type
        let typ: LineType;
        if lexed_line.as_any_mut().downcast_mut::<BadLine>().is_some() {
            typ = LineType::Bad;
        } else if lexed_line.as_any_mut().downcast_mut::<HumanLine>().is_some() {
            typ = LineType::Human;
        } else if lexed_line.as_any_mut().downcast_mut::<OperationLine>().is_some() {
            typ = LineType::Operation;
        } else if lexed_line.as_any_mut().downcast_mut::<MemoryLine>().is_some() {
            typ = LineType::Memory;
        } else if lexed_line.as_any_mut().downcast_mut::<LabelLine>().is_some() {
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
                "0000".to_string()  // Default value if no flags are present
            } else {
                // Here you could further process or map the flags into the correct string
                // For simplicity, just use the flags directly
                format!("{:04b}", nzp_flags.chars().fold(0, |acc, flag| {
                    (acc << 1) | match flag {
                        'n' => 8,
                        'z' => 4,
                        'p' => 2,
                        _ => 0,
                    }
                }))
            };

            if nzp == "0000" {
                return Err(LexError::InvalidArgument("Branch instruction with no NZP flags - will never branch. Did you mean to branch in all cases? (BRnzp)".into()));
            }
            
            //// end of nzp shenanigans
                        

            // Get the operand (the label or flags part)
            let req_label = get_operand_from_ind(1);
            
            if let Some(jump_addr) = label_addresses
                .iter()
                .filter_map(|(label, line_num)| req_label.find(label).map(|_| *line_num))
                .collect::<Vec<u16>>().first()
            {
                let code = op.as_opcode().to_owned()
                    + &nzp          // Add the "nzp" flags part to the opcode
                    + format!("{:08b}", *jump_addr as u8).as_str();
                Ok(code)
            } else {
                Err(LexError::InvalidArgument("Bad Immediate".into()))
            }
        }
        

        CMP => {
            // CMP Rs, Rt
            let Rs = Register::from_str(get_operand_from_ind(1))?;
            let Rt = Register::from_str(get_operand_from_ind(2))?;
            let code = "00100000".to_owned() + Rs.bits() + Rt.bits();
            Ok(code)
        }

        ADD | SUB | MUL | DIV => {
            // ADD Rd, Rs, Rt
            let Rd = Register::from_str(get_operand_from_ind(1))?;
            let Rs = Register::from_str(get_operand_from_ind(2))?;
            let Rt = Register::from_str(get_operand_from_ind(3))?;
            let code = op.as_opcode().to_owned() + Rd.bits() + Rs.bits() + Rt.bits();
            Ok(code)
        }

        Operation::LDR => {
            let Rd = Register::from_str(get_operand_from_ind(1))?;
            let Rs = Register::from_str(get_operand_from_ind(2))?;
            let code = op.as_opcode().to_owned() + Rd.bits() + Rs.bits() + "0000";
            Ok(code)
        }

        Operation::STR => {
            let Rs = Register::from_str(get_operand_from_ind(1))?;
            let Rt = Register::from_str(get_operand_from_ind(2))?;
            let code = op.as_opcode().to_owned() + "0000" + Rs.bits() + Rt.bits();
            Ok(code)
        }
        Operation::CONST => {
            let Rd = Register::from_str(get_operand_from_ind(1))?;
            let imm8 = get_operand_from_ind(2);
            let imm8 = imm8.replace("#", "");

            if imm8.parse::<u8>().is_ok() {
                let code = op.as_opcode().to_owned()
                    + Rd.bits()
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
                    .tokens.first()
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
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

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

    let gexed_lines: Vec<Result<MachineLine, LexError>> = lexed_lines
        .into_iter()
        .map(|line| operation_conv(line, label_addresses.clone()))
        .collect();

    dbg!(&gexed_lines);

    //// Do Printing
    ///
    println!("# Assembled output for {file_path}");

    // Column widths
    let width1 = 19; // +1 for the comma +2 for 0b
    let width2 = 30;
    let width3 = 0; // no padding on final line

    let mut memories: Vec<MachineLine> = vec![];

    for line in gexed_lines {
        match line {
            Ok(magic_sparkles) => {
                match magic_sparkles.line_type {
                    LineType::Memory => {
                        memories.push(magic_sparkles);
                        // dbg!(&magic_sparkles);
                        //tokens seperated by a comma + space, seperated from the comment with a #
                    }
                    LineType::Human => {
                        // don't print blank lines for now, please and thank you
                    }
                    // LineType::Bad => {}, //todo: this should probably be handled similarly to the Err statement at the end (maybe seperate error types? )
                    // LineType::Label=> {},
                    _ => {
                        let fmt_bin = match magic_sparkles.bin {
                            Some(bin) => "0b".to_owned() + &bin + ",",
                            None => "".to_owned(),
                        };

                        let comment = match magic_sparkles.comment {
                            Some(comment) => "; ".to_owned() + &comment,
                            None => "".to_owned(),
                        };

                        println!(
                            "{:<width1$} # {:<width2$} {:<width3$}",
                            fmt_bin,
                            magic_sparkles.parsed_line.tokens.join(" "),
                            comment
                        );
                    }
                }
            }
            Err(scary_monsters) => {
                dbg!(scary_monsters);
            }
        }
    }

    //seperate memory lines by data location
    let mut data_lines = vec![];
    let mut target_threads = vec![];

    for mem in memories {
        match mem.parsed_line.tokens.first().unwrap().as_str() {
            ".data" => data_lines.push(mem),
            ".threads" => target_threads.push(mem),
            _ => todo!(),
        }
    }

    println!();

    println!("# .data");
    let end = data_lines.len() - 1;
    for (ind, data) in data_lines.into_iter().enumerate() {
        // let name = mem.parsed_line.tokens.get(0);
        let a: Vec<String> = (data.parsed_line.tokens.clone())
            .split_off(1)
            .into_iter()
            .map(|f| f.to_owned() + ", ")
            .collect();
        let mut b = a.concat();

        if ind == end {
            b = b.strip_suffix(", ").unwrap_or("").to_string();
        }

        // println!("{}", name.unwrap_or(&"".to_string()));
        let w = 24;
        println!("{:<w$} # {}", b, data.comment.unwrap_or("".to_string()))
    }

    println!();

    println!(
        "remember to specify thread count ({}) in the testbench!",
        target_threads.first()
            .unwrap()
            .parsed_line
            .tokens
            .get(1)
            .unwrap()
    );
}
