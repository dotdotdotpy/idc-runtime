use crate::interpreter::math::evaluate_expression;
use crate::runtime::memory::MemoryStore;

/// Resolves an expression or literal key using the current memory.
/// If the string is quoted or contains math symbols, it will be evaluated.
/// Returns Result<String, String> to unify error handling.
pub fn resolve_key_or_expression(
    input: &str,
    memory: &MemoryStore,
) -> Result<String, String> {
    let trimmed = input.trim_matches('"');

    let has_math = trimmed.contains('+')
        || trimmed.contains('-')
        || trimmed.contains('*')
        || trimmed.contains('/')
        || trimmed.contains('%')
        || trimmed.contains('(')
        || trimmed.contains(')')
        || trimmed.contains("{{");

    if has_math {
        match evaluate_expression(trimmed, &memory.flatten_map()) {
            Ok(n) => Ok(n.to_string()),
            Err(e) => Err(format!("Failed to evaluate expression '{}': {}", input, e)),
        }
    } else {
        Ok(trimmed.to_string())
    }
}
