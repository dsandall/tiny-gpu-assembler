use std::cmp::Ordering;
use std::env;
use std::fs;
use std::ops::Index;

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

    if let None = lexed_line.as_any_mut().downcast_mut::<OperationLine>() {
        return {
            Ok(MachineLine {
                bin: None,
                comment: lexed_line.parsed().comment.clone(),
                line_num: lexed_line.parsed().line_num,
                parsed_line: lexed_line.parsed().clone(),
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
    let op: Operation = Operation::from_str(operation_line.parsed.tokens.get(0).unwrap()).unwrap();
    let parsed_line = operation_line.parsed.clone();

    if op.num_args() != parsed_line.tokens.len() as u8 - 1 {
        return Err(LexError::InvalidOperation("".to_string()));
    }

    let bin = match op {
        NOP => Ok(op.as_opcode().to_owned() + "000000000000"),

        BRnzp => {
            // Immediates
            let label = parsed_line.tokens.get(1);

            if let Some(req_label) = label {
                let &jump_addr= label_addresses
                    .iter()
                    .filter_map(|(label, line_num)| match req_label.find(label) {
                        Some(_) => Some(line_num.clone()),
                        None => None,
                    })
                    .collect::<Vec<u16>>().get(0).unwrap();


                if true {
                    let code = op.as_opcode().to_owned()
                        + "1000" // the spec lists this as nzp0, but it appears that it MUST be 1000
                        + format!("{:08b}", jump_addr as u8).as_str();
                    Ok(code)
                } else {
                    Err(LexError::InvalidArgument("Bad Immediate".into()))
                }
            } else {
                Err(LexError::InvalidArgument("Bad Immediate".into()))
            }
        }
        

        CMP => {
            // CMP Rs, Rt
            let Rs = Register::from_str(parsed_line.tokens.get(1).expect("should have 2 args"))?;
            let Rt = Register::from_str(parsed_line.tokens.get(2).expect("should have 2 args"))?;
            let code = "00100000".to_owned() + Rs.bits() + Rt.bits();
            Ok(code)
        }

        ADD | SUB | MUL | DIV => {
            // ADD Rd, Rs, Rt
            let Rd = Register::from_str(parsed_line.tokens.get(1).expect("should have 2 args"))?;
            let Rs = Register::from_str(parsed_line.tokens.get(2).expect("should have 2 args"))?;
            let Rt = Register::from_str(parsed_line.tokens.get(3).expect("should have 2 args"))?;
            let code = op.as_opcode().to_owned() + Rd.bits() + Rs.bits() + Rt.bits();
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
            let imm8 = parsed_line.tokens.get(2).expect("should have 2 args");
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
            bin: bin.ok(),
            comment: parsed_line.comment.clone(),
            line_num: parsed_line.line_num,
            parsed_line: parsed_line,
        })
    } else {
        Err(bin.err().unwrap())
    }
}

// fn gex<'a>(lexed_line: Box<dyn LexedLine>, label_numbers: &Vec<(String, u32)>, lexed_lines: & Vec<Box<dyn LexedLine>>) -> Result<MachineLine<'a>, LexError> {

//     // let mapped = Operation::from_str(inner);

//     // if mapped.is_ok() {
//     //     more_lex(mapped.unwrap(), lexed_line)
//     // } else {
//     //     // if the first token is invalid
//     //     println!("{}", inner);
//     //     println!("Code cannot be mapped!");
//     //     Err(LexError::InvalidOperation(inner.to_string()))
//     // }

// }

fn extract_label_assoc_lines(lexed_lines: &Vec<Box<dyn LexedLine>>) -> Vec<(String, u32)> {
    // handle Memory and labels (labels must be done prior to operations)
    let mut labels_lines: Vec<(String, u32)> = vec![]; //maps a Label String to the next line number which is a valid operation

    for line in lexed_lines {
        if let Some(_memory_line) = line.as_any().downcast_ref::<MemoryLine>() {
            // handle all memory lines
            // we don't actually need to do anything here until printing occurs
        } else if let Some(label_line) = line.as_any().downcast_ref::<LabelLine>() {
            // handle all label lines
            if label_line.parsed.tokens.len() != 1 {
                todo!()
            } else {
                let label = label_line
                    .parsed
                    .tokens
                    .get(0)
                    .unwrap()
                    .clone()
                    .replace(":", "");
                let num = label_line.parsed.line_num;

                //// go through the parsed lines and find the next operation line
                let line_number: u32; //a place to store the line number we find in the following loop
                let mut i = num.clone(); //loop variable

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
    dbg!(&parsed_lines);

    let mut lexed_lines: Vec<Box<dyn LexedLine>> = parsed_lines
        .into_iter()
        .map(|item| identify_line(item))
        .collect();

    dbg!(&lexed_lines);

    let mut label_lines = extract_label_assoc_lines(&lexed_lines);

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

    dbg!(&label_addresses);

    let gexed_lines: Vec<Result<MachineLine, LexError>> = lexed_lines
        .into_iter()
        .map(|line| operation_conv(line, label_addresses.clone()))
        .collect();

    dbg!(&gexed_lines);

    //// Do Printing

    // Column widths
    let width1 = 19; // +1 for the comma +2 for 0b
    let width2 = 30;
    let width3 = 0; // no padding on final line
    for line in gexed_lines {
        match line {
            Ok(magic_sparkles) => {
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

            Err(scary_monsters) => {
                dbg!(scary_monsters);
            }
        }
    }

    println!("In file {file_path}");
}
