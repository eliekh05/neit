use crate::{
    err::ErrT,
    lex::{TokType, Tokens},
    p::{parse, VVal, NST},
    p2::parse_condition,
};
use std::collections::HashMap;

#[allow(unused)]
pub fn p3(
    tok: &Tokens,
    tokiter: &mut std::iter::Peekable<std::slice::Iter<'_, Tokens>>,
    codes: &[&str],
    errors: &mut Vec<ErrT>,
    nst: &mut Vec<NST>,
    ln: &mut usize,
    vars: &HashMap<String, VVal>,
    file: &str,
) {
    match (tok.get_type(), tok.get_value()) {
        (TokType::CMD, "while") => {
            let mut cond = String::new();
            let mut body_tokens = Vec::new();
            let mut in_parentheses = false;
            let mut brace_count = 0;

            // Parse condition inside parentheses
            while let Some(tok) = tokiter.next() {
                match (tok.get_type(), tok.get_value()) {
                    (TokType::OP, "(") if !in_parentheses => {
                        in_parentheses = true;
                    }
                    (TokType::EOL,_) => {
                        *ln += 1;
                    }
                    (TokType::OP, ")") if in_parentheses => {
                        in_parentheses = false;
                        break; // Exit condition parsing
                    }
                    (TokType::SPACE, _) => {
                        continue; // Ignore spaces inside condition
                    }
                    (_, _) if in_parentheses => {
                        cond.push_str(tok.get_value());
                    }
                    _ => {
                        errors.push(ErrT::UnmatchedParen(*ln, codes[*ln].to_string()));
                        return;
                    }
                }
            }

            // Check for unmatched or empty condition
            if in_parentheses {
                errors.push(ErrT::UnmatchedParen(*ln, codes[*ln].to_string()));
                return;
            }
            if cond.is_empty() {
                errors.push(ErrT::EmptyCond(*ln, codes[*ln].to_string()));
                return;
            }

            // Parse the condition
            let cond_parsed = parse_condition(&cond, *ln, errors, vars, nst);
            let mut condition = match cond_parsed {
                Some(cond) => cond,
                None => {
                    errors.push(ErrT::InVCond(*ln, cond.clone()));
                    return;
                }
            };

            // Parse body inside braces with brace counting
            while let Some(tok) = tokiter.next() {
                match (tok.get_type(), tok.get_value()) {
                    (TokType::OP, "{") => {
                        brace_count += 1; // Increment brace count
                        if brace_count == 1 {
                            continue; // Skip the first `{` to start body parsing
                        }
                    }
                    (TokType::OP, "}") => {
                        brace_count -= 1; // Decrement brace count
                        if brace_count == 0 {
                            break; // Exit body parsing
                        }
                    }
                    (_, _) if brace_count > 0 => {
                        body_tokens.push(tok.clone());
                    }
                    _ => {
                        errors.push(ErrT::UnmatchedParen(*ln, codes[*ln].to_string()));
                        return;
                    }
                }
            }

            // Check for unmatched braces
            if brace_count != 0 {
                errors.push(ErrT::UnmatchedParen(*ln, "Unmatched braces in while loop".to_string()));
                return;
            }

            // Check for empty body
            if body_tokens.is_empty() {
                errors.push(ErrT::InVCond(*ln, "Empty body for while loop".to_string()));
                return;
            }

            // Parse the body tokens
            let body = parse(&body_tokens, codes, file, false, errors);
            nst.push(NST::NWHILE(condition, body));
        }
        _ => {}
    }
}
