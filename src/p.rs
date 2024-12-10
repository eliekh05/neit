use crate::{
    err::{generr, ErrT},
    lex::{TokType, Tokens},
    p2::{p2, Condition}, p3::p3,
};
use colored::Colorize;
use std::{collections::HashMap, process::exit};

#[derive(Debug, PartialEq)]
pub enum NST {
    PRINT(String),
    Var(Var),
    Input(String),
    Func(String, Vec<String>, Vec<NST>),
    NCLRSCRN,
    WAIT(u64),
    NIF(Condition, Vec<NST>),
    VarRD(String, VVal),
    NWHILE(Condition, Vec<NST>),
}

#[derive(Debug, PartialEq)]
pub struct Var {
    pub name: String,
    pub value: VVal,
}

#[derive(PartialEq, Clone, Debug)]
pub enum VVal {
    Str(String),
    Int(i32),
    F(f32),
    VarRef(String, String),
}

pub fn parse(
    toks: &[Tokens],
    codes: &[&str],
    file: &str,
    errext: bool,
    errors: &mut Vec<ErrT>,
) -> Vec<NST> {
    let mut vars: HashMap<String, VVal> = HashMap::new();
    let mut nst: Vec<NST> = Vec::new();
    let mut ln: usize = 1;
    let mut tok_iter = toks.iter().peekable();

    while let Some(tok) = tok_iter.next() {
        if let TokType::EOL = tok.get_type() {
            ln += 1;
        }

        match (tok.get_type(), tok.get_value()) {
            (TokType::CMD, "print") | (TokType::CMD, "println") => {
                let mut tp = String::new();
                let mut first_space_skipped = false;

                for pptok in tok_iter.by_ref() {
                    if pptok.get_type() != TokType::EOL {

                        if pptok.get_value() == " " && !first_space_skipped {
                            first_space_skipped = true;
                            continue;
                        }
                        tp.push_str(pptok.get_value());
                    } else {
                        ln += 1;
                        break;
                    }
                }

                let mut vm = false;
                let mut cvnm = String::new();
                let mut cpti = tp.chars().peekable();

                while let Some(c) = cpti.next() {
                    match (c, cpti.peek()) {
                        ('\\', Some('{')) | ('\\', Some('}')) => {
                            cpti.next();
                        }
                        ('{', _) if !vm => {
                            vm = true;
                        }
                        ('}', _) if vm => {
                            vm = false;
                            if vars.get(&cvnm).is_none()
                                && !nst.iter().any(|x| matches!(x, NST::Input(n) if *n == cvnm))
                            {
                                errors.push(ErrT::VNF(ln, cvnm.clone()));
                            }
                            cvnm.clear();
                        }
                        (c, _) if vm => {
                            cvnm.push(c);
                        }
                        _ => {}
                    }
                }

                if tok.get_type() == TokType::CMD && tok.get_value() == "println" {
                    tp.push_str("\\n");
                }
                nst.push(NST::PRINT(tp));
            }
            (TokType::CMD, "cls") => {
                nst.push(NST::NCLRSCRN);
                ln += 1;
            }
            (TokType::CMD, "may") => {
                let mut var_name = String::new();
                let mut var_value = String::new();
                let mut state = 0;
                let mut round = 0;
                let mut eqfound = false;

                for vtok in tok_iter.by_ref() {
                    if vtok.get_type() != TokType::SPACE {
                        match state {
                            0 if vtok.get_type() == TokType::INSTR => {
                                var_name = vtok.get_value().to_string();
                                state = 1;
                            }
                            1 if vtok.get_type() == TokType::OP
                                && vtok.get_value() == "="
                                && round == 0 =>
                            {
                                state = 2;
                                eqfound = true;
                                round += 1;
                            }
                            _ if round == 0 && !eqfound => {
                                errors.push(ErrT::EqNF(ln, var_name.clone()));
                                break;
                            }
                            2 if vtok.get_type() != TokType::EOL => {
                                var_value.push_str(vtok.get_value());
                            }
                            2 => {
                                let vval = parse_var_value(&var_value, ln, &mut vars, errors, &nst);

                                if vval != VVal::Str("__TAKEININPUT__".to_string()) {
                                    vars.insert(var_name.clone(), vval.clone());
                                    nst.push(NST::Var(Var {
                                        name: var_name.clone(),
                                        value: vval,
                                    }));
                                } else {
                                    nst.push(NST::Input(var_name.clone()));
                                }
                                break;
                            }
                            _ => {}
                        }
                    }
                }
                ln += 1;
            }
            (TokType::CMD, "wait") => {
                fn convert_to_ms(time_str: &str, errors: &mut Vec<ErrT>, line: usize) -> u64 {
                    let ogt = &time_str;
                    let time_str = time_str.to_ascii_lowercase();
                    let time_str = time_str.trim();
                    if time_str.ends_with("ms") {
                        let (num_str, _) = time_str.split_at(time_str.len() - 2);
                        let num: u64 = num_str.parse().unwrap_or(0);
                        num
                    } else if time_str.ends_with("s") {
                        let (num_str, _) = time_str.split_at(time_str.len() - 1);
                        let num: u64 = num_str.parse().unwrap_or(0);
                        num * 1000
                    } else if time_str.ends_with("m") {
                        let (num_str, _) = time_str.split_at(time_str.len() - 1);
                        let num: u64 = num_str.parse().unwrap_or(0);
                        num * 60 * 1000
                    } else if time_str.ends_with("hr") {
                        let (num_str, _) = time_str.split_at(time_str.len() - 2);
                        let num: u64 = num_str.parse().unwrap_or(0);
                        num * 60 * 60 * 1000
                    } else {
                        errors.push(ErrT::InVTimeVal(line, ogt.to_string()));
                        0
                    }
                }

                let mut a = String::new();
                for tok in tok_iter.by_ref() {
                    if tok.get_type() == TokType::EOL {
                        ln += 1;
                        //println!("[DEBUG] og wait time :~ {}", a);
                        let time_in_ms = convert_to_ms(&a, errors, ln);
                        //println!("[DEBUG] wait for :~ {} => {} ms", a, time_in_ms);
                        nst.push(NST::WAIT(time_in_ms));
                    } else {
                        a.push_str(tok.get_value());
                    }
                }
            }
            (TokType::CMD, "cmd") => {
                let mut bc = 0;
                let mut name = String::new();
                let mut args = Vec::new();
                let mut body = Vec::new();

                for ctok in tok_iter.by_ref() {
                    println!("bc : {} | tok : {:?}", bc, ctok);
                    if ctok.get_type() == TokType::EOL {
                        ln += 1;
                        continue;
                    }
                    if bc == 0 {
                        if ctok.get_type() == TokType::INSTR {
                            if name.is_empty() {
                                name = ctok.get_value().to_string();
                            } else {
                                args.push(ctok.get_value().to_string());
                            }
                        }
                        if ctok.get_type() == TokType::OP && ctok.get_value() == "{" {
                            bc += 1;
                        }
                    } else {
                        if ctok.get_type() == TokType::OP {
                            if ctok.get_value() == "{" {
                                bc += 1;
                            } else if ctok.get_value() == "}" {
                                bc -= 1;
                                if bc == 0 {
                                    break;
                                }
                            }
                        }
                        body.push(ctok.clone());
                    }
                }
                let func_body = parse(&body, codes, file, false, errors);
                nst.push(NST::Func(name, args, func_body));
            }
            _ => {
                if !p2(
                    tok,
                    &mut tok_iter,
                    codes,
                    errors,
                    &mut nst,
                    &mut ln,
                    &vars,
                    file,
                ) {
                    p3(tok, &mut tok_iter, codes, errors, &mut nst, &mut ln, &vars, file);
                }
            }
        }
    }

    if !errors.is_empty() {
        eprintln!(
            "{}{}\n{}",
            "Errors detected in file: ".bold().red(),
            file.yellow().bold(),
            "+".repeat(100).red().dimmed()
        );

        for err in errors {
            generr(err.clone(), &codes.to_vec());
            eprintln!("{}", "─".repeat(100).red().dimmed());
        }
        if errext {
            exit(-1);
        }
    }

    nst
}

