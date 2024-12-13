use colored::Colorize;

pub fn help() {
    println!("{}", "Neit Programming Language".green().bold());
    println!("{}", "==========================".green());

    println!("\n{}", "COMMANDS:".cyan().bold());
    println!(
        "  {}  {}",
        "h | help".blue().bold(),
        "Display this help message.".green()
    );
    println!(
        "  {}  {}",
        "run".yellow().bold(),
        "Lex, parse, generate, and execute Neit code.".green()
    );
    println!(
        "  {}  {}",
        "build".yellow().bold(),
        "Lex, parse, generate C code, and compile it.".green()
    );

    println!("{}", "\nOPTIONS:".cyan().bold());
    println!(
        "  {}  {}",
        "-o | --out=<file_name>".blue().bold(),
        "Specify the output file name (default: 'output').".green()
    );
    println!(
        "  {}  {}",
        "-static".blue().bold(),
        "Generate a statically linked binary.".green()
    );
    println!(
        "  {}  {}",
        "-opt=<LEVEL> | --optimisation=<level>".blue().bold(),
        "Choose the optimization level:".green()
    );
    println!(
        "    {}  {} (Minor optimization)",
        "1:".yellow(),
        "Faster compilation, slight improvements.".cyan()
    );
    println!(
        "    {}  {} (Balanced optimization)",
        "2:".yellow(),
        "Improves speed and size moderately.".cyan()
    );
    println!(
        "    {}  {} (Major optimization)",
        "3:".yellow(),
        "Maximizes performance and reduces size.".cyan()
    );
    println!(
        "    {}  {} (Aggressive optimization)",
        "4:".yellow(),
        "Longest compile time, but fastest and smallest output.".cyan()
    );
    println!(
        "  {}  {}",
        "-t | --target=<OS_NAME>".blue().bold(),
        "Specify the target OS for compilation.".green()
    );
    println!(
        "    {}  ",
        "Supported: 'linux', 'windows', and others.".cyan(),
    );
    println!(
        "  {}  {}",
        "-rc | --retain-c".blue().bold(),
        "Retain the generated C file after building.".green()
    );
    println!(
        "  {}  {}",
        "-g=<file> | --grammar=<file>".blue().bold(),
        "Use a custom grammar file for grammar rules".green()
    );

    println!("{}", "==========================".green());
}
