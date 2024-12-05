use std::collections::HashMap;
use crate::{err::ErrT, lex::{TokType, Tokens}, p::{parse, VVal, NST}};

#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    pub left: String,
    pub operator: String,
    pub right: String,
    pub left_type: ValueType,
    pub right_type: ValueType,
    pub c_operator: String,
    pub c_code: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValueType {
    Int,
    Float,
    Str,
    Bool,
}

pub fn p2(
    toks: &Tokens,
    tok_iter: &mut std::iter::Peekable<std::slice::Iter<'_, Tokens>>,
    codes: &[&str],
    errors: &mut Vec<ErrT>,
    nst: &mut Vec<NST>,
    ln: &mut usize,
    vars: &HashMap<String, VVal>,
) {
    match (toks.get_type(), toks.get_value()) {
        (TokType::CMD, "if") => {
            //println!("[DEBUG] inside if");
            let mut if_body: Vec<Tokens> = Vec::new();
            let mut if_condition = String::new();

            while let Some(tok) = tok_iter.next() {
                if tok.get_type() == TokType::OP && tok.get_value() == "(" {
                    let mut paren_balance = 1;
                    while let Some(inner_tok) = tok_iter.next() {
                        match (inner_tok.get_type(), inner_tok.get_value()) {
                            (TokType::OP, "(") => paren_balance += 1,
                            (TokType::OP, ")") => paren_balance -= 1,
                            _ => if_condition.push_str(inner_tok.get_value()),
                        }
                        if paren_balance == 0 {
                            break;
                        }
                    }
                    if paren_balance != 0 {
                        errors.push(ErrT::UnmatchedParen(*ln, codes[*ln].to_string()));
                        return;
                    }
                }

                if tok.get_type() == TokType::OP && tok.get_value() == "{" {
                    let mut brace_balance = 1;
                    while let Some(inner_tok) = tok_iter.next() {
                        if inner_tok.get_type() == TokType::OP {
                            if inner_tok.get_value() == "{" {
                                brace_balance += 1;
                            } else if inner_tok.get_value() == "}" {
                                brace_balance -= 1;
                            }
                        }
                        if inner_tok.get_type() == TokType::EOL {
                            *ln += 1;
                        }
                        if brace_balance == 0 {
                            break;
                        }
                        if_body.push(inner_tok.clone());
                    }
                    if brace_balance != 0 {
                        errors.push(ErrT::InVCond(*ln, "Unmatched braces in if body".to_string()));
                        return;
                    }
                }
            }
            //println!("[DEBUG] if condition : {}", if_condition);


            if if_condition.is_empty() {
                errors.push(ErrT::EmptyCond(*ln, codes[*ln].to_string()));
                return;
            }
            if let Some(parsed_condition) = parse_condition(&if_condition, *ln, errors, vars,&nst) {
                //println!("[DEBUG] parsed condition : {:?}", parsed_condition);
               // println!("[DEBUG] if body : {:?}", if_body);
                let parsed_body = parse(&if_body, codes, "", false, errors);
                //println!("[DEBUG] parsed body : {:?}", parsed_body);
                nst.push(NST::NIF(parsed_condition, parsed_body));
            }
        }
        (TokType::CMD, v) if (v != "print" && v != "println" && v != "cls" && v != "wait" && v != "may" && v != "cmd") => {
            let mut isvrd = false;
            let mut collected_value = String::new();

            while let Some(tok) = tok_iter.next() {
                if tok.get_type() == TokType::EOL {
                    *ln += 1;
                    if isvrd {
                        let var_name = v;
                        let var_value = collected_value.trim().to_string();

                        if vars.contains_key(var_name) {
                            let mut vv = if var_value.starts_with('"') && var_value.ends_with('"') {
                                VVal::Str(var_value.clone())
                            } else if var_value.parse::<i32>().is_ok() {
                                VVal::Int(var_value.parse::<i32>().unwrap())
                            } else if var_value.parse::<f32>().is_ok() {
                                VVal::F(var_value.parse::<f32>().unwrap())
                            } else if vars.get(&var_value).is_some() {
                                VVal::VarRef(var_name.to_string(), var_value.clone())
                            } else {
                                errors.push(ErrT::InValidVarVal(*ln, var_value.clone()));
                                VVal::Str(var_value.clone())
                            };
                        
                            let mut found = false;
                            for i in &mut *nst {
                                if let NST::Input(n) = i {
                                    if n == var_name {
                                        vv = VVal::Str(var_value.clone());
                                        found = true;
                                        break;
                                    }
                                }
                            }
                            if !found {
                                errors.push(ErrT::InValidVarVal(*ln, var_value.clone()));

                            }
                        
                            nst.push(NST::VarRD(var_name.to_string(), vv));
                        } else {
                            errors.push(ErrT::InValidVarVal(*ln, var_value.clone()));
                            break;
                        }
                        
                    }
                } else if tok.get_type() == TokType::OP && tok.get_value() == "=" {
                    isvrd = true;
                } else if isvrd {
                    collected_value.push_str(tok.get_value());
                }
            }
        }
        _ => {}
    }
}
#[allow(unused)]
fn p2_block(
    toks: &[Tokens],
    codes: &[&str],
    errors: &mut Vec<ErrT>,
    ln: &mut usize,
    vars: &HashMap<String, VVal>,
) -> Vec<NST> {
    let mut nested_nst = Vec::new();
    let mut tok_iter = toks.iter().peekable();
    while let Some(tok) = tok_iter.next() {
        p2(tok, &mut tok_iter, codes, errors, &mut nested_nst, ln, vars);
    }
    nested_nst
}
#[allow(unused)]
pub fn parse_condition(
    condition: &str,
    line_number: usize,
    errors: &mut Vec<ErrT>,
    vars: &HashMap<String, VVal>,
    nst : &Vec<NST>
) -> Option<Condition> {
    let mut index = 0;
    let mut operand_stack = Vec::new();
    let mut operator_stack: Vec<String> = Vec::new();
    let condition = condition.replace(" ","");
    let condition = condition.as_str();
    while index < condition.len() {
        let c = condition.chars().nth(index).unwrap();
        //println!("[DEBUG] c ~ {}",c);
        //println!("[DEBUG] ops ~ {:?} | oprs ~ {:?}",operand_stack,operator_stack);

        match c {
            ' ' => index += 1, // Skip whitespace
            '(' => {
                index += 1;
                if let Some(nested_condition) = parse_condition(&condition[index..], line_number, errors, vars,nst) {
                    operand_stack.push(format!("({})", nested_condition.c_code));
                }
            }
            ')' => {
                index += 1;
                break;
            }
            '0'..='9' | '"' | '\'' | 'a'..='z' | 'A'..='Z' | '_' => {
                if let Some(operand) = parse_operand_char_by_char(condition, &mut index, line_number, errors, vars,nst) {
                    operand_stack.push(operand);
                }
            }
            '&' | '|' | '=' | '!' | '<' | '>' | '+' | '-' | '*' | '/' => {
                if let Some(operator) = parse_operator_char_by_char(condition, &mut index, line_number, errors) {
                    while let Some(top_operator) = operator_stack.last() {
                        if has_higher_precedence(top_operator, &operator) {
                            apply_operator(&mut operand_stack, operator_stack.pop().unwrap(), line_number, errors)?;
                        } else {
                            break;
                        }
                    }
                    operator_stack.push(operator);
                }
            }
            _ => {
                errors.push(ErrT::InvalidCondOp(line_number, format!("Unexpected character: `{}`", c)));
                return None;
            }
        }
    }

    while let Some(operator) = operator_stack.pop() {
        apply_operator(&mut operand_stack, operator, line_number, errors)?;
    }

    if operand_stack.len() == 1 {
        let c_code = operand_stack.pop().unwrap();
        return Some(Condition {
            left: String::new(),
            operator: String::new(),
            right: String::new(),
            left_type: ValueType::Bool,
            right_type: ValueType::Bool,
            c_operator: String::new(),
            c_code,
        });
    }

    errors.push(ErrT::InvalidCondOp(line_number, "Invalid condition structure.".to_string()));
    None
}

