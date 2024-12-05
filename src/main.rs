use std::{env::args, process::exit};

use build::build;
use colored::Colorize;
use enable_ansi_support::enable_ansi_support;
use help::help;

pub mod build;
pub mod codegen;
mod err;
pub mod grm;
pub mod help;
mod lex;
pub mod nrunp;
mod p;
mod run;

fn main() {

    match enable_ansi_support(){
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}","Unable to enable ansii colors support :~ ANSI Code will be visible along lines , please ignore!".bright_yellow());
            eprintln!("{}{}","Specific Error Message :~ ",e);
        }
    }

    let args: Vec<String> = args().collect();
    if args.len() < 2 {
        help();
        exit(0)
    }
    let cmd = args[1].as_str();
    match cmd {
        "h" | "help" => help(),
        "build" => build(&args),
        "run" => { /*build the code*/ }
        _ => {
            eprintln!("Error :~ Unknown Command : {}\n", cmd);
            help();
            exit(1);
        }
    }
}
