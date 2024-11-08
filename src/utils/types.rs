use std::fmt;

use super::maths::evaluate_expression;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Tokens {
    Func(FN),
    FnCall(String, Vec<String>), /* String -> name of function , Vec<String> -> Args */
    Print(String, String), /* String -> Text to print stored on | rax:1(sys_write) , rsi:text , rdx:size/len_of_text , rdi:1 (1 for stdout)*/
    Var(Vars, String, bool), /* Vars -> Variable Data | String -> Variable Name | bool -> is change-able*/
    Revar(String, String),   /* Name , Value */
    In(String),              /* String -> Variable name to take input in */
    IFun(String, Vec<Tokens>),
    Cond(Vec<String>),  /* Vec<String> -> Conditions and case to call*/
    CCode(Vec<String>), /* Vec<String> -> C Codes */
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tokens::Func(func) => write!(f, "Function: {}\n", func.name),
            Tokens::FnCall(name, args) => {
                let args_display = args.join(", ");
                write!(f, "Function Call: {}({})\n", name, args_display)
            }
            Tokens::Print(text, _) => write!(f, "Print: \"{}\"\n", text),
            Tokens::Var(var_type, name, is_changeable) => {
                let changeable = if *is_changeable {
                    "mutable"
                } else {
                    "immutable"
                };
                write!(
                    f,
                    "Variable: {} - {:?} ({} variable)\n",
                    name, var_type, changeable
                )
            }
            Tokens::Revar(name, value) => write!(f, "Reassign Variable: {} = {}\n", name, value),
            Tokens::In(value) => write!(f, "Input: {}\n", value),
            Tokens::CCode(c) => {
                write!(
                    f,
                    "Plain C Code:\n{:?}",
                    for c in c {
                        println!("{}", format!("{}", c));
                    }
                )
            }
            Tokens::IFun(var_name, tokens) => {
                let tokens_display = tokens
                    .iter()
                    .map(|t| format!("{}", t))
                    .collect::<Vec<_>>()
                    .join("");
                write!(
                    f,
                    "Input Function: {} with tokens:\n{}",
                    var_name, tokens_display
                )
            }
            Tokens::Cond(conds) => {
                let conds = conds.join("\n     ");
                write!(f, "Conds:\n{}", conds)
            }
        }
    }
}

pub fn get_vars(tokens: &Vec<fvars>) -> Vec<Vars> {
    let mut vrs: Vec<Vars> = Vec::new();
    for i in tokens {
        vrs.push(i.v.clone());
    }
    vrs
}

pub fn get_vars_tkns(tokens: &Vec<Tokens>) -> Vec<Vars> {
    let mut vrs: Vec<Vars> = Vec::new();
    for i in tokens {
        if let Tokens::Var(v, _, _) = i {
            vrs.push(v.clone());
        }
    }
    vrs
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct FN {
    pub name: String,
    pub is_global: bool,
    pub code: Vec<Tokens>,
    pub args: Vec<Args>,
    pub local_vars: Vec<fvars>,
    /* variable system */
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct fvars {
    pub v: Vars,
    pub n: String,
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Args /* type(name_of_arg)*/ {
    Str(String),
    Int(String),
    Float(String),
    EMP(String),
    E,
}

impl Args {
    pub fn new(name: String, t: &str) -> Args {
        match t {
            "string" => Args::Str(name),
            "int" => Args::Int(name),
            "float" => Args::Float(name),
            "emp" => Args::EMP(name),
            _ => Args::E,
        }
    }
}

impl FN {
    pub fn new(
        name: String,
        is_global: bool,
        code: Vec<Tokens>,
        args: Vec<Args>,
        local_vars: Vec<fvars>,
    ) -> Self {
        FN {
            name,
            is_global,
            code,
            args,
            local_vars,
        }
    }

    pub fn add_code(&mut self, tkn: Tokens) {
        self.code.push(tkn);
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Vars {
    STR(String),
    INT(i64),
    F(f64),
    EX(String),
}

#[allow(dead_code)]
impl Default for Vars {
    fn default() -> Self {
        Self::new()
    }
}

impl Vars {
    pub fn to_asm(&self, name: String, counter: i32) -> String {
        match self {
            Vars::STR(value) => format!(
                "\n{name}_{counter} db '{value}', 0\n",
                name = name,
                value = value
            ),
            Vars::INT(value) => format!("\n{name} dq {value}\n", name = name, value = value),
            Vars::F(value) => format!(
                "\n{name} dq {value}\n",
                name = name,
                value = f64_to_bits(*value) // Convert float to its bit representation
            ),
            Vars::EX(_) => String::from("\n"), // We'll skip EX for now since it's more complex
        }
    }

    pub fn new() -> Vars {
        Vars::STR("___|___".to_string())
    }

    pub fn update_type(&mut self, value: &str, vrs: &Vec<Tokens>) -> Result<Vars, String> {
        let value = value.trim();

        // Check if the value is a string (enclosed in quotes)
        if value.starts_with('"') && value.ends_with('"') {
            let str_value = value.trim_matches('"').to_string();
            *self = Vars::STR(str_value);
            return Ok(self.clone());
        }

        // Attempt to parse the value as an integer
        if let Ok(int_value) = value.parse::<i64>() {
            *self = Vars::INT(int_value);
            return Ok(self.clone());
        }

        // Attempt to parse the value as a float
        if let Ok(float_value) = value.parse::<f64>() {
            *self = Vars::F(float_value);
            return Ok(self.clone());
        }

        // Check if the value is a valid expression
        match evaluate_expression(value, &mut vrs.clone()) {
            Ok(result) => {
                if result.to_string().contains(".") {
                    *self = Vars::F(result);
                } else {
                    match result.to_string().parse::<i64>() {
                        Ok(int_value) => *self = Vars::INT(int_value),
                        Err(_) => return Err("✘ Error: I couldn’t parse that value as an integer—it's playing hard to get! \n\
                        → Make sure the value looks like a proper integer; no sneaky decimals or funny business!".to_string()),
                    }
                }
                Ok(self.clone())
            }

            Err(_e) => {
                if value.ends_with(";") {
                    return Err(format!(
                        "Oh no! A rogue semicolon detected at the end of your value: '{}'\n\
                        Did you really think you could bring that here? \n\
                        This is a semicolon-free zone, my friend!\n\
                        Let’s keep your code classy—remove it before it starts a riot!",
                        value.trim_end_matches(';') // Display the value without the semicolon
                    ));
                }

                return Err(format!(
                    "✘ Error: I tried to make sense of the value '{}' but it just wouldn’t play nice! \n\
                    → It couldn’t be parsed as a valid type.\n\
                    →→ Hint: Make sure your value is in the right format—like a string (\"string\"), an integer (123), a float (123.45), or even an expression (e.g., a+b).",
                    value
                ));
            }
        }

        // If all parsing attempts fail, return an error
    }
}

fn f64_to_bits(f: f64) -> u64 {
    f.to_bits()
}