fn parse_operand_char_by_char(
    condition: &str,
    index: &mut usize,
    line_number: usize,
    errors: &mut Vec<ErrT>,
    vars: &HashMap<String, VVal>,
    nst : &Vec<NST>
) -> Option<String> {
    let mut buffer = String::new();
    while *index < condition.len() {
        let c = condition.chars().nth(*index).unwrap();
        match c {
            ' ' | '(' | ')' | '&' | '|' | '=' | '!' | '<' | '>' | '+' | '-' | '*' | '/' => break,
            _ => {
                buffer.push(c);
                *index += 1;
            }
        }
    }

    let value_type = determine_value_type(&buffer, vars, errors, line_number,nst)?;
    match value_type {
        ValueType::Str => Some(buffer),
        ValueType::Int | ValueType::Float => Some(buffer),
        _ => {
            errors.push(ErrT::InVCond(line_number, format!("Invalid operand: {}", buffer)));
            None
        }
    }
}

fn parse_operator_char_by_char(
    condition: &str,
    index: &mut usize,
    line_number: usize,
    errors: &mut Vec<ErrT>,
) -> Option<String> {
    let mut operator = String::new();
    while *index < condition.len() {
        let c = condition.chars().nth(*index).unwrap();
        match c {
            '=' | '!' | '<' | '>' | '&' | '|' => {
                operator.push(c);
                *index += 1;
            }
            _ => break,
        }
    }

    if ["==", "!=", "<", ">", "<=", ">=", "&&", "||"].contains(&operator.as_str()) {
        Some(operator)
    } else {
        errors.push(ErrT::InvalidCondOp(line_number, format!("Invalid operator: `{}`", operator)));
        None
    }
}