fn parse_var_value(
    var_value: &str,
    ln: usize,
    vars: &mut HashMap<String, VVal>,
    errors: &mut Vec<ErrT>,
    nst: &Vec<NST>,
) -> VVal {
    let trimmed = var_value.trim();

    // Check for complex expressions (e.g., "op + 1", "1 + 1")
    if let Some((left, operator, right)) = parse_expression(trimmed) {
        // Parse left and right operands
        let left_val = parse_var_value(left, ln, vars, errors, nst);
        let right_val = parse_var_value(right, ln, vars, errors, nst);

        // Perform type checking
        match (&left_val, &right_val) {
            (VVal::Int(_), VVal::Int(_)) | (VVal::F(_), VVal::F(_)) => {
                // Both operands are numeric, valid operation
                return match operator {
                    "+" | "-" | "*" | "/" => combine_numeric_operands(left_val, right_val, operator),
                    _ => {
                        errors.push(ErrT::InvalidCondOp(ln, operator.to_string()));
                        VVal::Str("ERR_INVALID_OPERATOR".to_string())
                    }
                };
            }
            (VVal::Str(_), VVal::Str(_)) if operator == "+" => {
                // String concatenation (only "+" is allowed for strings)
                if let (VVal::Str(l), VVal::Str(r)) = (left_val, right_val) {
                    VVal::Str(format!("{}{}", l, r));
                } else {
                    unreachable!()
                }
            }
            _ => {
                // Type mismatch error
                errors.push(ErrT::InvalidOperand(
                    ln,
                    format!(
                        "Cannot apply operator `{}` between `{}` and `{}`",
                        operator, left_val.type_as_str(), right_val.type_as_str()
                    ),
                ));
                VVal::Str("ERR_TYPE_MISMATCH".to_string());
            }
        }
    }

    // Handle simple values (not expressions)
    if (trimmed.starts_with('"') && trimmed.ends_with('"'))
        || (trimmed.starts_with("'") && trimmed.ends_with("'"))
    {
        let value = trimmed
            .trim_start_matches('"')
            .trim_start_matches("'")
            .trim_end_matches('"')
            .trim_end_matches("'");
        VVal::Str(value.to_string())
    } else if let Ok(val) = trimmed.parse::<i32>() {
        VVal::Int(val)
    } else if let Ok(val) = trimmed.parse::<f32>() {
        VVal::F(val)
    } else if let Some(v) = vars.get(trimmed) {
        VVal::VarRef(
            trimmed.to_string(),
            match v {
                VVal::Int(_) => "i".to_string(),
                VVal::F(_) => "f".to_string(),
                VVal::Str(_) => "s".to_string(),
                VVal::VarRef(_, t) => t.to_string(),
            },
        )
    } else if nst
        .iter()
        .any(|x| matches!(x, NST::Input(n) if *n == trimmed))
    {
        VVal::VarRef("".to_string(), "s".to_string())
    } else if trimmed == "takein()" {
        VVal::Str("__TAKEININPUT__".to_string())
    } else {
        errors.push(ErrT::InValidVarVal(ln, trimmed.to_string()));
        VVal::Str("ERR_VAR_NOT_FOUND___!".to_string())
    }
}
fn parse_expression(input: &str) -> Option<(&str, &str, &str)> {
    let operators = ["+", "-", "*", "/"];
    for operator in operators {
        if let Some(pos) = input.find(operator) {
            let left = &input[..pos].trim();
            let right = &input[pos + operator.len()..].trim();
            return Some((left, operator, right));
        }
    }
    None
}
fn combine_numeric_operands(left: VVal, right: VVal, operator: &str) -> VVal {
    match (left, right) {
        (VVal::Int(l), VVal::Int(r)) => match operator {
            "+" => VVal::Int(l + r),
            "-" => VVal::Int(l - r),
            "*" => VVal::Int(l * r),
            "/" => VVal::Int(l / r),
            _ => unreachable!(),
        },
        (VVal::F(l), VVal::F(r)) => match operator {
            "+" => VVal::F(l + r),
            "-" => VVal::F(l - r),
            "*" => VVal::F(l * r),
            "/" => VVal::F(l / r),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}
impl VVal {
    fn type_as_str(&self) -> &str {
        match self {
            VVal::Int(_) => "int",
            VVal::F(_) => "float",
            VVal::Str(_) => "string",
            VVal::VarRef(_, t) => t.as_str(),
        }
    }
}
