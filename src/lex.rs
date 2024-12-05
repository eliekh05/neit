#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tokens {
    ttype: TokType,
    value: String,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TokType {
    CMD,
    SPACE,
    EOL,
    INSTR,
    OP,
}

impl Tokens {
    pub fn new() -> Vec<Tokens> {
        Vec::with_capacity(100) // Pre-allocate capacity to avoid reallocations
    }

    pub fn mktok(ttype: TokType, value: String) -> Tokens {
        Tokens { ttype, value }
    }

    pub fn get_type(&self) -> TokType {
        self.ttype
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }
}

pub fn lex(code: &str, toks: &mut Vec<Tokens>) {
    /// Utility function to check if the character is part of a newline sequence.
    ///
    /// Handles:
    /// - `\n` (Unix, Linux, modern macOS)
    /// - `\r\n` (Windows)
    /// - `\r` (Classic macOS, legacy systems)
    fn nl(c: char, next: Option<char>, iter: &mut std::iter::Peekable<std::str::Chars>) -> bool {
        if c == '\r' && next == Some('\n') {
            iter.next();
            true
        } else {
            c == '\n' || c == '\r'
        }
    }

    let mut iter = code.chars().peekable();
    let mut fw = true; // Flag for first token
    let mut curt = String::with_capacity(64);
    let mut in_multiline_comment = false;

    while let Some(&c) = iter.peek() {
        iter.next();
        let next = iter.peek().copied();
        if in_multiline_comment {
            if c == '#' && iter.peek() == Some(&'#') {
                iter.next(); // Skip second '#'
                in_multiline_comment = false;
            }
            continue;
        } else if nl(c, next, &mut iter) {
            if !curt.is_empty() {
                if fw {
                    toks.push(Tokens::mktok(TokType::CMD, curt.clone()));
                } else {
                    toks.push(Tokens::mktok(TokType::INSTR, curt.clone()));
                }
                curt.clear();
            }
            fw = true;

            toks.push(Tokens::mktok(TokType::EOL, String::from("\n")));
            continue;
        }

        match c {
            '#' if iter.peek() == Some(&'#') => {
                iter.next(); // Skip second '#'
                in_multiline_comment = true;
            }
            '#' => {
                while let Some(&next_c) = iter.peek() {
                    if next_c == '\n' {
                        toks.push(Tokens::mktok(TokType::EOL, String::from("\n")));
                        break;
                    }
                    iter.next(); // Consume characters in single-line comment
                }
            }
            ' ' => {
                if !curt.is_empty() {
                    if fw {
                        toks.push(Tokens::mktok(TokType::CMD, curt.clone()));
                        fw = false;
                    } else {
                        toks.push(Tokens::mktok(TokType::INSTR, curt.clone()));
                    }
                    curt.clear();
                }
                toks.push(Tokens::mktok(TokType::SPACE, String::from(" ")));
            }
            _ if c.is_ascii_punctuation() => {
                if !curt.is_empty() {
                    if fw {
                        toks.push(Tokens::mktok(TokType::CMD, curt.clone()));
                        fw = false;
                    } else {
                        toks.push(Tokens::mktok(TokType::INSTR, curt.clone()));
                    }
                    curt.clear();
                }
                toks.push(Tokens::mktok(TokType::OP, c.to_string()));
            }
            _ => {
                curt.push(c); // Accumulate characters into the current token
            }
        }
    }

    if !curt.is_empty() {
        if fw {
            toks.push(Tokens::mktok(TokType::CMD, curt.clone()));
        } else {
            toks.push(Tokens::mktok(TokType::INSTR, curt.clone()));
        }
    }

    toks.push(Tokens::mktok(TokType::EOL, String::from("\n")));
}
