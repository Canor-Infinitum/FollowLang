// follow-project/src/fwp.rs

#[        // R#[derive(Debug, Clone, PartialEq)]
    pub ricci_scalar_scaled: String, // R*
    pub stress_energy: String,       // T_(αβ)
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectConstants {
    pub kappa: String,   // κ
    pub lambda: String,  // Λ
    pub dimension: String, // n
}

impl FollowProjectSpec {
    pub fn qgct() -> Self {
        Self {
            name: "QGCT-Schwarzer-Follow".into(),
            tensors: ProjectTensors {
                metric: "g_(αβ)".into(),
                ricci_tensor: "R_(αβ)".into(),
                ricci_scalar: "R".into(),
                ricci_scalar_scaled: "R*".into(),
                stress_energy: "T_(αβ)".into(),
            },
            constants: ProjectConstants {
                kappa: "κ".into(),
                lambda: "Λ".into(),
                dimension: "n".into(),
            },
        }
    }
}

pub struct FollowProjectSpec {
    pub name: String,
    pub tensors: ProjectTensors,
    pub constants: ProjectConstants,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectTensors {
    pub metric: String,              // g_(αβ)
    pub ricci_tensor: String,        // R_(αβ)
