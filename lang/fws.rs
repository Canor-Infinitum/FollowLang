// follow-source/src/fws.rs

#[derive(Debug, Clone, PartialEq)]
pub struct FollowSourceSpec {
    pub name: String,
    pub equation: String,
    pub tensors: Vec<String>,
    pub functions: Vec<String>,
}

impl FollowSourceSpec {
    pub fn qgct_equation() -> Self {
        Self {
            name: "EinsteinLikeScaledRicci_QGCT".into(),
            equation: r#"
0 = (
  R_(αβ) - (1/2) R* g_(αβ)
  + ((F'/F) * (1-n)) * ( ∇_α∇_β f - (1/2) g_(αβ) Δf )
  + ((1-n)/(4 F^2)) * (4 F F'' + (F')^2 (n-6))
    * ( ∇_α f ∇_β f - (1/2) g_(αβ) ∇_μ f ∇^μ f )
  + κ T_(αβ)
  + Λ F g_(αβ)
)
"#.into(),
            tensors: vec![
                "R_(αβ)".into(),
                "g_(αβ)".into(),
                "T_(αβ)".into(),
            ],
            functions: vec![
                "F(f)".into(),
                "F'".into(),
                "F''".into(),
                "Δf".into(),
            ],
        }
    }
}