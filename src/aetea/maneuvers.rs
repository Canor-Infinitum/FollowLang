//! # Maneuvers Language
//!
//! Represents a meta‑group of Flow/Action fragments.  Here we just keep the text.

#[derive(Debug)]
pub struct ManeuverExpr {
    pub raw: String,
}

impl ManeuverExpr {
    pub fn parse(input: &str) -> Result<Self, String> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err("Maneuver expression cannot be empty".into());
        }
        Ok(ManeuverExpr { raw: trimmed.to_string() })
    }
}