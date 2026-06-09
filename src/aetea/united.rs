//! # United Interchange Language (UI)
//!
//! A placeholder for schemas that inter‑connect Flow/Action/Maneuver blocks.

#[derive(Debug)]
pub struct UIExpr {
    pub raw: String,
}

impl UIExpr {
    pub fn parse(input: &str) -> Result<Self, String> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err("UI expression cannot be empty".into());
        }
        Ok(UIExpr { raw: trimmed.to_string() })
    }
}