use std::collections::HashMap;

const DEBUG_PRINT: bool = false;

/// Public: Evaluates a math or logical expression with memory substitution.
pub fn evaluate_expression(expression: &str, memory: &HashMap<String, String>) -> Result<i32, String> {
    let cleaned = strip_quotes(expression);
    let resolved = resolve_placeholders(&cleaned, memory);
    let stripped = resolved.trim_matches('"').to_string();

    if DEBUG_PRINT {
        println!("Resolved expression: [{}]", stripped);
    }

    let tokens = tokenize(&stripped)?;
    if DEBUG_PRINT {
        println!("Tokens: {:?}", tokens);
    }

    let rpn = shunting_yard(&tokens)?;
    if DEBUG_PRINT {
        println!("RPN Output: {:?}", rpn);
    }

    evaluate_rpn(&rpn)
}

/// Public: Replaces all `{{math}}` blocks in a string with evaluated results.
pub fn resolve_math_placeholders(input: &str, memory: &HashMap<String, String>) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '{' && chars.peek() == Some(&'{') {
            chars.next(); // skip second '{'
            let mut expr = String::new();

            while let Some(&next) = chars.peek() {
                if next == '}' {
                    chars.next();
                    if chars.peek() == Some(&'}') {
                        chars.next();
                        break;
                    } else {
                        expr.push('}');
                    }
                } else {
                    expr.push(next);
                    chars.next();
                }
            }

            match evaluate_expression(&expr, memory) {
                Ok(result) => output.push_str(&result.to_string()),
                Err(_) => output.push_str("0"),
            }
        } else {
            output.push(c);
        }
    }

    output
}

fn strip_quotes(input: &str) -> &str {
    if input.starts_with('"') && input.ends_with('"') && input.len() >= 2 {
        &input[1..input.len() - 1]
    } else {
        input
    }
}

fn resolve_placeholders(input: &str, memory: &HashMap<String, String>) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '{' && chars.peek() == Some(&'{') {
            chars.next();
            let mut key = String::new();

            while let Some(&next) = chars.peek() {
                if next == '}' {
                    chars.next();
                    if chars.peek() == Some(&'}') {
                        chars.next();
                        break;
                    } else {
                        key.push('}');
                    }
                } else {
                    key.push(next);
                    chars.next();
                }
            }

            let resolved = memory.get(key.trim()).cloned().unwrap_or_else(|| "0".to_string());
            output.push_str(&resolved);
        } else {
            output.push(c);
        }
    }

    output
}

fn tokenize(expr: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut chars = expr.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            '0'..='9' => {
                let mut num = String::new();
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() {
                        num.push(d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(num);
            }
            '+' | '-' | '*' | '/' | '%' | '(' | ')' => {
                tokens.push(c.to_string());
                chars.next();
            }
            '=' | '!' | '<' | '>' => {
                let mut op = c.to_string();
                chars.next();
                if chars.peek() == Some(&'=') {
                    op.push('=');
                    chars.next();
                }
                tokens.push(op);
            }
            ' ' => {
                chars.next();
            }
            _ => return Err(format!("Math error: unexpected character '{}'", c)),
        }
    }

    Ok(tokens)
}

fn shunting_yard(tokens: &[String]) -> Result<Vec<String>, String> {
    let mut output = Vec::new();
    let mut ops: Vec<String> = Vec::new();

    for token in tokens {
        if token.parse::<i32>().is_ok() {
            output.push(token.clone());
        } else if is_operator(token) {
            while let Some(top) = ops.last() {
                if is_operator(top) && precedence(top) >= precedence(token) {
                    output.push(ops.pop().unwrap());
                } else {
                    break;
                }
            }
            ops.push(token.clone());
        } else if token == "(" {
            ops.push(token.clone());
        } else if token == ")" {
            let mut matched = false;
            while let Some(op) = ops.pop() {
                if op == "(" {
                    matched = true;
                    break;
                }
                output.push(op);
            }
            if !matched {
                return Err("Math error: unmatched ')'".to_string());
            }
        } else {
            return Err(format!("Math error: unknown token '{}'", token));
        }
    }

    while let Some(op) = ops.pop() {
        if op == "(" || op == ")" {
            return Err("Math error: mismatched parentheses".to_string());
        }
        output.push(op);
    }

    Ok(output)
}

fn evaluate_rpn(tokens: &[String]) -> Result<i32, String> {
    let mut stack: Vec<i32> = Vec::new();

    for token in tokens {
        if let Ok(n) = token.parse::<i32>() {
            stack.push(n);
        } else {
            let b = stack.pop().ok_or_else(|| "Math error: missing right operand".to_string())?;
            let a = stack.pop().ok_or_else(|| "Math error: missing left operand".to_string())?;

            let result = match token.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => {
                    if b == 0 {
                        return Err("Math error: division by zero".into());
                    }
                    a / b
                }
                "%" => {
                    if b == 0 {
                        return Err("Math error: modulo by zero".into());
                    }
                    a % b
                }
                "==" => (a == b) as i32,
                "!=" => (a != b) as i32,
                "<" => (a < b) as i32,
                "<=" => (a <= b) as i32,
                ">" => (a > b) as i32,
                ">=" => (a >= b) as i32,
                _ => return Err(format!("Math error: unknown operator '{}'", token)),
            };

            stack.push(result);
        }
    }

    stack.pop().ok_or_else(|| "Math error: expression produced no result".to_string())
}

fn is_operator(op: &str) -> bool {
    matches!(op, "+" | "-" | "*" | "/" | "%" | "==" | "!=" | "<" | "<=" | ">" | ">=")
}

fn precedence(op: &str) -> i32 {
    match op {
        "==" | "!=" | "<" | "<=" | ">" | ">=" => 0,
        "+" | "-" => 1,
        "*" | "/" | "%" => 2,
        _ => -1,
    }
}
