use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Token {
    pub token_type: String,
    pub value: String,
    pub modifiers: Vec<String>,
    pub indentation: usize,
}

/// Tokenizes an .idc file line-by-line using indentation rules and quoted string awareness
pub fn tokenize_file(file_path: &str) -> io::Result<Vec<Token>> {
    let file = File::open(file_path)
        .map_err(|e| io::Error::new(io::ErrorKind::NotFound, format!("Error opening file: {}", e)))?;
    let reader = BufReader::new(file);
    let mut tokens = Vec::new();

    let foundational_40: HashSet<&str> = [
        "on", "every", "if", "unless", "is", "has", "load", "save", "remember", "get", "respond",
        "show", "log", "retry", "else", "then", "wait", "as", "with", "from", "to", "authorize",
        "purge", "reset", "stop", "continue", "alert", "check", "verify", "update", "delete",
        "create", "connect", "disconnect", "use", "define", "alias", "capture", "test", "print", "route",
        "true", "false", "while", "break", "respond_event", "start", "end", "trigger", "event", "reflect",
        "set"
    ].iter().copied().collect();

    for (_line_number, line) in reader.lines().enumerate() {
        let line = line?;
        let trimmed = line.trim_start();
        let indentation = line.len() - trimmed.len();

        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        let mut parts = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut paren_depth = 0;

        for c in trimmed.chars() {
            match c {
                '"' => {
                    in_quotes = !in_quotes;
                    current.push(c);
                }
                '(' if !in_quotes => {
                    paren_depth += 1;
                    current.push(c);
                }
                ')' if !in_quotes => {
                    paren_depth -= 1;
                    current.push(c);
                }
                ' ' if !in_quotes && paren_depth == 0 => {
                    if !current.is_empty() {
                        parts.push(current.clone());
                        current.clear();
                    }
                }
                _ => current.push(c),
            }
        }
        if !current.is_empty() {
            parts.push(current);
        }

        let first = parts.get(0).cloned().unwrap_or_else(|| "unknown".to_string());
        let token_type = if foundational_40.contains(first.as_str()) {
            first
        } else {
            "unknown".to_string()
        };

        let value = parts.get(1).cloned().unwrap_or_default();
        let modifiers = if parts.len() > 2 {
            parts[2..].to_vec()
        } else {
            vec![]
        };

        tokens.push(Token {
            token_type,
            value,
            modifiers,
            indentation,
        });
    }

    Ok(tokens)
}