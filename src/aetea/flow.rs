//! # Flow Language
//!
//! A lightweight parser that turns a string like `{ +1_2, -i_3 }` into an AST of tokens.

#[derive(Debug)]
pub struct FlowExpr {
    /// Tokens parsed from the Flow expression.
    pub tokens: Vec<String>,
}

impl FlowExpr {
    /// Parse a Flow Language string into a `FlowExpr`.
    ///
    /// * Must start with `{` and end with `}`.
    /// * Tokens are comma‑separated; each token may be a signed
    ///   base identifier (e.g. `+1_2`, `-i_3`).
    pub fn parse(input: &str) -> Result<Self, String> {
        let trimmed = input.trim();
        if !trimmed.starts_with('{') || !trimmed.ends_with('}') {
            return Err("Flow expressions must be wrapped in braces '{}'".into());
        }
        // Strip the outer braces
        let inner = &trimmed[1..trimmed.len() - 1];
        let tokens: Vec<String> = inner
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        Ok(FlowExpr { tokens })
    }
}