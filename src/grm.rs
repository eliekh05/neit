use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[allow(unused)]
#[derive(Debug)]
enum ProgramError {
    FileNotFound,
    FileReadError(io::Error),
    InvalidGrammarFormat,
}

impl ProgramError {
    fn description(&self) -> String {
        match self {
            ProgramError::FileNotFound => "Grammar file not found.".to_string(),
            ProgramError::FileReadError(err) => format!("Error reading file: {}", err),
            ProgramError::InvalidGrammarFormat => "Invalid grammar format detected.".to_string(),
        }
    }
}

pub fn pgrm(data: &mut String, grmf: &str) {
    let path = Path::new(grmf);
    println!("Starting Semigen Engine...");

    if !path.exists() || !path.is_file() {
        eprintln!("{}", ProgramError::FileNotFound.description());
        return;
    }
    let file = match File::open(grmf) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", ProgramError::FileReadError(e).description());
            return;
        }
    };

    let reader = io::BufReader::new(file);
    let mut grammar_map = HashMap::new();
    println!("Parsing grammar file...");
    for line in reader.lines() {
        if let Ok(line_str) = line {
            let cleaned_line = line_str.trim();
            if cleaned_line.is_empty() {
                continue;
            }

            let mut parts = cleaned_line.splitn(2, '~');
            if let (Some(original), Some(replacement)) = (parts.next(), parts.next()) {
                let original = original.trim();
                let replacement = replacement.trim();
                if !original.is_empty() && !replacement.is_empty() {
                    grammar_map.insert(replacement.to_string(), original.to_string());
                } else {
                    eprintln!("{}", ProgramError::InvalidGrammarFormat.description());
                    return;
                }
            } else {
                eprintln!("{}", ProgramError::InvalidGrammarFormat.description());
                return;
            }
        }
    }
    let mut result = String::new();
    println!("Applying grammar rules...");
    for line in data.lines() {
        let mut words = line.split_whitespace();
        if let Some(first_word) = words.next() {
            if let Some(replacement) = grammar_map.get(first_word) {
                result.push_str(replacement);
                result.push(' ');
            } else {
                result.push_str(first_word);
                result.push(' ');
            }
            result.push_str(words.clone().collect::<Vec<&str>>().join(" ").as_str());
        }
        result.push('\n');
    }
    println!("Grammar applied successfully.");
    *data = result;
}
