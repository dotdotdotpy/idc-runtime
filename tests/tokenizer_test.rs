use crate::parser::tokenizer::{tokenize_file, Token};

#[test]
fn test_tokenize_basic_idc() {
    // Call the tokenizer function
    let result = tokenize_file("examples/basic.idc");
    assert!(result.is_ok(), "Failed to tokenize file");

    // Get the tokens
    let tokens = result.unwrap();
    assert!(tokens.len() > 0, "No tokens were generated");

    // Test that the first token is 'on'
    let first_token = &tokens[0];
    assert_eq!(first_token.token_type, "on", "First token should be 'on'");
    assert_eq!(first_token.value, "\"start\"", "First token value should be '\"start\"'");

    // Test token count (optional based on your expectations)
    assert_eq!(tokens.len(), 11, "Token count mismatch");  // Adjust the expected length based on your file

    // Test a specific token (optional)
    let second_token = &tokens[1];
    assert_eq!(second_token.token_type, "respond", "Second token should be 'respond'");
    assert_eq!(second_token.modifiers, vec!["complete\""], "Second token should have the correct modifier");
}

#[test]
fn test_true_and_false_conditions() {
    // Test for tokens "true" and "false" being correctly identified as conditions
    let result = tokenize_file("examples/true_false.idc");
    assert!(result.is_ok(), "Failed to tokenize file");

    let tokens = result.unwrap();

    let first_token = &tokens[0];
    assert_eq!(first_token.token_type, "true", "First token should be 'true'");

    let second_token = &tokens[1];
    assert_eq!(second_token.token_type, "false", "Second token should be 'false'");
}

#[test]
fn test_else_condition() {
    // Test for 'else' being correctly classified
    let result = tokenize_file("examples/else_condition.idc");
    assert!(result.is_ok(), "Failed to tokenize file");

    let tokens = result.unwrap();

    let first_token = &tokens[0];
    assert_eq!(first_token.token_type, "if", "First token should be 'if'");

    let second_token = &tokens[1];
    assert_eq!(second_token.token_type, "else", "Second token should be 'else'");
}

#[test]
fn test_invalid_tokenize() {
    // Test for an invalid file
    let result = tokenize_file("examples/invalid.idc");
    assert!(result.is_err(), "Tokenization should fail for an invalid file");
}
