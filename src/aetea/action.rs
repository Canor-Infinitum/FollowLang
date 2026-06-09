//! # Action Language
//!
//! Stores an action expression as raw text.  A real implementation would
//! build an AST and evaluate modular constraints.

#[derive(Debug)]
pub struct ActionExpr {
    /// Raw textual representation.
    pub raw: String,
}

impl ActionExpr {
    /// Parse a string into `ActionExpr`.
    pub fn parse(input: &str) -> Result<Self, String> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err("Action expression cannot be empty".into());
        }
        Ok(ActionExpr { raw: trimmed.to_string() })
    }
}