fn has_higher_precedence(op1: &str, op2: &str) -> bool {
    let precedence = |op: &str| match op {
        "&&" | "||" => 1,
        "<" | ">" | "<=" | ">=" => 2,
        "==" | "!=" => 3,
        "+" | "-" => 4,
        "*" | "/" => 5,
        _ => 0,
    };

    precedence(op1) > precedence(op2)
}

fn apply_operator(
    operand_stack: &mut Vec<String>,
    operator: String,
    line_number: usize,
    errors: &mut Vec<ErrT>,
) -> Option<()> {
    if operand_stack.len() < 2 {
        errors.push(ErrT::InvalidCondOp(line_number, "Not enough operands for operator.".to_string()));
        return None;
    }

    let right = operand_stack.pop().unwrap();
    let left = operand_stack.pop().unwrap();

    // Handle string comparison logic
    let combined = if operator == "==" || operator == "!=" {
        // Check if either operand is a string variable from NST::Input
        if left.starts_with('"') || left.starts_with('\'') {
            // String literal comparison
            format!(
                "strcmp({}, {}) {} 0",
                left, right, if operator == "==" { "==" } else { "!=" }
            )
        } else {
            // String variable (e.g., NST::Input) comparison
            format!(
                "strcmp({}, {}) {} 0",
                left, right, if operator == "==" { "==" } else { "!=" }
            )
        }
    } else {
        format!("{} {} {}", left, operator, right)
    };

    operand_stack.push(combined);
    Some(())
}

fn determine_value_type(
    expr: &str,
    vars: &HashMap<String, VVal>,
    errors: &mut Vec<ErrT>,
    line_number: usize,
    nst : &Vec<NST>
) -> Option<ValueType> {
    if expr.starts_with('"') || expr.starts_with('\'') {
        return Some(ValueType::Str);
    }

    match vars.get(expr) {
        Some(VVal::Int(_)) => Some(ValueType::Int),
        Some(VVal::F(_)) => Some(ValueType::Float),
        Some(VVal::Str(_)) => Some(ValueType::Str),
        _ => {
            if expr.parse::<i32>().is_ok() {
                Some(ValueType::Int)
            } else if expr.parse::<f32>().is_ok() {
                Some(ValueType::Float)
            } else {
                for i in nst{
                    match i{
                        NST::Input(n) => {
                            if n == expr{
                                return Some(ValueType::Str);
                            }
                        } 
                        _ => {}
                    }
                }
                errors.push(ErrT::VNF(line_number, expr.to_string()));
                None
            }
        }
    }
}