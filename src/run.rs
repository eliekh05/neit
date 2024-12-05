use clearscreen::clear;
use colored::Colorize;
use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
    process::exit,
    thread::sleep,
    time::Duration,
};

use crate::p::{VVal, NST};

#[allow(unused)]
pub fn run(nst: &[NST]) {
    let mut vars: HashMap<String, VVal> = HashMap::new();

    for t in nst {
        if let NST::Var(v) = t {
            vars.insert(v.name.clone(), v.value.clone());
        }
    }

    for t in nst {
        if let NST::PRINT(txt) = t {
            let mut inv = false;
            let mut vname = String::new();
            let mut output = String::new();

            let mut chars = txt.chars().peekable();
            while let Some(c) = chars.next() {
                if c == '\\' {
                    if let Some(next_char) = chars.next() {
                        let escaped = match next_char {
                            '\\' => '\\',
                            'n' => '\n',
                            't' => '\t',
                            '"' => '"',
                            _ => next_char,
                        };
                        output.push(escaped);
                    }
                } else if c == '{' {
                    inv = true;
                } else if inv {
                    if c == '}' {
                        if let Some(value) = vars.get(&vname) {
                            output.push_str(
                                &(match value {
                                    VVal::Str(s) => s.clone(),
                                    VVal::Int(i) => i.to_string(),
                                    VVal::F(f) => f.to_string(),
                                    VVal::VarRef(ref_name, _) => resolve_variable(ref_name, &vars),
                                })
                                .to_string(),
                            );
                        }
                        vname.clear();
                        inv = false;
                    } else {
                        vname.push(c);
                    }
                } else {
                    output.push(c);
                }
            }

            print!("{}", output);
            if let Err(e) = stdout().flush() {
                eprintln!("{}", format!("STDOUT failed: {}", e).red());
                exit(1);
            }
        } else if let NST::Input(v) = t {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            vars.insert(v.to_string(), VVal::Str(input.trim().to_string()));
        } else if let NST::NCLRSCRN = t {
            match clear() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", format!("Failed to clear screen: {}", e).red());
                    exit(1);
                }
            }
        } else if let NST::WAIT(t) = t {
            sleep(Duration::from_millis(*t));
        }
    }
}

fn resolve_variable(name: &str, vars: &HashMap<String, VVal>) -> String {
    if let Some(value) = vars.get(name) {
        match value {
            VVal::Str(s) => s.clone(),
            VVal::Int(i) => i.to_string(),
            VVal::F(f) => f.to_string(),
            VVal::VarRef(ref_name, _) => resolve_variable(ref_name, vars),
        }
    } else {
        format!("Unknown variable: {}", name)
    }
}
