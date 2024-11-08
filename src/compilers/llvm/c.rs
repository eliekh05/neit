use crate::{
    utils::{
        tokens::print::p_to_c,
        types::{Args, Tokens, Vars},
    },
    UCMF, UCMI,
};
use std::{collections::HashSet, process::exit};

use super::bc::cfmt;

pub fn to_c(tokens: &[Tokens]) -> String {
    let mut c_code = String::with_capacity(1024);
    c_code.push_str("#include <stdio.h>\n#include <string.h>\n");
    let mut funs = String::with_capacity(512);
    c_code.push_str("int fdi(int a, int b);\n");
    c_code.push_str("double fdf(double a, double b);\n");

    let mut declared_vars: HashSet<String> = HashSet::new();
    let mut ccind = 0;
    for token in tokens {
        if let Tokens::Func(fun) = token {
            let arg_vars: Vec<&String> = fun
                .args
                .iter()
                .map(|arg| match arg {
                    Args::Str(name) | Args::Int(name) | Args::Float(name) => name,
                    Args::EMP(e) => e,
                    _ => unreachable!(),
                })
                .collect();

            funs.push_str(&format!("void {}({}) {{\n", fun.name, make_args(&fun.args)));
            process(
                &mut funs,
                &arg_vars,
                true,
                &fun.code,
                &mut declared_vars,
                &mut ccind,
            );
            funs.push_str("}\n\n");
        }
    }

    c_code.push_str("int main() {\n");
    let nft: Vec<Tokens> = tokens
        .iter()
        .filter(|token| !matches!(token, Tokens::Func(_)))
        .cloned() // Clone the Tokens to get owned values
        .collect();

    let non_function_tokens: &[Tokens] = &nft; // Now it's a slice of Tokens

    process(
        &mut c_code,
        &[],
        false,
        non_function_tokens,
        &mut declared_vars,
        &mut ccind,
    );

    if unsafe { UCMI } {
        funs.push_str("int fdi(int a, int b) {\nif (b == 0) return 0;\nint result = a / b;\nif ((a % b != 0) && ((a < 0) != (b < 0))) result--;\nreturn result;\n}\n");
    }
    if unsafe { UCMF } {
        funs.push_str("double fdf(double a, double b) {\nif (b == 0.0) return 0.0;\ndouble result = a / b;\nreturn (result > 0 && result != (int)result) ? (int)result : (result < 0 && result != (int)result) ? (int)result - 1 : result;\n}\n");
    }

    c_code.push_str("return 0;\n}\n");
    c_code.push_str(&funs);
    c_code = cfmt(&c_code);
    c_code
}

fn process(
    func: &mut String,
    arg_vars: &[&String],
    iff: bool,
    tokens: &[Tokens],
    declared_vars: &mut HashSet<String>,
    cci: &mut i32,
) {
    let mut ini = 0;
    for token in tokens {
        match token {
            Tokens::IFun(_name, code) if iff => {
                let mut gcc = String::new();
                process(&mut gcc, arg_vars, false, code, declared_vars, cci);
                func.push_str(&gcc);
            }
            Tokens::CCode(c) => {
                func.push_str(format!("{}\n", c.join("\n")).as_str());
            }
            Tokens::Cond(conds) => {
                let mut condc = String::new();
                let mut else_block = String::new();
                let mut last_condition_found = false;

                for (i, s) in conds.iter().enumerate() {
                    if s.trim().is_empty() {
                        continue;
                    }

                    let pts: Vec<&str> = s.split(':').collect();
                    if pts.len() != 2 {
                        eprintln!("✘ Error: Invalid Condition Format");
                        continue;
                    }

                    let cond = pts[0].trim();
                    let code = pts[1].trim();

                    if cond == "last" {
                        if last_condition_found {
                            eprintln!("Error! Multiple 'last' conditions found.");
                            exit(1);
                        }
                        last_condition_found = true;

                        for t in tokens {
                            if let Tokens::IFun(n, c) = t {
                                if n == code {
                                    let mut addc = String::new();
                                    process(&mut addc, arg_vars, true, c, declared_vars, cci);
                                    else_block.push_str(&format!("{}\n", addc));
                                }
                            }
                        }
                        continue;
                    }

                    condc.push_str(&format!(
                        "{}if ({}) {{\n",
                        if i == 0 { "" } else { "else " },
                        cond
                    ));

                    for t in tokens {
                        if let Tokens::IFun(n, c) = t {
                            if n == code {
                                let mut addc = String::new();
                                process(&mut addc, arg_vars, true, c, declared_vars, cci);
                                condc.push_str(&addc);
                            }
                        }
                    }
                    condc.push_str("}\n");
                }

                if last_condition_found {
                    condc.push_str("else {\n");
                    condc.push_str(&else_block);
                    condc.push_str("}\n");
                }
                func.push_str(&condc);
            }

            Tokens::Print(v, _) => {
                let pc = p_to_c(v, &tokens.to_vec());
                func.push_str(&format!("printf({});\n", pc));
            }
            Tokens::In(vnm) => {
                func.push_str(&format!("fgets({}, sizeof({}) - 1, stdin);\nsize_t len_{} = strcspn({}, \"\\n\");\n{}[len_{}] = '\\0';\n", vnm, vnm, vnm, vnm,vnm,vnm));
                ini = ini + 1;
            }

            Tokens::FnCall(fc, args) => {
                func.push_str(&format!("{}({});\n", fc, args.join(",")));
            }
            Tokens::Var(v, n, mutable) => {
                if arg_vars.contains(&n) || declared_vars.contains(n) {
                    continue;
                }

                declared_vars.insert(n.clone());
                let var_declaration = match (v, *mutable) {
                    (Vars::STR(s), true) => format!("char {}[{}] = \"{}\";\n", n, n.len() + 333, s),
                    (Vars::INT(s), true) => format!("int {} = {};\n", n, s),
                    (Vars::F(f), true) => format!("double {} = {};\n", n, f),
                    (Vars::STR(s), false) => format!("const char *{} = \"{}\";\n", n, s),
                    (Vars::INT(s), false) => format!("const int {} = {};\n", n, s),
                    (Vars::F(f), false) => format!("const double {} = {};\n", n, f),
                    _ => unreachable!("✘ Error: Unsupported variable type."),
                };

                func.push_str(&var_declaration);
            }
            Tokens::Revar(n, v) => {
                func.push_str(&format!("{} = {};\n", n, v));
            }
            _ => {}
        }
    }
}

fn make_args(args: &[Args]) -> String {
    let mut farg = String::with_capacity(256);
    for (i, arg) in args.iter().enumerate() {
        match arg {
            Args::Str(name) => farg.push_str(&format!("char *{}", name)),
            Args::Int(name) => farg.push_str(&format!("int {}", name)),
            Args::Float(name) => farg.push_str(&format!("double {}", name)),
            Args::EMP(_) => {}
            Args::E => {}
        }
        if i < args.len() - 1 {
            farg.push_str(", ");
        }
    }
    farg
}
