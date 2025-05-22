use crate::parser::ast::AstNode;

/// Enum for semantic-level issues discovered during validation
#[derive(Debug, Clone)]
pub enum SemanticWarning {
    UnresolvedReference(String),
    MissingRespond,
    InvalidFlow(String),
    UnknownNodeType(String),
    InvalidConfiguration(String),
}

/// Analyzes AST nodes for semantic issues like missing values, empty blocks, or unknown types
pub fn analyze_semantics(ast: &[AstNode]) -> Vec<SemanticWarning> {
    let mut warnings = Vec::new();

    fn walk(node: &AstNode, warnings: &mut Vec<SemanticWarning>) {
        match node.node_type.as_str() {
            "get" if node.value.is_empty() => warnings.push(
                SemanticWarning::UnresolvedReference("Missing key in `get` statement".into()),
            ),
            "remember" if node.value.is_empty() => warnings.push(
                SemanticWarning::UnresolvedReference("`remember` used with no target".into()),
            ),
            "respond" if node.value.is_empty() => warnings.push(
                SemanticWarning::MissingRespond,
            ),
            "if" | "while" => {
                if node.children.is_empty() {
                    warnings.push(SemanticWarning::InvalidFlow(format!(
                        "{} block has no body",
                        node.node_type
                    )));
                }
            }
            "set" => {
                if node.value.is_empty() {
                    warnings.push(SemanticWarning::InvalidConfiguration(
                        "`set` used with no config target".into(),
                    ));
                } else if node.value == "promote_always" {
                    if let Some(v) = node.modifiers.get(0) {
                        if v != "true" && v != "false" {
                            warnings.push(SemanticWarning::InvalidConfiguration(format!(
                                "`set promote_always` expects 'true' or 'false', got '{}'",
                                v
                            )));
                        }
                    } else {
                        warnings.push(SemanticWarning::InvalidConfiguration(
                            "`set promote_always` used with no value".into(),
                        ));
                    }
                }
            }
            "unknown" => warnings.push(SemanticWarning::UnknownNodeType(node.value.clone())),
            _ => {}
        }

        for child in &node.children {
            walk(child, warnings);
        }
    }

    for node in ast {
        walk(node, &mut warnings);
    }

    warnings
}
