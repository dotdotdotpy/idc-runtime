use crate::parser::tokenizer::Token;
use serde::Serialize;

/// Represents a node in the abstract syntax tree (AST)
#[derive(Debug, Serialize, Clone)]
pub struct AstNode {
    pub node_type: String,
    pub value: String,
    pub modifiers: Vec<String>,
    pub children: Vec<AstNode>,
}

/// Errors that can occur during AST construction
#[derive(Debug)]
pub enum AstBuildError {
    EmptyTokenList,
    UnexpectedIndentation,
    TokenParseError(String),
}

/// Build the AST from a token stream based on indentation hierarchy
pub fn build_ast(tokens: &[Token]) -> Result<Vec<AstNode>, AstBuildError> {
    if tokens.is_empty() {
        return Err(AstBuildError::EmptyTokenList);
    }

    fn build_nested(
        tokens: &[Token],
        start_index: usize,
        parent_indent: usize,
    ) -> Result<(Vec<AstNode>, usize), AstBuildError> {
        let mut nodes = Vec::new();
        let mut index = start_index;

        while index < tokens.len() {
            let token = &tokens[index];

            if token.indentation < parent_indent {
                break;
            }

            let current_indent = token.indentation;
            let mut node = AstNode {
                node_type: token.token_type.clone(),
                value: token.value.clone(),
                modifiers: token.modifiers.clone(),
                children: Vec::new(),
            };

            if index + 1 < tokens.len() && tokens[index + 1].indentation > current_indent {
                let (child_nodes, consumed) =
                    build_nested(tokens, index + 1, tokens[index + 1].indentation)?;
                node.children = child_nodes;
                index = consumed;
            } else {
                index += 1;
            }

            nodes.push(node);
        }

        Ok((nodes, index))
    }

    let (ast, _) = build_nested(tokens, 0, 0)?;
    Ok(ast)
}
