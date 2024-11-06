use super::types::{Tokens, Vars};

// Required types
#[derive(Debug, Clone)]
pub enum CondT {
    STR(String),
    INT(i32),
    F(f64),
}

// Function to check if a given string is a valid C-style condition
pub fn ccc(cond: &str, vars: &[Tokens]) -> Result<(), String> {
    let mut curwrd = String::new();
    let mut current_type: Option<CondT> = None;
    let binding = cond.replace(" ", ""); // Remove whitespace for easier parsing

    let mut chars = binding.chars().enumerate().peekable();
    let _operators = ["!=", "==", ">=", "<=", ">", "<"];
    let mut in_strcmp = false; // Flag for string comparison

    while let Some((i, c)) = chars.next() {
        let nxtc = chars.peek().map(|(_, next_c)| *next_c);

        match c {
            '!' | '=' | '>' | '<' => {
                if !curwrd.is_empty() {
                    if let Some(var_type) = check_word_type(&curwrd, vars) {
                        if let Some(ref cur_type) = current_type {
                            if !is_type_compatible(cur_type, &var_type) {
                                return Err(format!(
                                    "Type mismatch at index {}: expected '{:?}', found '{:?}'",
                                    i, cur_type, var_type
                                ));
                            }
                        } else {
                            current_type = Some(var_type.clone());
                        }

                        // If current_type is STR and we're in a strcmp, close strcmp call
                        if matches!(current_type, Some(CondT::STR(_))) && in_strcmp {
                            in_strcmp = false;
                        }
                    } else {
                        return Err(format!("Invalid word at index {}: '{}'", i, curwrd));
                    }
                }
                curwrd.clear();

                // Collect operator and handle accordingly
                if let Some(op) = nxtc {
                    let mut operator = c.to_string();
                    operator.push(op);
                    match operator.as_str() {
                        "!=" | "==" => {
                            chars.next(); // Move past the operator
                            if matches!(current_type, Some(CondT::STR(_))) {
                                // Handle strcmp for strings
                                in_strcmp = true;
                            }
                        }
                        ">=" | "<=" if !matches!(current_type, Some(CondT::STR(_))) => {
                            chars.next();
                        }
                        ">" | "<" if !matches!(current_type, Some(CondT::STR(_))) => {}
                        "=<" | "=>" | "=!" | "===" => {
                            return Err(format!(
                                "Invalid operator at index {}: '{}'. Did you mean '{}'?",
                                i,
                                operator,
                                correct_operator(&operator)
                            ));
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                curwrd.push(c);
            }
        }
    }

    if !curwrd.is_empty() {
        if let Some(var_type) = check_word_type(&curwrd, vars) {
            if let Some(ref cur_type) = current_type {
                if !is_type_compatible(cur_type, &var_type) {
                    return Err(format!(
                        "Final type mismatch: expected '{:?}', found '{:?}' for '{}'",
                        cur_type, var_type, curwrd
                    ));
                }
            }
        } else {
            return Err(format!("Invalid word in final check: '{}'", curwrd));
        }
    }

    Ok(())
}

// Checks the type of a word based on known variable types in `vars`
fn check_word_type(word: &str, vars: &[Tokens]) -> Option<CondT> {
    if word.starts_with('"') && word.ends_with('"') {
        return Some(CondT::STR(word.to_string()));
    }

    if let Ok(int_val) = word.parse::<i32>() {
        return Some(CondT::INT(int_val));
    }

    if let Ok(float_val) = word.parse::<f64>() {
        return Some(CondT::F(float_val));
    }

    for token in vars {
        if let Tokens::Var(var_type, var_name, _) = token {
            if var_name == word {
                return match var_type {
                    Vars::STR(_) => Some(CondT::STR(var_name.clone())),
                    Vars::INT(_) => Some(CondT::INT(0)),
                    Vars::F(_) => Some(CondT::F(0.0)),
                    _ => None,
                };
            }
        }
    }

    None
}

// Checks compatibility of two condT types
fn is_type_compatible(current_type: &CondT, new_type: &CondT) -> bool {
    matches!(
        (current_type, new_type),
        (CondT::STR(_), CondT::STR(_))
            | (CondT::INT(_), CondT::INT(_))
            | (CondT::F(_), CondT::F(_))
            | (CondT::INT(_), CondT::F(_))
            | (CondT::F(_), CondT::INT(_))
    )
}

// Suggests corrections for common operator mistakes
fn correct_operator(invalid_op: &str) -> &str {
    match invalid_op {
        "=<" => "<=",
        "=>" => ">=",
        "=!" => "!=",
        "===" => "==",
        "<>" => "!=",
        _ => "",
    }
}
