use std::{fs::File, io::Read};

#[derive(Debug)]
pub struct Grammar {
    pub def: String,
    pub new: String,
}

pub fn process_files(
    input_file: &str,
    user_grammar_file: Option<&str>,
    neit_file: Option<&str>,
    ispymod: bool,
) -> String {
    let mut nc = String::new();
    let defengine = gen_grm();
    let mut usrgrm: Vec<Grammar> = Vec::new();

    println!("Processing files...");
    println!("Input file: {:?}", input_file);
    println!("User grammar file: {:?}", user_grammar_file);
    println!("Neit file: {:?}", neit_file);
    println!("Is PyMod: {:?}", ispymod);
    
    if !input_file.is_empty() {
        process_grammar_file(input_file, &mut usrgrm);
    }

    if let Some(file) = user_grammar_file {
        process_grammar_file(file, &mut usrgrm);
    }
    
    if let Some(file) = neit_file {
        nc = process_neit_file(file, &usrgrm, &defengine, ispymod);
    }
    
    nc
}

pub fn process_grammar_file(file_path: &str, usrgrm: &mut Vec<Grammar>) {
    println!("Processing grammar file: {}", file_path);
    let mut file = File::open(file_path).unwrap_or_else(|e| {
        eprintln!("Could not find the grammar file '{}'. Ensure it exists: {}", file_path, e);
        std::process::exit(1);
    });

    let mut content = String::new();
    if let Err(e) = file.read_to_string(&mut content) {
        eprintln!("Error reading the source grammar file '{}': {}", file_path, e);
        std::process::exit(1);
    }

    let mut index = 1;
    for ln in content.lines() {
        if ln.starts_with('#') {
            println!("Skipping comment line: {}", ln);
            continue;
        }

        let mut parts = ln.split('~');
        let ogv = parts.next().unwrap_or("").trim();
        let nv = parts.next().unwrap_or("").trim();

        if parts.next().is_some() || ogv.is_empty() || nv.is_empty() || ln.trim() == "py" {
            eprintln!(
                "Error on line({}) in the file '{}' : {}",
                index, file_path, ln
            );
            std::process::exit(1);
        }

        println!("Adding Grammar mapping: '{}' -> '{}'", ogv, nv);
        usrgrm.push(Grammar {
            def: ogv.to_string(),
            new: nv.to_string(),
        });
        index += 1;
    }

    println!("Successfully processed grammar file: {}", file_path);
}

pub fn process_neit_file(file_path: &str, usrgrm: &[Grammar], defengine: &[Grammar], ispymod: bool) -> String {
    println!("Processing NEIT file: {}", file_path);
    let mut nc = String::new();
    let mut file = File::open(file_path).unwrap_or_else(|_| {
        eprintln!("Could not open neit file '{}'", file_path);
        std::process::exit(1);
    });

    let mut content = String::new();
    if let Err(e) = file.read_to_string(&mut content) {
        eprintln!("Error reading file '{}': {}", file_path, e);
        std::process::exit(1);
    }

    let mut processed_lines = Vec::new();
    let mut prev_indent = 0;

    if ispymod {
        for line in content.lines() {
            let line_indent = count_leading_spaces(line);
            println!("Processing line: '{}' with indent level: {}", line, line_indent);
            
            // Add closing braces based on indentation levels
            while line_indent < prev_indent {
                processed_lines.push("}".to_string());
                println!("Closing function block with '}}' due to indentation decrease");
                prev_indent -= 4; // Assuming an indent level of 4 spaces for each level
            }

            if line_indent > prev_indent {
                // Add an opening brace for a new block
                processed_lines.push("{".to_string());
                println!("Opening function block with '{{' due to indentation increase");
                prev_indent = line_indent; // Update previous indent level
            }

            processed_lines.push(line.to_string());
            println!("Added line to processed lines: {}", line);
        }
    } else {
        for line in content.lines() {
            processed_lines.push(line.to_string());
            println!("Added line without PyMod processing: {}", line);
        }
    }

    // Process each line for word replacements
    let mut final_lines = Vec::new();
    for line in processed_lines {
        let mut modified_line = String::new();
        let mut current_word = String::new();
        let mut in_string_mode = false;

        for c in line.chars() {
            if c == '"' {
                in_string_mode = !in_string_mode;
                modified_line.push(c);
                continue;
            }

            if in_string_mode {
                modified_line.push(c);
            } else {
                if c.is_whitespace() || c.is_ascii_punctuation() {
                    if !current_word.is_empty() {
                        let replaced_word = replace_word(&current_word, usrgrm, defengine);
                        modified_line.push_str(&replaced_word);
                        println!("Replaced word: '{}' with: '{}'", current_word, replaced_word);
                        current_word.clear();
                    }
                    modified_line.push(c);
                } else {
                    current_word.push(c);
                }
            }
        }

        if !current_word.is_empty() {
            let replaced_word = replace_word(&current_word, usrgrm, defengine);
            modified_line.push_str(&replaced_word);
            println!("Replaced last word: '{}' with: '{}'", current_word, replaced_word);
        }

        final_lines.push(modified_line);
    }

    nc = final_lines.join("\n");
    println!("Successfully processed NEIT file: {}", file_path);
    nc
}

fn count_leading_spaces(line: &str) -> usize {
    let leading_spaces = line.chars().take_while(|&c| c == ' ').count();
    println!("Leading spaces in line '{}': {}", line, leading_spaces);
    leading_spaces
}

fn replace_word(word: &str, usrgrm: &[Grammar], defengine: &[Grammar]) -> String {
    for mapping in usrgrm.iter().chain(defengine.iter()) {
        if word == mapping.new {
            println!("Replacing word: '{}' with: '{}'", word, mapping.def);
            return mapping.def.clone();
        }
    }
    word.to_string()
}

pub fn gen_grm() -> Vec<Grammar> {
    vec![
        Grammar { def: "fn".to_string(), new: "fn".to_string() },
        Grammar { def: "may".to_string(), new: "may".to_string() },
        Grammar { def: "must".to_string(), new: "must".to_string() },
        Grammar { def: "pub".to_string(), new: "pub".to_string() },
        Grammar { def: "if".to_string(), new: "if".to_string() },
        Grammar { def: "case".to_string(), new: "case".to_string() },
        Grammar { def: "[cmode]".to_string(), new: "[cmode]".to_string() },
        Grammar { def: "![cmode]".to_string(), new: "![cmode]".to_string() },
        Grammar { def: "[c]".to_string(), new: "[c]".to_string() },
        Grammar { def: "![c]".to_string(), new: "![c]".to_string() },
        Grammar { def: "{".to_string(), new: "{".to_string() },
        Grammar { def: "}".to_string(), new: "}".to_string() },
        Grammar { def: "takein".to_string(), new: "takein".to_string() },
        Grammar { def: "println".to_string(), new: "println".to_string() },
        Grammar { def: "print".to_string(), new: "print".to_string() },
        Grammar { def: "=".to_string(), new: "=".to_string() },
    ]
}
