use colored::*;

#[allow(unused)]
/// Enum representing various error types that can occur during parsing.
pub enum ErrT {
    /// Represents an invalid value assigned to a variable.
    /// Holds:
    /// - `usize`: Line number where the error occurred.
    /// - `String`: The problematic value or expression causing the error.
    InValidVarVal(usize, String),

    /// Represents a variable not found in the current scope.
    /// Holds:
    /// - `usize`: Line number where the error occurred.
    /// - `String`: The name of the variable that was not found.
    VNF(usize, String),

    /// Represents a string with unmatched quotes.
    /// Holds:
    /// - `usize`: Line number where the error occurred.
    /// - `String`: The string with the unmatched quotes.
    UnMQ(usize, String),

    /// Represents a missing or misplaced equal sign in a variable declaration.
    /// Holds:
    /// - `usize`: Line number where the error occurred.
    /// - `String`: The code fragment where the issue was detected.
    EqNF(usize, String),
    ///Invalid Time Value for 'wait' command
    InVTimeVal(usize, String),
}
pub fn generr(err: ErrT, codes: &Vec<&str>) {
    match err {
        ErrT::InValidVarVal(line, value) => {
            let codeline = &codes[line - 1];
            println!("{}", "ERROR: Invalid Variable Assignment".bold().red());
            println!(
                " ├─ {} {}",
                "Line:".bright_white(),
                format!("{}", line).yellow().bold()
            );
            println!(
                " ├─ {} {}",
                "Cause:".bright_white(),
                format!(
                    "The value `{}` assigned to the variable is ambiguous or incompatible.",
                    value
                )
                .yellow()
            );
            println!(" ├─ {}", "Explanation:".bright_white());
            println!(
                " │   {}",
                "This error typically occurs when a value's type cannot be inferred,".bright_cyan()
            );
            println!(
                " │   {}",
                "or the value violates type rules (e.g., assigning a string to an integer)."
                    .bright_cyan()
            );
            println!(
                " │   {}",
                "Check the assignment value for typos, or ensure it's compatible with the variable's type."
                    .bright_cyan()
            );
            println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
        }
        ErrT::VNF(line, var_name) => {
            let codeline = &codes[line - 1];
            println!("{}", "ERROR: Undeclared Variable Reference".bold().red());
            println!(
                " ├─ {} {}",
                "Line:".bright_white(),
                format!("{}", line).yellow().bold()
            );
            println!(
                " ├─ {} {}",
                "Cause:".bright_white(),
                format!(
                    "The variable `{}` has not been declared in the current scope.",
                    var_name
                )
                .yellow()
            );
            println!(" ├─ {}", "Explanation:".bright_white());
            println!(
                " │   {}",
                "This occurs when attempting to use a variable name that is undefined or misspelled."
                    .bright_cyan()
            );
            println!(
                " │   {}",
                "Declare the variable before using it, and ensure there are no typos in the name."
                    .bright_cyan()
            );
            println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
        }
        ErrT::UnMQ(line, string_literal) => {
            let codeline = &codes[line - 1];
            println!("{}", "ERROR: Unmatched Quotes Detected".bold().red());
            println!(
                " ├─ {} {}",
                "Line:".bright_white(),
                format!("{}", line).yellow().bold()
            );
            println!(
                " ├─ {} {}",
                "Cause:".bright_white(),
                format!(
                    "The string `{}` starts or ends with a quote but lacks a matching pair.",
                    string_literal
                )
                .yellow()
            );
            println!(" ├─ {}", "Explanation:".bright_white());
            println!(
                " │   {}",
                "String literals must start and end with matching quotes (either single or double)."
                    .bright_cyan()
            );
            println!(
                " │   {}",
                "Check for missing or mismatched quotes in the provided string literal."
                    .bright_cyan()
            );
            println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
        }
        ErrT::EqNF(line, fragment) => {
            let codeline = &codes[line - 1];
            println!("{}", "ERROR: Equal Sign Not Found".bold().red());
            println!(
                " ├─ {} {}",
                "Line:".bright_white(),
                format!("{}", line).yellow().bold()
            );
            println!(
                " ├─ {} {}",
                "Cause:".bright_white(),
                format!(
                    "The `=` sign is missing or misplaced in the statement: `{}`.",
                    fragment
                )
                .yellow()
            );
            println!(" ├─ {}", "Explanation:".bright_white());
            println!(
                " │   {}",
                "The equal sign (`=`) is used to assign a value to a variable.".bright_cyan()
            );
            println!(
                " │   {}",
                "Ensure that the `=` is placed after the variable name and before the value."
                    .bright_cyan()
            );
            println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
        }
        ErrT::InVTimeVal(line, fragment) => {
            let codeline = &codes[line - 1];
            println!("{}", "ERROR: Invalid Time Value".bold().red());
            println!(
                " ├─ {} {}",
                "Line:".bright_white(),
                format!("{}", line).yellow().bold()
            );
            println!(
                " ├─ {} {}",
                "Cause:".bright_white(),
                format!(
                    "The time value is either missing or incorrectly formatted in the statement: `{}`.",
                    fragment
                )
                .yellow()
            );
            println!(" ├─ {}", "Explanation:".bright_white());
            println!(
                " │   {}",
                "Ensure that the time value is correctly specified with a valid format."
                    .bright_cyan()
            );
            println!(
                " │   {}",
                "For example, use time values like `1s`, `500ms`, `2m`, `1hr` for seconds, milliseconds, minutes, or hours"
                    .bright_cyan()
            );
            println!(" └─ {} {}", "Code:".bright_white(), codeline.red().italic());
        }
    }
}
