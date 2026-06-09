//! # Elite Language
//!
//! Assembly‑style procedural or symbolic instructions.

#[derive(Debug)]
pub struct EliteExpr {
    pub raw: String,
}

impl EliteExpr {
    pub fn parse(input: &str) -> Result<Self, String> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err("Elite expression cannot be empty".into());
        }
        Ok(EliteExpr { raw: trimmed.to_string() })
    }
}